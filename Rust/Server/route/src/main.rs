use actix_web::{rt::System, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use std::sync::mpsc;
use std::thread;

// struct AppStateWithCounter {
//     counter: Mutex<i32>,
// }

// async fn index(data: web::Data<AppStateWithCounter>) -> String {
//     let mut counter = data.counter.lock().unwrap();
//     *counter += 1;
//     format!("请求次数: {}", counter)
// }
#[derive(Serialize)]
struct CustomizeResponder {
    name: &'static str,
}

impl Responder for CustomizeResponder {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn index() -> impl Responder {
    CustomizeResponder { name: "zheng" }
}

#[actix_web::main]
async fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let sys = System::new("http-server");
        let srv = HttpServer::new(|| App::new().route("/", web::get().to(index)))
            .bind("127.0.0.1:8080")?
            .shutdown_timeout(60) // <- Set shutdown timeout to 60 seconds
            .run();

        let _ = tx.send(srv);
        sys.run()
    });

    let srv = rx.recv().unwrap();

    // 暂停接受传入的连接
    srv.pause().await;
    // 重新开始接受传入的连接
    srv.resume().await;
    // 停止服务器
    // srv.stop(true).await;
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(
//                 web::scope("/")
//                     .guard(guard::Header("Host", "www.rust-lang.org"))
//                     .route("", web::to(|| HttpResponse::Ok().body("www"))),
//             )
//             .service(
//                 web::scope("/")
//                     .guard(guard::Header("Host", "users.rust-lang.org"))
//                     .route("", web::to(|| HttpResponse::Ok().body("user"))),
//             )
//             .route("/", web::to(|| HttpResponse::Ok()))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

// /** 共享数据data */
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0),
//     });
//     print!("123");
//     HttpServer::new(move || {
//         App::new().service(
//             web::scope("/app")
//                 .data(counter.clone())
//                 .route("/index", web::get().to(index)),
//         )
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
