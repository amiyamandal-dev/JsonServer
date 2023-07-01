use fast_log::Config;
use log::{error, info, warn};
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{convert::Infallible, error::Error};
use tokio::net::TcpListener;

use async_process::{Command, Stdio};
use futures_lite::{io::BufReader, prelude::*};
use hyper::server::conn::Http;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::str;
use tower::make::Shared;

async fn log_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path_uri = req.uri();
    info!("{path_uri}");
    router(req).await
   
}


async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {


    handler(req).await

}


async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    info!("at handler");
    let mut child = Command::new("C:\\ProgramData\\mambaforge\\python.exe")
        .args(["hello_world.py", "--name=abcd"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    let mut rez_vec:Vec<String> = Vec::new();

    while let Some(line) = lines.next().await {
        match line{
            Ok(t) => {
                rez_vec.push(t);
            },
            Err(e) => {
                error!("{e}");
            },
        }
    }

    Ok(Response::new(Body::from(rez_vec.join("\n"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    fast_log::init(
        Config::new()
            .level(log::LevelFilter::Info)
            .console()
            .chan_len(Some(100000)),
    )
    .unwrap();

    // let make_service = Shared::new(service_fn(log_request));

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 9777);
    info!("server started");
    let listener = TcpListener::bind(socket).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            let http = Http::new();

            let conn = http.serve_connection(stream, service_fn(log_request));
            if let Err(e) = conn.await {
                error!("server connection error: {}", e);
            }
        });
    }
}
