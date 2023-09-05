#[cfg(feature = "actix")]
#[cfg(any(feature = "diesel", feature = "sqlx"))]
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use futures::join;

use std::env;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use dotenvy::dotenv;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use parallel::{inserts::Inserts, pooling, reads::Reads};

#[cfg(feature = "diesel")]
async fn init(database_url: &str) -> pooling::DieselAsync {
    dotenv().ok();

    pooling::DieselAsync::new(database_url).await
}

#[cfg(feature = "sqlx")]
async fn init(database_url: &str) -> pooling::SqlxAsync {
    dotenv().ok();

    pooling::SqlxAsync::new(database_url).await
}

#[cfg(feature = "axum")]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    #[cfg(any(feature = "diesel", feature = "sqlx"))]
    let store = init(&database_url).await;
    let app = axum::Router::new()
        .route("/", axum::routing::get(executor))
        .route("/sequential", axum::routing::get(executor_sequential))
        .with_state(store);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[cfg(feature = "actix")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    // env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("{database_url}");

    #[cfg(any(feature = "diesel", feature = "sqlx"))]
    let _ = {
        let store = init(&database_url).await;
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(store.clone()))
                .route("/", web::get().to(executor))
                .route("/sequential", web::get().to(executor_sequential))
        })
        .bind(("0.0.0.0", 8080))?
        .run()
    };
    Ok(())
}

#[cfg(feature = "actix")]
#[cfg(any(feature = "diesel", feature = "sqlx"))]
async fn executor(
    #[cfg(feature = "sqlx")] store: web::Data<pooling::SqlxAsync>,
    #[cfg(feature = "diesel")] store: web::Data<pooling::DieselAsync>,
) -> impl Responder {
    println!("main entering");

    let start_time = parallel::utils::current_time();
    let store = store.get_ref();

    let pi_1_async = Inserts::insert_pi_with_instrument(store, 1);
    let pi_2_async = Inserts::insert_pi_with_instrument(store, 2);
    let pi_3_async = Inserts::insert_pi_with_instrument(store, 3);
    let pi_4_async = Inserts::insert_pi_with_instrument(store, 4);
    let pi_5_async = Inserts::insert_pi_with_instrument(store, 5);
    let pi_6_async = Inserts::insert_pi_with_instrument(store, 6);
    let pi_7_async = Inserts::insert_pi_with_instrument(store, 7);
    let pi_8_async = Inserts::insert_pi_with_instrument(store, 8);
    let pi_9_async = Inserts::insert_pi_with_instrument(store, 9);
    let pi_10_async = Inserts::insert_pi_with_instrument(store, 10);
    let pi_11_async = Inserts::insert_pi_with_instrument(store, 11);
    let pi_12_async = Inserts::insert_pi_with_instrument(store, 12);

    let (
        pi_1_async,
        pi_2_async,
        pi_3_async,
        pi_4_async,
        pi_5_async,
        pi_6_async,
        pi_7_async,
        pi_8_async,
        pi_9_async,
        pi_10_async,
        pi_11_async,
        pi_12_async,
    ) = join!(
        pi_1_async,
        pi_2_async,
        pi_3_async,
        pi_4_async,
        pi_5_async,
        pi_6_async,
        pi_7_async,
        pi_8_async,
        pi_9_async,
        pi_10_async,
        pi_11_async,
        pi_12_async
    );

    let pi_1_async = Reads::read_pi_with_instrument(store, pi_1_async.payment_id, 1);
    let pi_2_async = Reads::read_pi_with_instrument(store, pi_2_async.payment_id, 2);
    let pi_3_async = Reads::read_pi_with_instrument(store, pi_3_async.payment_id, 3);
    let pi_4_async = Reads::read_pi_with_instrument(store, pi_4_async.payment_id, 4);
    let pi_5_async = Reads::read_pi_with_instrument(store, pi_5_async.payment_id, 5);
    let pi_6_async = Reads::read_pi_with_instrument(store, pi_6_async.payment_id, 6);
    let pi_7_async = Reads::read_pi_with_instrument(store, pi_7_async.payment_id, 7);
    let pi_8_async = Reads::read_pi_with_instrument(store, pi_8_async.payment_id, 8);
    let pi_9_async = Reads::read_pi_with_instrument(store, pi_9_async.payment_id, 9);
    let pi_10_async = Reads::read_pi_with_instrument(store, pi_10_async.payment_id, 10);
    let pi_11_async = Reads::read_pi_with_instrument(store, pi_11_async.payment_id, 11);
    let pi_12_async = Reads::read_pi_with_instrument(store, pi_12_async.payment_id, 12);

    let _ = join!(
        pi_1_async,
        pi_2_async,
        pi_3_async,
        pi_4_async,
        pi_5_async,
        pi_6_async,
        pi_7_async,
        pi_8_async,
        pi_9_async,
        pi_10_async,
        pi_11_async,
        pi_12_async
    );
    let end_time = parallel::utils::current_time();
    let diff = end_time - start_time;
    HttpResponse::Ok().body(format!("{diff}"))
}

