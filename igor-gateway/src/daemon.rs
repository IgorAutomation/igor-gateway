use crate::config::Config;
use crate::db::create_pool;
use crate::grpc::hello_world::greeter_server::GreeterServer;
use crate::grpc::MyGreeter;
use crate::grpc::mqtt::BrokerMetricsServer;
use crate::helpers::{EitherBody, Error};
use futures::future::{self, Either, TryFutureExt};
use http::version::Version;
use hyper::{service::make_service_fn, Server};
use std::convert::Infallible;
use tonic::transport::Server as TonicServer;
use tower::Service;
use tracing::info;
use warp::Filter;
use crate::grpc::mqtt::MqttBrokerMetricsService;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let mut warp = warp::service(warp::path("hello").map(|| "hello, world!"));
    let builder = Config::builder()?;
    let config: Config = builder.init()?;
    info!("Current configuration:\n{:#?}", config);
    config.ensure_paths()?;

    let pool = create_pool(&config).await?;

    info!("Listening on {}", config.listen);
    Server::bind(&config.listen)
        .serve(make_service_fn(move |_| {
            let greeter = GreeterServer::new(MyGreeter::new(pool.clone()));
            let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
            tokio::spawn(async move {
                health_reporter
                    .set_serving::<GreeterServer<MyGreeter>>()
                    .await;
            });

            let mbm_service = BrokerMetricsServer::new(MqttBrokerMetricsService::new(config.clone()));
            info!("Created MqttBrokerMetrics service");

            let mut tonic = TonicServer::builder()
                .add_service(greeter)
                .add_service(health_service)
                .add_service(mbm_service)
                .into_service();
            future::ok::<_, Infallible>(tower::service_fn(
                move |req: hyper::Request<hyper::Body>| match req.version() {
                    Version::HTTP_11 | Version::HTTP_10 => Either::Left(
                        warp.call(req)
                            .map_ok(|res| res.map(EitherBody::Left))
                            .map_err(Error::from),
                    ),
                    Version::HTTP_2 => Either::Right(
                        tonic
                            .call(req)
                            .map_ok(|res| res.map(EitherBody::Right))
                            .map_err(Error::from),
                    ),
                    _ => unimplemented!(),
                },
            ))
        }))
        .await?;

    Ok(())
}
