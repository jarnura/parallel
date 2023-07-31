use crate::schema::payment_intent;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

#[derive(
    Clone, Debug, Eq, PartialEq, Identifiable, Queryable, Serialize, Deserialize, sqlx::FromRow,
)]
#[diesel(table_name = payment_intent)]
pub struct PaymentIntent {
    pub id: i32,
    pub payment_id: String,
    pub merchant_id: String,
    pub status: String,
    pub amount: i32,
    pub currency: Option<String>,
    pub amount_captured: Option<i32>,
    pub customer_id: Option<String>,
    pub description: Option<String>,
    pub return_url: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub connector_id: Option<String>,
    pub shipping_address_id: Option<String>,
    pub billing_address_id: Option<String>,
    pub statement_descriptor_name: Option<String>,
    pub statement_descriptor_suffix: Option<String>,
    #[serde(with = "iso8601")]
    pub created_at: PrimitiveDateTime,
    #[serde(with = "iso8601")]
    pub modified_at: PrimitiveDateTime,
    #[serde(default, with = "iso8601::option")]
    pub last_synced: Option<PrimitiveDateTime>,
    pub setup_future_usage: Option<String>,
    pub off_session: Option<bool>,
    pub client_secret: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = payment_intent)]
pub struct PaymentIntentNew {
    pub payment_id: String,
    pub merchant_id: String,
    pub status: String,
    pub amount: i32,
    pub currency: Option<String>,
    pub amount_captured: Option<i32>,
    pub customer_id: Option<String>,
    pub description: Option<String>,
    pub return_url: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub connector_id: Option<String>,
    pub shipping_address_id: Option<String>,
    pub billing_address_id: Option<String>,
    pub statement_descriptor_name: Option<String>,
    pub statement_descriptor_suffix: Option<String>,
    #[serde(default, with = "iso8601::option")]
    pub created_at: Option<PrimitiveDateTime>,
    #[serde(default, with = "iso8601::option")]
    pub modified_at: Option<PrimitiveDateTime>,
    #[serde(default, with = "iso8601::option")]
    pub last_synced: Option<PrimitiveDateTime>,
    pub setup_future_usage: Option<String>,
    pub off_session: Option<bool>,
    pub client_secret: Option<String>,
}

/// Use the well-known ISO 8601 format when serializing and deserializing an
/// [`PrimitiveDateTime`][PrimitiveDateTime].
///
/// [PrimitiveDateTime]: ::time::PrimitiveDateTime
pub mod iso8601 {
    use std::num::NonZeroU8;

    use serde::{ser::Error as _, Deserializer, Serialize, Serializer};
    use time::{
        format_description::well_known::{
            iso8601::{Config, EncodedConfig, TimePrecision},
            Iso8601,
        },
        serde::iso8601,
        PrimitiveDateTime, UtcOffset,
    };

    const FORMAT_CONFIG: EncodedConfig = Config::DEFAULT
        .set_time_precision(TimePrecision::Second {
            decimal_digits: NonZeroU8::new(3),
        })
        .encode();

    /// Serialize a [`PrimitiveDateTime`] using the well-known ISO 8601 format.
    pub fn serialize<S>(date_time: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        date_time
            .assume_utc()
            .format(&Iso8601::<FORMAT_CONFIG>)
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }

    /// Deserialize an [`PrimitiveDateTime`] from its ISO 8601 representation.
    pub fn deserialize<'a, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
    where
        D: Deserializer<'a>,
    {
        iso8601::deserialize(deserializer).map(|offset_date_time| {
            let utc_date_time = offset_date_time.to_offset(UtcOffset::UTC);
            PrimitiveDateTime::new(utc_date_time.date(), utc_date_time.time())
        })
    }

    /// Use the well-known ISO 8601 format when serializing and deserializing an
    /// [`Option<PrimitiveDateTime>`][PrimitiveDateTime].
    ///
    /// [PrimitiveDateTime]: ::time::PrimitiveDateTime
    pub mod option {
        use serde::Serialize;
        use time::format_description::well_known::Iso8601;

