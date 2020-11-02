use rumqttc::{MqttOptions, AsyncClient, SubscribeTopic, QoS, Event, Incoming, Publish};
use tonic::{Request, Response, Status};
use crate::config::{Config};
use std::error::Error;
use tokio::sync::RwLock;
use tracing::{info};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use futures::TryFutureExt;

static METRIC_TOPICS: &'static [&'static str] = &["$SYS/broker/version"];

mod broker_metrics {
    tonic::include_proto!("mqtt.broker_metrics");
}

pub use broker_metrics::{VersionResponse, VersionRequest, broker_metrics_server::{BrokerMetrics, BrokerMetricsServer}};


#[derive(Default, Debug)]
struct MqttMetrics {
    version: Option<String>,
}

pub struct MqttBrokerMetricsService {
    metrics: Arc<RwLock<MqttMetrics>>,
    handle: JoinHandle<Result<(), ()>>,
}

impl MqttBrokerMetricsService {
    pub fn new(config: Config) -> Self {
        info!("Building MqttBrokerMetricsService");
        let metrics: Arc<RwLock<MqttMetrics>> = Default::default();
        let handle = {
            let metrics = metrics.clone();
            tokio::spawn(async move {
                spawn_broker_poller(config, metrics.clone()).map_err(|_| ()).await
            })
        };

        MqttBrokerMetricsService {
            metrics, handle
        }

    }
}

#[tonic::async_trait]
impl broker_metrics::broker_metrics_server::BrokerMetrics for MqttBrokerMetricsService {
    async fn version(&self, _request: Request<broker_metrics::VersionRequest>) -> Result<Response<broker_metrics::VersionResponse>, Status> {
        let m = self.metrics.read().await;
        if let Some(ref version) = m.version {
            Ok(Response::new(broker_metrics::VersionResponse {
                version: version.to_owned()
            }))
        } else {
            Err(Status::unavailable("Not connected to MQTT broker"))
        }
    }
}

async fn spawn_broker_poller(config: Config, metrics: Arc<RwLock<MqttMetrics>>) -> Result<(), Box<dyn Error>> {
    info!("Starting mqtt metrics poller");
    let mut mqttoptions = MqttOptions::new("broker_metrics", config.mqtt.host.ip().to_string(), config.mqtt.host.port());
    if let Some(ref creds) = config.mqtt.authentication {
        mqttoptions.set_credentials(&creds.username, &creds.password);
    }
    mqttoptions.set_keep_alive(5);

    let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let topics = METRIC_TOPICS.iter()
        .map(|topic| SubscribeTopic::new(topic.to_string(), QoS::AtLeastOnce));

    client.subscribe_many(topics).await?;

    let metrics = metrics.clone();
    loop {
        let notification = eventloop.poll().await?;
        match notification {
            Event::Incoming(Incoming::Publish(Publish {topic, payload, ..}))  if topic.eq_ignore_ascii_case("$SYS/broker/version") => {
                let version = String::from_utf8_lossy(payload.as_ref()).to_string();
                info!("Connected to {}", version);
                let mut m = metrics.write().await;
                m.version.replace(version);
            },
            _ => {}
        };
        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}