use log::{debug, error, warn};

mod config;
mod errors;
mod models;
mod server;
mod services;

fn main() {
    env_logger::init();
    debug!("main thread: {:?}", std::thread::current());

    let config = config::AppConfig::from_env();
    let server = server::Server::new(config);

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(server.run()) {
        Ok(_) => warn!("server terminated"),
        Err(err) => error!("fatal error: {:?}", &err),
    };
}
