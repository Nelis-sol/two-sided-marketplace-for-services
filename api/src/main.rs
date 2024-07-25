use axum::{
    Router,
    routing::{get, post},
    response::{IntoResponse, Response},
    Json,
};

use api::routes::{
    service::*,
    listing::*,
    buyer::*
};


#[tokio::main]
async fn main() {

    // Build the application with routes
    let app = Router::new()
        // Service routes
        .route("/service/create", post(create_service))
        .route("/service/get/:id", get(get_service_one))
        .route("/service/get/all", get(get_service_all))
        .route("/service/update", post(update_service))
        .route("/service/delete", post(delete_service))
        // Listing routes
        .route("/listing/create", post(create_listing))
        .route("/listing/get/:id", get(get_listing_one))
        .route("/listing/get/all", get(get_listing_all))
        .route("/listing/update", post(update_listing))
        .route("/listing/delete", post(delete_listing))
        // Buyer routes
        .route("/buyer/buynow", post(buy_now));

    // Specify port for incoming requests
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    
    // Run the server
    axum::serve(listener, app).await.unwrap();

}
