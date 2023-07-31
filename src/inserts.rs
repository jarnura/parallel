#[cfg(feature = "diesel")]
use async_bb8_diesel::AsyncRunQueryDsl;
use time::{OffsetDateTime, PrimitiveDateTime};

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use crate::{models, pooling};

// pub struct Inserts<T: pooling::DatabasePooling>{
//     store: pooling::Store<T>
// }

pub struct Inserts;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
impl Inserts {
    fn make_pi() -> models::PaymentIntentNew {
        let created_at @ modified_at = Some(current_time());
        models::PaymentIntentNew {
            created_at,
            modified_at,
            ..Default::default()
        }
    }
}

#[cfg(feature = "diesel")]
impl Inserts {
    pub async fn insert_pi(store: &pooling::DieselAsync) -> models::PaymentIntent {
        use crate::schema::payment_intent::dsl::*;
        let conn = pooling::get_connection(store).await;
        let pi = Self::make_pi();
        println!("{pi:#?}");

        diesel::insert_into(payment_intent)
            .values(pi)
            .get_result_async(&*conn)
            .await
            .expect("Unable to insert")
    }

    pub async fn insert_pi_with_instrument(store: &pooling::DieselAsync, i_x: i8) -> models::PaymentIntent {
        println!("pi entering {}", i_x);
        let start_time = current_time();
        // let pi = Self::insert_pi(self.store.master_pool.into()).await;
        let pi = Self::insert_pi(store).await;
        let end_time = current_time();
        println!("{}", end_time - start_time);
        println!("pi exit {}", i_x);
        pi
    }
}

#[cfg(feature = "sqlx")]
// impl Inserts<pooling::SqlxAsync> {
impl Inserts {
    pub async fn insert_pi(store: &pooling::SqlxAsync) -> models::PaymentIntent {
        // use crate::schema::payment_intent::dsl::*;
        let conn = pooling::get_connection(store).await;
        let pi = Self::make_pi();
        println!("{pi:#?}");
        sqlx::query_as!(
            models::PaymentIntent,
            r#"
    INSERT INTO payment_intent ( payment_id, merchant_id, status, amount, created_at, modified_at )
    VALUES ( $1, $2, $3, $4, $5, $6 )
    RETURNING *
            "#,
            pi.payment_id,
            pi.merchant_id,
            pi.status,
            pi.amount,
            pi.created_at,
            pi.modified_at,
        )
        .fetch_one(conn)
        .await
        .expect("Unable to insert")
    }

    pub async fn insert_pi_with_instrument(store: &pooling::SqlxAsync, i_x: i8) -> models::PaymentIntent {
        println!("pi entering {}", i_x);
        let start_time = current_time();
        // let pi = Self::insert_pi(self.store.master_pool.into()).await;
        let pi = Self::insert_pi(store).await;
        let end_time = current_time();
        println!("{}", end_time - start_time);
        println!("pi exit {}", i_x);
        pi
    }
}

pub fn current_time() -> PrimitiveDateTime {
    let utc_date_time = OffsetDateTime::now_utc();
    PrimitiveDateTime::new(utc_date_time.date(), utc_date_time.time())
}
