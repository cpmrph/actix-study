use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};

#[derive(Clone)]
struct AppState {
    map: Arc<Mutex<HashMap<String, i32>>>,
}

#[derive(Debug, Deserialize, Clone)]
struct Application {
    user_name: String,
    sleep_duration_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
struct AppConfig {
    application: Application,
}

impl AppConfig {
    fn load() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(File::with_name("config/config.toml"))
            .build()?;

        settings.try_deserialize()
    }

    fn sleep_duration(&self) -> Duration {
        Duration::from_secs(self.application.sleep_duration_secs)
    }
}

async fn index(data: web::Data<AppState>, config: web::Data<AppConfig>) -> impl Responder {
    let mut map = data.map.lock().unwrap();
    let counter = map.entry("counter".to_string()).or_insert(0);
    *counter += 1;
    HttpResponse::Ok().body(format!(
        "User name: {}, Counter: {}",
        config.application.user_name, counter
    ))
}

async fn monitor_state(state: Arc<Mutex<HashMap<String, i32>>>, config: AppConfig) {
    loop {
        {
            let map = state.lock().unwrap();
            if let Some(counter) = map.get("counter") {
                println!("Current Counter: {}", counter);
            } else {
                println!("Counter not found");
            }
        }
        sleep(config.sleep_duration()).await;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = AppConfig::load().expect("Failed to load configuration");

    let state = AppState {
        map: Arc::new(Mutex::new(HashMap::new())),
    };

    // Clone the state and pass configuration to the monitor_state
    let state_clone = state.map.clone();
    tokio::spawn(monitor_state(state_clone, config.clone()));

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(config.clone())) // Pass config
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
