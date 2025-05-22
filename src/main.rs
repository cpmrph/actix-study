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
            let mut map = state.lock().unwrap();
            if map.contains_key("counter") {
                let counter = map.get_mut("counter").unwrap();
                println!("Current Counter: {}", counter);
                if counter > &mut 10 {
                    println!("Counter exceeded 10, resetting to 0");
                    *counter = 0;
                }
            } else {
                map.insert("counter".to_string(), 1);
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

// * State Setup: We initialize a HashMap with a counter set to 9 and wrap it in Arc<Mutex<...>>.
// * Configuration: We create an AppConfig with a short sleep_duration_secs to speed up the test.
// * Communication Channel: An optional one-shot channel is used here to signal when the counter reset logic completes, ensuring the test waits appropriately.
// * Monitoring Task: monitor_state is spawned as an asynchronous task. It will increment and check the counter value.
// * Assertions: After waiting for the monitor to work, the test checks whether the counter was correctly reset to 0.
// This setup ensures that you can test the behavior of the monitor_state function within your constraints. Note that you may need to adjust the sleep durations to ensure the test reflects realistic conditions based on your actual application timing requirements.
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::sync::oneshot;

    #[tokio::test]
    async fn test_monitor_state() {
        // Create a shared state with initial counter value
        let state = Arc::new(Mutex::new(HashMap::new()));
        let mut map = state.lock().unwrap();
        map.insert("counter".to_string(), 9);
        drop(map);

        // Load configuration with a small sleep duration for testing
        let config = AppConfig {
            application: Application {
                user_name: "TestUser".to_string(),
                sleep_duration_secs: 1,
            },
        };

        // Setup a channel to send a signal when the counter is reset
        let (tx, rx) = oneshot::channel();

        // Spawn the monitor_state task
        let state_clone = state.clone();
        tokio::spawn(async move {
            monitor_state(state_clone, config).await;
            let _ = tx.send(());
        });

        // Wait for some time to allow monitor_state to process
        tokio::time::sleep(Duration::from_secs(5)).await;

        // Check the counter value after the monitor has run
        {
            let map = state.lock().unwrap();
            assert_eq!(map.get("counter").unwrap(), &0);
        }

        // Optionally wait for the task to complete if using signals
        let _ = rx.await;
    }
}
