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

async fn watch(data: web::Data<AppState>, _: web::Data<AppConfig>) -> impl Responder {
    for _ in 0..10 {
        let map = data.map.lock().unwrap();
        if map.contains_key("counter") {
            let counter = map.get("counter").unwrap();
            if counter > &2 {
                return HttpResponse::Ok().body(format!("Finished by counter exceeded."));
            }
            sleep(Duration::from_secs(1)).await;
        }
    }

    HttpResponse::Ok().body(format!("Finished. Counter not exceeded."))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = AppConfig::load().expect("Failed to load configuration");

    let state = AppState {
        map: Arc::new(Mutex::new(HashMap::new())),
    };

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(config.clone())) // Pass config
            .route("/", web::get().to(index))
            .route("/watch", web::get().to(watch))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    #[actix_rt::test]
    async fn test_watch_response_when_counter_exceeds() {
        // State setup: Initialize counter to 3 so watch will finish immediately
        let state = Arc::new(Mutex::new(HashMap::new()));
        {
            let mut map = state.lock().unwrap();
            map.insert("counter".to_string(), 3);
        }

        // Mock configuration
        let config = web::Data::new(AppConfig {
            application: Application {
                user_name: "TestUser".to_string(),
                sleep_duration_secs: 1,
            },
        });

        // Create app with the /watch endpoint
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { map: state.clone() }))
                .app_data(config.clone())
                .route("/watch", web::get().to(watch)),
        )
        .await;

        // Send request to /watch
        let req = test::TestRequest::get().uri("/watch").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert that the response is as expected
        assert!(resp.status().is_success());
        let body_bytes = test::read_body(resp).await;
        assert_eq!(
            std::str::from_utf8(&body_bytes).unwrap(),
            "Finished by counter exceeded."
        );
    }

    #[actix_rt::test]
    async fn test_watch_response_when_counter_not_exceeded() {
        // State setup: Initialize counter to 1
        let state = Arc::new(Mutex::new(HashMap::new()));
        {
            let mut map = state.lock().unwrap();
            map.insert("counter".to_string(), 1);
        }

        // Mock configuration
        let config = web::Data::new(AppConfig {
            application: Application {
                user_name: "TestUser".to_string(),
                sleep_duration_secs: 1,
            },
        });

        // Create app with the /watch endpoint
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(AppState { map: state.clone() }))
                .app_data(config.clone())
                .route("/watch", web::get().to(watch)),
        )
        .await;

        // Send request to /watch
        let req = test::TestRequest::get().uri("/watch").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert that the response is as expected
        assert!(resp.status().is_success());
        let body_bytes = test::read_body(resp).await;
        assert_eq!(
            std::str::from_utf8(&body_bytes).unwrap(),
            "Finished. Counter not exceeded."
        );
    }
}
