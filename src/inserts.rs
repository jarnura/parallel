#[cfg(feature = "diesel")]
use async_bb8_diesel::AsyncRunQueryDsl;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use crate::utils;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use crate::utils::instrument;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use crate::{models, pooling};

// pub struct Inserts<T: pooling::DatabasePooling>{
//     store: pooling::Store<T>
// }

pub struct Inserts;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
impl Inserts {
    #[tracing::instrument(skip_all)]
    fn make_pi() -> models::PaymentIntentNew {
        let created_at @ modified_at = Some(utils::current_time());
        models::PaymentIntentNew {
            created_at,
            modified_at,
            ..Default::default()
        }
    }
}

#[cfg(feature = "diesel")]
impl Inserts {
    #[tracing::instrument(skip_all)]
    pub async fn insert_pi(store: &pooling::DieselAsync) -> models::PaymentIntent {
        use crate::schema::payment_intent::dsl::*;
        let conn = pooling::get_connection(store).await;
        let pi = Self::make_pi();
        // println!("{pi:#?}");

        let pi = diesel::insert_into(payment_intent)
            .values(pi)
            .get_result_async(&*conn)
            .await
            .expect("Unable to insert");

        drop(conn);

        pi
    }

    #[tracing::instrument(skip_all)]
    pub async fn insert_pi_with_instrument(
        store: &pooling::DieselAsync,
        ix: i8,
    ) -> models::PaymentIntent {
        instrument(|| Self::insert_pi(store), ix).await
    }
}

#[cfg(feature = "sqlx")]
// impl Inserts<pooling::SqlxAsync> {
impl Inserts {
    #[tracing::instrument]
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

    #[tracing::instrument]
    pub async fn insert_pi_with_instrument(
        store: &pooling::SqlxAsync,
        ix: i8,
    ) -> models::PaymentIntent {
        instrument(|| Self::insert_pi(store), ix).await
    }
}
