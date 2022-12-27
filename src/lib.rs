use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use std::sync::Mutex;

pub struct HelloActix {
    pub port: u16,
}

struct HelloActixState {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/")]
async fn hello(data: web::Data<HelloActixState>) -> impl Responder {
    let app_name = &data.app_name;

    let mut counter = data.counter.lock().unwrap();

    *counter += 1;

    HttpResponse::Ok().body(format!("Hello {app_name}!\nCounter: {counter}"))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

impl HelloActix {
    pub fn new(port: u16) -> Self {
        HelloActix { port }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        info!("Starting HTTP server at 127.0.0.1:{}", self.port);

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(HelloActixState {
                    app_name: String::from("My first Actix Server"),
                    counter: Mutex::new(0),
                }))
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .service(hello)
                .service(echo)
                .route("/heys", web::get().to(manual_hello))
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self},
        test,
    };

    #[actix_web::test]
    async fn test_hello_ok() {
        let req = test::TestRequest::default().to_http_request();
        let result = manual_hello().await;
        let resp = result.respond_to(&req);
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
