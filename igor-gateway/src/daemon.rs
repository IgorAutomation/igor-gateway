use crate::helpers::{Error, EitherBody};
use std::convert::Infallible;
use tonic::transport::Server as TonicServer;
use tower::Service;
use warp::Filter;
use futures::future::{self, Either, TryFutureExt};
use hyper::{service::make_service_fn, Server};
use http::version::Version;
use tracing::info;
use crate::grpc::MyGreeter;
use crate::grpc::hello_world::greeter_server::{GreeterServer};
use crate::db::create_pool;
use crate::config::Config;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {


    let mut warp = warp::service(warp::path("hello").map(|| "hello, world!"));
    let builder = Config::builder().unwrap();
    //builder.add_chunk_full(input, Priority::default(), DEFAULT_DUPLICATE_STRATEGY).unwrap();
    let config: Config = builder.build().unwrap();
    let pool = create_pool(&config).await.unwrap();

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

            let mut tonic = TonicServer::builder()
                .add_service(greeter)
                .add_service(health_service)
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
