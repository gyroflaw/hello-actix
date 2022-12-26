use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;

pub struct HelloActix {
    pub port: u16,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
