#[macro_use]
extern crate dotenv_codegen;

use dotenv::dotenv;
use hello_actix::HelloActix;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = dotenv!("PORT").parse().unwrap();

    let app = HelloActix::new(port);
    app.run().await
}
