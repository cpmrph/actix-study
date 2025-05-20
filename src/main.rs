use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::sync::{Arc, Mutex};
use tokio::time::{Duration, sleep};

#[derive(Clone)]
struct AppState {
    counter: Arc<Mutex<i32>>,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Counter: {}", counter))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        counter: Arc::new(Mutex::new(0)),
    };

    // クローンされた状態で監視タスクを生成
    let state_clone = state.clone();
    tokio::spawn(async move {
        loop {
            {
                let counter = state_clone.counter.lock().unwrap();
                println!("Current Counter: {}", counter);
            }
            sleep(Duration::from_secs(5)).await; // 5秒ごとに状態を監視
        }
    });

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