#[cfg(feature = "actix")]
async fn executor_sequential(
    #[cfg(feature = "sqlx")] store: web::Data<pooling::SqlxAsync>,
    #[cfg(feature = "diesel")] store: web::Data<pooling::DieselAsync>,
) -> impl Responder {
    println!("main entering");

    let start_time = parallel::utils::current_time();
    let store = &store;

    let pi_1_async = Inserts::insert_pi_with_instrument(store, 1).await;
    let pi_2_async = Inserts::insert_pi_with_instrument(store, 2).await;
    let pi_3_async = Inserts::insert_pi_with_instrument(store, 3).await;
    let pi_4_async = Inserts::insert_pi_with_instrument(store, 4).await;
    let pi_5_async = Inserts::insert_pi_with_instrument(store, 5).await;
    let pi_6_async = Inserts::insert_pi_with_instrument(store, 6).await;
    let pi_7_async = Inserts::insert_pi_with_instrument(store, 7).await;
    let pi_8_async = Inserts::insert_pi_with_instrument(store, 8).await;
    let pi_9_async = Inserts::insert_pi_with_instrument(store, 9).await;
    let pi_10_async = Inserts::insert_pi_with_instrument(store, 10).await;
    let pi_11_async = Inserts::insert_pi_with_instrument(store, 11).await;
    let pi_12_async = Inserts::insert_pi_with_instrument(store, 12).await;

    let pi_1_async = Reads::read_pi_with_instrument(store, pi_1_async.payment_id, 1).await;
    let pi_2_async = Reads::read_pi_with_instrument(store, pi_2_async.payment_id, 2).await;
    let pi_3_async = Reads::read_pi_with_instrument(store, pi_3_async.payment_id, 3).await;
    let pi_4_async = Reads::read_pi_with_instrument(store, pi_4_async.payment_id, 4).await;
    let pi_5_async = Reads::read_pi_with_instrument(store, pi_5_async.payment_id, 5).await;
    let pi_6_async = Reads::read_pi_with_instrument(store, pi_6_async.payment_id, 6).await;
    let pi_7_async = Reads::read_pi_with_instrument(store, pi_7_async.payment_id, 7).await;
    let pi_8_async = Reads::read_pi_with_instrument(store, pi_8_async.payment_id, 8).await;
    let pi_9_async = Reads::read_pi_with_instrument(store, pi_9_async.payment_id, 9).await;
    let pi_10_async = Reads::read_pi_with_instrument(store, pi_10_async.payment_id, 10).await;
    let pi_11_async = Reads::read_pi_with_instrument(store, pi_11_async.payment_id, 11).await;
    let pi_12_async = Reads::read_pi_with_instrument(store, pi_12_async.payment_id, 12).await;

    let end_time = parallel::utils::current_time();
    let diff = end_time - start_time;
    HttpResponse::Ok().body(format!("{diff}"))
}

#[cfg(feature = "axum")]
async fn executor(
    #[cfg(feature = "sqlx")] axum::extract::State(store): axum::extract::State<pooling::SqlxAsync>,
    #[cfg(feature = "diesel")] axum::extract::State(store): axum::extract::State<
        pooling::DieselAsync,
    >,
) -> impl axum::response::IntoResponse {
    use axum::response::IntoResponse;

    println!("main entering");

    let start_time = parallel::utils::current_time();
    let store = &store;

    let pi_1_async = Inserts::insert_pi_with_instrument(store, 1);
    let pi_2_async = Inserts::insert_pi_with_instrument(store, 2);
    let pi_3_async = Inserts::insert_pi_with_instrument(store, 3);
    let pi_4_async = Inserts::insert_pi_with_instrument(store, 4);
    let pi_5_async = Inserts::insert_pi_with_instrument(store, 5);
    let pi_6_async = Inserts::insert_pi_with_instrument(store, 6);
    let pi_7_async = Inserts::insert_pi_with_instrument(store, 7);
    let pi_8_async = Inserts::insert_pi_with_instrument(store, 8);
    let pi_9_async = Inserts::insert_pi_with_instrument(store, 9);
    let pi_10_async = Inserts::insert_pi_with_instrument(store, 10);
    let pi_11_async = Inserts::insert_pi_with_instrument(store, 11);
    let pi_12_async = Inserts::insert_pi_with_instrument(store, 12);

    let (
        pi_1_async,
        pi_2_async,
        pi_3_async,
        pi_4_async,
        pi_5_async,
        pi_6_async,
        pi_7_async,
        pi_8_async,
        pi_9_async,
        pi_10_async,
        pi_11_async,
        pi_12_async,
    ) = join!(
        pi_1_async,
        pi_2_async,
        pi_3_async,
        pi_4_async,
        pi_5_async,
        pi_6_async,
        pi_7_async,
        pi_8_async,
        pi_9_async,
        pi_10_async,
        pi_11_async,
        pi_12_async
    );

    let pi_1_async = Reads::read_pi_with_instrument(store, pi_1_async.payment_id, 1);
    let pi_2_async = Reads::read_pi_with_instrument(store, pi_2_async.payment_id, 2);
    let pi_3_async = Reads::read_pi_with_instrument(store, pi_3_async.payment_id, 3);
    let pi_4_async = Reads::read_pi_with_instrument(store, pi_4_async.payment_id, 4);
    let pi_5_async = Reads::read_pi_with_instrument(store, pi_5_async.payment_id, 5);
    let pi_6_async = Reads::read_pi_with_instrument(store, pi_6_async.payment_id, 6);
    let pi_7_async = Reads::read_pi_with_instrument(store, pi_7_async.payment_id, 7);
    let pi_8_async = Reads::read_pi_with_instrument(store, pi_8_async.payment_id, 8);
    let pi_9_async = Reads::read_pi_with_instrument(store, pi_9_async.payment_id, 9);
    let pi_10_async = Reads::read_pi_with_instrument(store, pi_10_async.payment_id, 10);
    let pi_11_async = Reads::read_pi_with_instrument(store, pi_11_async.payment_id, 11);
    let pi_12_async = Reads::read_pi_with_instrument(store, pi_12_async.payment_id, 12);

    let _ = join!(
        pi_1_async,
        pi_2_async,
        pi_3_async,
        pi_4_async,
        pi_5_async,
        pi_6_async,
        pi_7_async,
        pi_8_async,
        pi_9_async,
        pi_10_async,
        pi_11_async,
        pi_12_async
    );
    let end_time = parallel::utils::current_time();
    let diff = end_time - start_time;
    diff.to_string().into_response()
}

