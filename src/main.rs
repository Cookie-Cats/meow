use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
use clap::Parser;
use std::net::{ToSocketAddrs, SocketAddr};


#[derive(Parser)]
#[command(name = "meow")]
#[command(author = "Diazepam. <https://github.com/metaphorme>")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Sweet cats always meow~ back.\nIf you need help, please visit https://github.com/Cookie-Cats/meow")]
struct Cli {
    #[arg(short = 'l', long, value_name = "0.0.0.0:8080")]
    listen: Option<String>,
}

async fn index(req: HttpRequest) -> impl Responder {
    let addr = req.peer_addr().unwrap_or("0.0.0.0:0".parse().unwrap());
    let ip = addr.ip().to_string();
    format!("{}", ip)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化日志服务
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // 获取监听端口
    let cli = Cli::parse();
    let listen_addr: SocketAddr = cli.listen.map_or(
        // Default value if listen is None
        "0.0.0.0:8080".parse().unwrap(),
        // Function to apply if listen is Some
        |s| s.to_socket_addrs().unwrap().next().unwrap(),
    );

    log::info!("Sweet cats always meow~ back.");
    log::info!("Starting HTTP server at http://{}", listen_addr);

    HttpServer::new(|| {
        App::new()
            // 启动日志
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(index))
            .service(web::resource("/").to(index))
    })
    .bind(listen_addr)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test, web, App, Error};
    use super::*;

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?.as_ref(), b"0.0.0.0");

        Ok(())
    }
}
