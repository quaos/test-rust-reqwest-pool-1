use log::{debug, info, error};
use serde_json::json;
use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;
use std::result::Result;
use std::sync::Arc; // , Mutex};
// use tokio::sync::Mutex;
use warp::{Filter, http::StatusCode};

use crate::config::AppConfig;
use crate::errors::AppError;
// use crate::models::search_result::SearchResult;
use crate::services;

pub struct Server {
  config: Arc<AppConfig>,
  http_client: Arc<reqwest::Client>,
}

impl Server {
  pub fn new(config: AppConfig) -> Self {
    debug!("creating new Server with config: {:?}", &config);

    let http_client = Self::create_http_client(&config)
      .expect("failed to create reqwest HTTP Client instance");

    Self {
      config: Arc::new(config),
      http_client: Arc::new(http_client),
    }
  }

  fn create_http_client(config: &AppConfig) -> Result<reqwest::Client, reqwest::Error> {
    reqwest::Client::builder()
      .use_native_tls()
      .connect_timeout(config.client_connect_timeout)
      .connection_verbose(true)
      .gzip(true)
      .build()
  }

  pub async fn run(self) -> Result<(), Box<dyn Error + 'static>> {
    let Self {
      config,
      http_client,
    } = self;
    let full_addr = format!("{}:{}", &config.host, config.port);
    
    debug!("executing: Server.run() in thread: {:?}", std::thread::current());

    let config_owner_filter = warp::any()
      .map(move || config.clone());

    let http_client_owner_filter = warp::any()
      .map(move || http_client.clone());

    let search_handler = warp::path("q")
      .and(config_owner_filter.clone())
      .and(http_client_owner_filter.clone())
      .and(warp::path::param())
      .and_then(move |config, http_client, keyword| async {
        Self::handle_search(config, http_client, keyword).await
      });

    let routes = search_handler
        .recover(Self::handle_rejection);
    
    info!("Starting HTTP Server on {}", &full_addr);

    let socket_addr = full_addr.parse::<SocketAddr>()?;
        
    warp::serve(routes)
      .run(socket_addr)
      .await;

    Ok(())
  }

  async fn handle_search(
    config: Arc<AppConfig>,
    http_client: Arc<reqwest::Client>,
    keyword: String,
  ) -> Result<impl warp::Reply, warp::Rejection> {
    debug!("executing: handle_search() in thread: {:?}", std::thread::current());

    let google_search_client = services::google_search::Client::new(
      &config.google_search_api_engine_id,
      &config.google_search_api_key,
      http_client
    );

    let result = match google_search_client.search(&keyword).await {
      Ok(data) => data,
      Err(err) => return Err(warp::reject::custom(err)),
    };

    Ok(warp::reply::json(&result))
  }

  async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    debug!("executing: handle_rejection() in thread: {:?}", std::thread::current());
    
    let reply = if err.is_not_found() {
      warp::reply::with_status(
        warp::reply::json(&json!({
          "message": "Not Found",
        })),
        StatusCode::NOT_FOUND
      )
    } else if let Some(app_err) = err.find::<AppError>() {
      let status_code = match &app_err {
        AppError::HttpClientError(_) => StatusCode::BAD_GATEWAY,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
      };
      warp::reply::with_status(
        warp::reply::json(&json!({
          "message": format!("{:?}", &app_err),
        })),
        status_code
      )
    } else {
      error!("unhandled rejection: {:?}", &err);
      warp::reply::with_status(
        warp::reply::json(&json!({
          "message": format!("{:?}", &err),
        })),
        StatusCode::INTERNAL_SERVER_ERROR
      )
    };

    Ok(reply)
  }
}

impl warp::reject::Reject for AppError {}
