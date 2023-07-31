// @generated automatically by Diesel CLI.

diesel::table! {
    connector_response (id) {
        id -> Int4,
        #[max_length = 255]
        payment_id -> Varchar,
        #[max_length = 255]
        merchant_id -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        amount -> Int4,
        #[max_length = 255]
        currency -> Nullable<Varchar>,
        amount_captured -> Nullable<Int4>,
        #[max_length = 255]
        customer_id -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        return_url -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        #[max_length = 255]
        connector_id -> Nullable<Varchar>,
        #[max_length = 255]
        shipping_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        billing_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_name -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_suffix -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        last_synced -> Nullable<Timestamp>,
        #[max_length = 255]
        setup_future_usage -> Nullable<Varchar>,
        off_session -> Nullable<Bool>,
        #[max_length = 255]
        client_secret -> Nullable<Varchar>,
    }
}

diesel::table! {
    dummy_one (id) {
        id -> Int4,
        #[max_length = 255]
        payment_id -> Varchar,
        #[max_length = 255]
        merchant_id -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        amount -> Int4,
        #[max_length = 255]
        currency -> Nullable<Varchar>,
        amount_captured -> Nullable<Int4>,
        #[max_length = 255]
        customer_id -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        return_url -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        #[max_length = 255]
        connector_id -> Nullable<Varchar>,
        #[max_length = 255]
        shipping_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        billing_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_name -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_suffix -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        last_synced -> Nullable<Timestamp>,
        #[max_length = 255]
        setup_future_usage -> Nullable<Varchar>,
        off_session -> Nullable<Bool>,
        #[max_length = 255]
        client_secret -> Nullable<Varchar>,
    }
}

diesel::table! {
    dummy_three (id) {
        id -> Int4,
        #[max_length = 255]
        payment_id -> Varchar,
        #[max_length = 255]
        merchant_id -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        amount -> Int4,
        #[max_length = 255]
        currency -> Nullable<Varchar>,
        amount_captured -> Nullable<Int4>,
        #[max_length = 255]
        customer_id -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        return_url -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        #[max_length = 255]
        connector_id -> Nullable<Varchar>,
        #[max_length = 255]
        shipping_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        billing_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_name -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_suffix -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        last_synced -> Nullable<Timestamp>,
        #[max_length = 255]
        setup_future_usage -> Nullable<Varchar>,
        off_session -> Nullable<Bool>,
        #[max_length = 255]
        client_secret -> Nullable<Varchar>,
    }
}

diesel::table! {
    dummy_two (id) {
        id -> Int4,
        #[max_length = 255]
        payment_id -> Varchar,
        #[max_length = 255]
        merchant_id -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        amount -> Int4,
        #[max_length = 255]
        currency -> Nullable<Varchar>,
        amount_captured -> Nullable<Int4>,
        #[max_length = 255]
        customer_id -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        return_url -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        #[max_length = 255]
        connector_id -> Nullable<Varchar>,
        #[max_length = 255]
        shipping_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        billing_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_name -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_suffix -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        last_synced -> Nullable<Timestamp>,
        #[max_length = 255]
        setup_future_usage -> Nullable<Varchar>,
        off_session -> Nullable<Bool>,
        #[max_length = 255]
        client_secret -> Nullable<Varchar>,
    }
}

diesel::table! {
    payment_attempt (id) {
        id -> Int4,
        #[max_length = 255]
        payment_id -> Varchar,
        #[max_length = 255]
        merchant_id -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        amount -> Int4,
        #[max_length = 255]
        currency -> Nullable<Varchar>,
        amount_captured -> Nullable<Int4>,
        #[max_length = 255]
        customer_id -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        return_url -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        #[max_length = 255]
        connector_id -> Nullable<Varchar>,
        #[max_length = 255]
        shipping_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        billing_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_name -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_suffix -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        last_synced -> Nullable<Timestamp>,
        #[max_length = 255]
        setup_future_usage -> Nullable<Varchar>,
        off_session -> Nullable<Bool>,
        #[max_length = 255]
        client_secret -> Nullable<Varchar>,
    }
}

diesel::table! {
    payment_intent (id) {
        id -> Int4,
        #[max_length = 255]
        payment_id -> Varchar,
        #[max_length = 255]
        merchant_id -> Varchar,
        #[max_length = 255]
        status -> Varchar,
        amount -> Int4,
        #[max_length = 255]
        currency -> Nullable<Varchar>,
        amount_captured -> Nullable<Int4>,
        #[max_length = 255]
        customer_id -> Nullable<Varchar>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        return_url -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        #[max_length = 255]
        connector_id -> Nullable<Varchar>,
        #[max_length = 255]
        shipping_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        billing_address_id -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_name -> Nullable<Varchar>,
        #[max_length = 255]
        statement_descriptor_suffix -> Nullable<Varchar>,
        created_at -> Timestamp,
        modified_at -> Timestamp,
        last_synced -> Nullable<Timestamp>,
        #[max_length = 255]
        setup_future_usage -> Nullable<Varchar>,
        off_session -> Nullable<Bool>,
        #[max_length = 255]
        client_secret -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    connector_response,
    dummy_one,
    dummy_three,
    dummy_two,
    payment_attempt,
    payment_intent,
);
