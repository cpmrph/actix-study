use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};

#[derive(Clone)]
struct AppState {
    map: Arc<Mutex<HashMap<String, i32>>>,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut map = data.map.lock().unwrap();
    let counter = map.entry("counter".to_string()).or_insert(0);
    *counter += 1;
    HttpResponse::Ok().body(format!("Counter: {}", counter))
}

async fn monitor_state(state: Arc<Mutex<HashMap<String, i32>>>) {
    loop {
        {
            let map = state.lock().unwrap();
            if let Some(counter) = map.get("counter") {
                println!("Current Counter: {}", counter);
            } else {
                println!("Counter not found");
            }
        }
        sleep(Duration::from_secs(5)).await; // 5秒ごとに状態を監視
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        map: Arc::new(Mutex::new(HashMap::new())),
    };

    // クローンされた状態で監視タスクを生成
    let state_clone = state.map.clone();
    tokio::spawn(monitor_state(state_clone));

    // HTTPサーバの起動
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
