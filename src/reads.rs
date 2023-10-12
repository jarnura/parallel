#[cfg(any(feature = "diesel", feature = "sqlx"))]
use crate::utils::instrument;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use crate::{models, pooling};

#[cfg(feature = "diesel")]
use async_bb8_diesel::AsyncRunQueryDsl;

#[cfg(feature = "diesel")]
use diesel::prelude::*;
pub struct Reads;

#[cfg(feature = "diesel")]
impl Reads {
    #[tracing::instrument(skip_all)]
    pub async fn read_pi(store: &pooling::DieselAsync, pid: String) -> models::PaymentIntent {
        use crate::schema::payment_intent::dsl::*;
        let conn = pooling::get_connection(store).await;
        let pi = payment_intent
            .filter(payment_id.eq(pid))
            .first_async::<models::PaymentIntent>(&*conn)
            .await
            .expect("Error loading payment_intent");
        drop(conn);
        pi
    }

    #[tracing::instrument(skip_all)]
    pub async fn read_pi_with_instrument(
        store: &pooling::DieselAsync,
        pid: String,
        ix: i8,
    ) -> models::PaymentIntent {
        instrument(|| Self::read_pi(store, pid), ix).await
    }
}

#[cfg(feature = "sqlx")]
impl Reads {
    pub async fn read_pi(store: &pooling::SqlxAsync, pid: String) -> models::PaymentIntent {
        // use crate::schema::payment_intent::dsl::*;
        let conn = pooling::get_connection(store).await;
        let pi = sqlx::query_as!(
            models::PaymentIntent,
            r#"SELECT * FROM payment_intent WHERE payment_id = $1"#,
            pid
        )
        .fetch_one(conn)
        .await
        .expect("Error loading payment_intent");
        pi
    }

    pub async fn read_pi_with_instrument(
        store: &pooling::SqlxAsync,
        pid: String,
        ix: i8,
    ) -> models::PaymentIntent {
        instrument(|| Self::read_pi(store, pid), ix).await
    }
}
