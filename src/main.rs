#[cfg(any(feature = "diesel", feature = "sqlx"))]
use futures::join;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use std::env;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use dotenvy::dotenv;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use parallel::{inserts::Inserts, pooling, reads::Reads};

#[cfg(feature = "diesel")]
async fn init() -> pooling::DieselAsync {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    pooling::DieselAsync::new(&database_url).await
}

#[cfg(feature = "sqlx")]
async fn init() -> pooling::SqlxAsync {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    pooling::SqlxAsync::new(&database_url).await
}

#[actix_web::main]
async fn main() {
    println!("main entering");
    let start_time = parallel::pooling::current_time();

    #[cfg(any(feature = "diesel", feature = "sqlx"))]
    {
        let store = init().await;
        let pi_1_async = Inserts::insert_pi_with_instrument(&store, 1);
        let pi_2_async = Inserts::insert_pi_with_instrument(&store, 2);
        let pi_3_async = Inserts::insert_pi_with_instrument(&store, 3);
        let pi_4_async = Inserts::insert_pi_with_instrument(&store, 4);
        let pi_5_async = Inserts::insert_pi_with_instrument(&store, 5);
        let pi_6_async = Inserts::insert_pi_with_instrument(&store, 6);
        let (pi_1_async, pi_2_async, pi_3_async, pi_4_async, pi_5_async, pi_6_async) =
            join!(pi_1_async, pi_2_async, pi_3_async, pi_4_async, pi_5_async, pi_6_async);

        let pi_1_async = Reads::read_pi_with_instrument(&store, pi_1_async.payment_id, 1);
        let pi_2_async = Reads::read_pi_with_instrument(&store, pi_2_async.payment_id, 2);
        let pi_3_async = Reads::read_pi_with_instrument(&store, pi_3_async.payment_id, 3);
        let pi_4_async = Reads::read_pi_with_instrument(&store, pi_4_async.payment_id, 4);
        let pi_5_async = Reads::read_pi_with_instrument(&store, pi_5_async.payment_id, 5);
        let pi_6_async = Reads::read_pi_with_instrument(&store, pi_6_async.payment_id, 6);
        let _ = join!(pi_1_async, pi_2_async, pi_3_async, pi_4_async, pi_5_async, pi_6_async);
    }
    let end_time = parallel::pooling::current_time();
    println!("{}", end_time - start_time);
}