        use super::*;

        /// Serialize an [`Option<PrimitiveDateTime>`] using the well-known ISO 8601 format.
        pub fn serialize<S>(
            date_time: &Option<PrimitiveDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            date_time
                .map(|date_time| date_time.assume_utc().format(&Iso8601::<FORMAT_CONFIG>))
                .transpose()
                .map_err(S::Error::custom)?
                .serialize(serializer)
        }

        /// Deserialize an [`Option<PrimitiveDateTime>`] from its ISO 8601 representation.
        pub fn deserialize<'a, D>(deserializer: D) -> Result<Option<PrimitiveDateTime>, D::Error>
        where
            D: Deserializer<'a>,
        {
            iso8601::option::deserialize(deserializer).map(|option_offset_date_time| {
                option_offset_date_time.map(|offset_date_time| {
                    let utc_date_time = offset_date_time.to_offset(UtcOffset::UTC);
                    PrimitiveDateTime::new(utc_date_time.date(), utc_date_time.time())
                })
            })
        }
    }
}

/// Use the UNIX timestamp when serializing and deserializing an
/// [`PrimitiveDateTime`][PrimitiveDateTime].
///
/// [PrimitiveDateTime]: ::time::PrimitiveDateTime
pub mod timestamp {

    use serde::{Deserializer, Serialize, Serializer};
    use time::{serde::timestamp, PrimitiveDateTime, UtcOffset};

    /// Serialize a [`PrimitiveDateTime`] using UNIX timestamp.
    pub fn serialize<S>(date_time: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        date_time
            .assume_utc()
            .unix_timestamp()
            .serialize(serializer)
    }

    /// Deserialize an [`PrimitiveDateTime`] from UNIX timestamp.
    pub fn deserialize<'a, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
    where
        D: Deserializer<'a>,
    {
        timestamp::deserialize(deserializer).map(|offset_date_time| {
            let utc_date_time = offset_date_time.to_offset(UtcOffset::UTC);
            PrimitiveDateTime::new(utc_date_time.date(), utc_date_time.time())
        })
    }

    /// Use the UNIX timestamp when serializing and deserializing an
    /// [`Option<PrimitiveDateTime>`][PrimitiveDateTime].
    ///
    /// [PrimitiveDateTime]: ::time::PrimitiveDateTime
    pub mod option {
        use serde::Serialize;

        use super::*;

        /// Serialize an [`Option<PrimitiveDateTime>`] from UNIX timestamp.
        pub fn serialize<S>(
            date_time: &Option<PrimitiveDateTime>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            date_time
                .map(|date_time| date_time.assume_utc().unix_timestamp())
                .serialize(serializer)
        }

        /// Deserialize an [`Option<PrimitiveDateTime>`] from UNIX timestamp.
        pub fn deserialize<'a, D>(deserializer: D) -> Result<Option<PrimitiveDateTime>, D::Error>
        where
            D: Deserializer<'a>,
        {
            timestamp::option::deserialize(deserializer).map(|option_offset_date_time| {
                option_offset_date_time.map(|offset_date_time| {
                    let utc_date_time = offset_date_time.to_offset(UtcOffset::UTC);
                    PrimitiveDateTime::new(utc_date_time.date(), utc_date_time.time())
                })
            })
        }
    }
}

/// <https://github.com/serde-rs/serde/issues/994#issuecomment-316895860>

pub mod json_string {
    use serde::de::{self, Deserialize, DeserializeOwned, Deserializer};
    use serde_json;

    /// Deserialize a string which is in json format
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: DeserializeOwned,
        D: Deserializer<'de>,
    {
        let j = String::deserialize(deserializer)?;
        serde_json::from_str(&j).map_err(de::Error::custom)
    }
}
