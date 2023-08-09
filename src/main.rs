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
        })
        .bind(("0.0.0.0", 8082))?
        .run()
        .await
    };
    Ok(())
}

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
    let (pi_1_async, pi_2_async, pi_3_async, pi_4_async, pi_5_async, pi_6_async) =
        join!(pi_1_async, pi_2_async, pi_3_async, pi_4_async, pi_5_async, pi_6_async);

    let pi_1_async = Reads::read_pi_with_instrument(store, pi_1_async.payment_id, 1);
    let pi_2_async = Reads::read_pi_with_instrument(store, pi_2_async.payment_id, 2);
    let pi_3_async = Reads::read_pi_with_instrument(store, pi_3_async.payment_id, 3);
    let pi_4_async = Reads::read_pi_with_instrument(store, pi_4_async.payment_id, 4);
    let pi_5_async = Reads::read_pi_with_instrument(store, pi_5_async.payment_id, 5);
    let pi_6_async = Reads::read_pi_with_instrument(store, pi_6_async.payment_id, 6);
    let _ = join!(pi_1_async, pi_2_async, pi_3_async, pi_4_async, pi_5_async, pi_6_async);
    let end_time = parallel::utils::current_time();
    let diff = end_time - start_time;
    println!("{}", diff);
    HttpResponse::Ok().body(format!("{diff}"))
}
