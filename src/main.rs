use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Build router mapping "/hello" to the handler function
    let app = Router::new().route("/hello", get(hello_handler));

    // 2. Bind TCP listener to port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("\n\n Server running on http://127.0.0.1:3000/hello");

    // 3. Start the server
    axum::serve(listener, app).await?;

    Ok(())
}

// Separate handler function
async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello World</h1>")
}
