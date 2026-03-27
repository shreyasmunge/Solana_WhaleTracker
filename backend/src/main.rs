use axum::{
    routing::get,
    Router,
    extract::Path,
    response::Json,
};
use serde::Serialize;
use serde_json::json;
use reqwest::Client;
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct WalletResponse {
    balance: f64,
    price: f64,
    value_usd: f64,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/wallet/:address", get(wallet_handler))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .unwrap();

    println!("🚀 Backend running on http://localhost:8000");

    axum::serve(listener, app).await.unwrap();
}

async fn wallet_handler(Path(address): Path<String>) -> Json<WalletResponse> {
    let client = Client::new();

    // Fetch balance
    let balance_res = client
        .post("https://api.mainnet-beta.solana.com")
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBalance",
            "params": [address]
        }))
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let balance_lamports = balance_res["result"]["value"]
        .as_f64()
        .unwrap_or(0.0);

    let balance_sol = balance_lamports / 1_000_000_000.0;

 let price_response = client
    .get("https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd")
    .header("User-Agent", "solana-whale-tracker/1.0")
    .send()
    .await;

let price: f64 = match price_response {
    Ok(resp) => {
        let json = resp.json::<serde_json::Value>().await.unwrap_or_default();
        json["solana"]["usd"].as_f64().unwrap_or(0.0)
    }
    Err(_) => 0.0,
};

    Json(WalletResponse {
        balance: balance_sol,
        price,
        value_usd: balance_sol * price,
    })
}