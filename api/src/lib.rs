mod blog;
mod routes;

use axum::Router;

#[tokio::main]
pub async fn start() {
    // 获取PORT值
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    // build our application with a single route
    let app = Router::new().merge(routes::compose()); // Router::new().route("/", get(|| async { "Hello, World!" }));

    println!("{app:#?}");
    
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!(
        "Server running on http://{}",
        str::replace(
            listener.local_addr().unwrap().to_string().as_str(),
            "0.0.0.0",
            "127.0.0.1"
        )
    );
    axum::serve(listener, app).await.unwrap();
}