#[cfg(feature = "axum")]
async fn executor_sequential(
    #[cfg(feature = "sqlx")] axum::extract::State(store): axum::extract::State<pooling::SqlxAsync>,
    #[cfg(feature = "diesel")] axum::extract::State(store): axum::extract::State<
        pooling::DieselAsync,
    >,
) -> impl axum::response::IntoResponse {
    use axum::response::IntoResponse;

    println!("main entering");

    let start_time = parallel::utils::current_time();
    let store = &store;

    let pi_1_async = Inserts::insert_pi_with_instrument(store, 1).await;
    let pi_2_async = Inserts::insert_pi_with_instrument(store, 2).await;
    let pi_3_async = Inserts::insert_pi_with_instrument(store, 3).await;
    let pi_4_async = Inserts::insert_pi_with_instrument(store, 4).await;
    let pi_5_async = Inserts::insert_pi_with_instrument(store, 5).await;
    let pi_6_async = Inserts::insert_pi_with_instrument(store, 6).await;
    let pi_7_async = Inserts::insert_pi_with_instrument(store, 7).await;
    let pi_8_async = Inserts::insert_pi_with_instrument(store, 8).await;
    let pi_9_async = Inserts::insert_pi_with_instrument(store, 9).await;
    let pi_10_async = Inserts::insert_pi_with_instrument(store, 10).await;
    let pi_11_async = Inserts::insert_pi_with_instrument(store, 11).await;
    let pi_12_async = Inserts::insert_pi_with_instrument(store, 12).await;

    let pi_1_async = Reads::read_pi_with_instrument(store, pi_1_async.payment_id, 1).await;
    let pi_2_async = Reads::read_pi_with_instrument(store, pi_2_async.payment_id, 2).await;
    let pi_3_async = Reads::read_pi_with_instrument(store, pi_3_async.payment_id, 3).await;
    let pi_4_async = Reads::read_pi_with_instrument(store, pi_4_async.payment_id, 4).await;
    let pi_5_async = Reads::read_pi_with_instrument(store, pi_5_async.payment_id, 5).await;
    let pi_6_async = Reads::read_pi_with_instrument(store, pi_6_async.payment_id, 6).await;
    let pi_7_async = Reads::read_pi_with_instrument(store, pi_7_async.payment_id, 7).await;
    let pi_8_async = Reads::read_pi_with_instrument(store, pi_8_async.payment_id, 8).await;
    let pi_9_async = Reads::read_pi_with_instrument(store, pi_9_async.payment_id, 9).await;
    let pi_10_async = Reads::read_pi_with_instrument(store, pi_10_async.payment_id, 10).await;
    let pi_11_async = Reads::read_pi_with_instrument(store, pi_11_async.payment_id, 11).await;
    let pi_12_async = Reads::read_pi_with_instrument(store, pi_12_async.payment_id, 12).await;

    let end_time = parallel::utils::current_time();
    let diff = end_time - start_time;
    diff.to_string().into_response()
}

// dummy main
#[cfg(not(any(feature = "actix", feature = "axum")))]
fn main() {
    println!("Hello world");
}
