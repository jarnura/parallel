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
    let start_time = parallel::inserts::current_time();

    #[cfg(any(feature = "diesel", feature = "sqlx"))]
    {
        // let mut handles = Vec::new();
        // let rt = actix_rt::Runtime::new().unwrap();
        let store = init().await;
        let pi_1_async = Inserts::insert_pi_with_instrument(&store, 1);
        let pi_2_async = Inserts::insert_pi_with_instrument(&store, 2);
        let pi_3_async = Inserts::insert_pi_with_instrument(&store, 3);
        let pi_4_async = Inserts::insert_pi_with_instrument(&store, 4);
        let pi_5_async = Inserts::insert_pi_with_instrument(&store, 5);
        let pi_6_async = Inserts::insert_pi_with_instrument(&store, 6);
        let (pi_1, pi_2, pi_3, pi_4, pi_5, pi_6) = join!(pi_1_async, pi_2_async, pi_3_async, pi_4_async, pi_5_async, pi_6_async);



        // let pi = Reads::read_pi(&store, pi.payment_id).await;
        // println!("{pi_1:#?},{pi_2:#?}");
        // println!("{pi_3:#?},{pi_4:#?}");
        // println!("{pi_5:#?},{pi_6:#?}");
    }
    let end_time = parallel::inserts::current_time();
    println!("{}", end_time - start_time);
}
