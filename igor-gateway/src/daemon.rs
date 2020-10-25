use crate::helpers::{Error, EitherBody};
use std::convert::Infallible;
use tonic::transport::Server as TonicServer;
use tower::Service;
use warp::Filter;
use futures::future::{self, Either, TryFutureExt};
use hyper::{service::make_service_fn, Server};
use http::version::Version;
use crate::grpc::MyGreeter;
use crate::grpc::hello_world::greeter_server::{GreeterServer};

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:1337".parse().unwrap();

    println!("Listening on {}", addr);

    let mut warp = warp::service(warp::path("hello").map(|| "hello, world!"));

    Server::bind(&addr)
        .serve(make_service_fn(move |_| {
            let greeter = GreeterServer::new(MyGreeter::default());
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
