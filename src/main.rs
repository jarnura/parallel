use opentelemetry::global;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "actix")]
#[cfg(any(feature = "diesel", feature = "sqlx"))]
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use futures::join;

// use console_subscriber;
use std::env;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use dotenvy::dotenv;

#[cfg(any(feature = "diesel", feature = "sqlx"))]
use parallel::{inserts::Inserts, pooling, reads::Reads};

use tracing;

#[cfg(feature = "diesel")]
#[tracing::instrument]
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
    use tracing_subscriber::prelude::*;

    let console_layer = console_subscriber::ConsoleLayer::builder()
        .with_default_env()
        .server_addr(([127, 0, 0, 1], 5555))
        .spawn();

    tracing_subscriber::registry()
        .with(console_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();

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

use actix_web_opentelemetry::{RequestMetrics, RequestTracing};
    use tracing_subscriber::prelude::*;

#[derive(Debug, Clone)]
pub struct OpenTelemetryStack {
    request_metrics: actix_web_opentelemetry::RequestMetrics,
}

impl OpenTelemetryStack {
    pub fn new() -> Self {
        dotenv().ok();
        let app_name = std::env::var("CARGO_BIN_NAME").unwrap_or("demo".to_string());

        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_endpoint(std::env::var("JAEGER_ENDPOINT").unwrap_or("localhost:6831".to_string()))
            .with_service_name(app_name.clone())
                
.with_auto_split_batch(true)            .install_batch(opentelemetry::runtime::Tokio)
            .expect("Failed to install OpenTelemetry tracer.");

        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let subscriber = tracing_subscriber::Registry::default().with(telemetry);
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to install `tracing` subscriber.");

        let request_metrics = RequestMetrics::default();
        Self {
            request_metrics,
        }
    }

    pub fn metrics(&self) -> actix_web_opentelemetry::RequestMetrics {
        self.request_metrics.clone()
    }
}


use tokio::sync::mpsc;

pub async fn receiver_for_error(rx: tokio::sync::oneshot::Receiver<()>, mut server: impl Stop) {
    match rx.await {
        Ok(sig) => {
            println!("Channel recevied {sig:?}");
        }
        Err(err) => {
            server.stop_server().await;
            println!("Channel receiver error{err}");
        }
    }
}

#[async_trait::async_trait]
pub trait Stop {
    async fn stop_server(&mut self);
}

#[async_trait::async_trait]
impl Stop for actix_web::dev::ServerHandle {
    async fn stop_server(&mut self) {
        let _ = self.stop(true).await;
    }
}
#[async_trait::async_trait]
impl Stop for mpsc::Sender<()> {
    async fn stop_server(&mut self) {
        let _ = self.send(()).await.map_err(|err| println!("{err}"));
    }
}

#[cfg(feature = "actix")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use std::fs::File;
    use std::io::Write;
    use pprof::protos::Message;
    
    // std::env::set_var("RUST_LOG", "debug");
    // env_logger::init();
    
    // tracing_subscriber::fmt::try_init()?;

    // global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    // let tracer = opentelemetry_jaeger::new_agent_pipeline()
        // .with_endpoint(std::env::var("JAEGER_ENDPOINT").unwrap_or("localhost:6831".to_string()))
        // .with_auto_split_batch(true)
        // .with_service_name("parallel-jaeger")
        // .install_batch(opentelemetry::runtime::Tokio)?;

    // let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // let console_layer = console_subscriber::ConsoleLayer::builder()
    //     .with_default_env()
    //     // .server_addr(([127, 0, 0, 1], 5555))
    //     .spawn();

    // let subscriber = tracing_subscriber::Registry::default().with(opentelemetry);
    // tracing::subscriber::set_global_default(subscriber)?;

    // tracing_subscriber::registry()
    //     // .with(console_layer)
    //     .with(opentelemetry)
    //     .try_init()?;
    let _prof_guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread", "vdso"]).build().unwrap();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let worker = env::var("WORKER").unwrap_or("1".to_string()).parse().unwrap_or(1);
    let shutdown_timeout = env::var("SHUTDOWN_TIMEOUT").unwrap_or("10".to_string()).parse().unwrap_or(10);
    let file_path = env::var("FILE_PATH").unwrap_or("/mnt/reports".to_string());
    println!("{database_url}");
        let telemetry = OpenTelemetryStack::new();
    let (tx, rx) = tokio::sync::oneshot::channel();

    #[cfg(any(feature = "diesel", feature = "sqlx"))]
    let _ = {
        let store = init(&database_url).await;
        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(store.clone()))
                .configure(config)
                // .route("/parallel", web::get().to(executor))
                // .route("/sequential", web::get().to(executor_sequential))
                .wrap(actix_web::middleware::Logger::default())
                .wrap(RequestTracing::new())
                .wrap(telemetry.metrics())
        })
        .workers(worker)
        .shutdown_timeout(shutdown_timeout)
        .bind(("0.0.0.0", 8080))?
        .run();
        
        tokio::spawn(receiver_for_error(rx, server.handle()));

        server.await.expect("Failed to create the server");
    };


    println!("Report generation started");

    let now = parallel::utils::current_time().to_string();
    
    if let Ok(report) = _prof_guard.report().build() {
        let file = File::create(format!("{file_path}/flamegraph_{now}.svg")).unwrap();
        let mut options = pprof::flamegraph::Options::default();
        options.image_width = Some(2500);
        report.flamegraph_with_options(file, &mut options).unwrap();
    };

    if let Ok(report) = _prof_guard.report().build() {
        let mut file = File::create(format!("{file_path}/profile_{now}.pb")).unwrap();
        let profile = report.pprof().unwrap();

        let mut content = Vec::new();
        profile.write_to_vec(&mut content).unwrap();
        file.write_all(&content).unwrap();
    };

    println!("Report generated");

    Ok(())
}

use actix_web::get;

#[cfg(feature = "actix")]
#[cfg(any(feature = "diesel", feature = "sqlx"))]
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/parallel").service(executor));
}

#[cfg(feature = "actix")]
#[cfg(any(feature = "diesel", feature = "sqlx"))]
#[tracing::instrument(skip_all)]
#[get("/")]
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
    // let pi_6_async = Inserts::insert_pi_with_instrument(store, 6);
    // let pi_7_async = Inserts::insert_pi_with_instrument(store, 7);
    // let pi_8_async = Inserts::insert_pi_with_instrument(store, 8);
    // let pi_9_async = Inserts::insert_pi_with_instrument(store, 9);
    // let pi_10_async = Inserts::insert_pi_with_instrument(store, 10);
    // let pi_11_async = Inserts::insert_pi_with_instrument(store, 11);
    // let pi_12_async = Inserts::insert_pi_with_instrument(store, 12);
    actix_web::rt::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let (
        pi_1_async,
        pi_2_async,
        pi_3_async,
        pi_4_async,
        pi_5_async,
        // pi_6_async,
        // pi_7_async,
        // pi_8_async,
        // pi_9_async,
        // pi_10_async,
        // pi_11_async,
        // pi_12_async,
    ) = join!(
        pi_1_async,
        pi_2_async,
        pi_3_async,
        pi_4_async,
        pi_5_async,
        // pi_6_async,
        // pi_7_async,
        // pi_8_async,
        // pi_9_async,
        // pi_10_async,
        // pi_11_async,
        // pi_12_async
    );

    // actix_web::rt::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let pi_1_async = Reads::read_pi_with_instrument(store, pi_1_async.payment_id, 1);
    let pi_2_async = Reads::read_pi_with_instrument(store, pi_2_async.payment_id, 2);
    let pi_3_async = Reads::read_pi_with_instrument(store, pi_3_async.payment_id, 3);
    let pi_4_async = Reads::read_pi_with_instrument(store, pi_4_async.payment_id, 4);
    let pi_5_async = Reads::read_pi_with_instrument(store, pi_5_async.payment_id, 5);
    // let pi_6_async = Reads::read_pi_with_instrument(store, pi_6_async.payment_id, 6);
    // let pi_7_async = Reads::read_pi_with_instrument(store, pi_7_async.payment_id, 7);
    // let pi_8_async = Reads::read_pi_with_instrument(store, pi_8_async.payment_id, 8);
    // let pi_9_async = Reads::read_pi_with_instrument(store, pi_9_async.payment_id, 9);
    // let pi_10_async = Reads::read_pi_with_instrument(store, pi_10_async.payment_id, 10);
    // let pi_11_async = Reads::read_pi_with_instrument(store, pi_11_async.payment_id, 11);
    // let pi_12_async = Reads::read_pi_with_instrument(store, pi_12_async.payment_id, 12);
    // actix_web::rt::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let _ = join!(
        pi_1_async,
        pi_2_async,
        pi_3_async,
        pi_4_async,
        pi_5_async,
        // pi_6_async,
        // pi_7_async,
        // pi_8_async,
        // pi_9_async,
        // pi_10_async,
        // pi_11_async,
        // pi_12_async
    );
    // actix_web::rt::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let end_time = parallel::utils::current_time();
    let diff = end_time - start_time;
    HttpResponse::Ok().body(format!("{diff}"))
}

#[cfg(feature = "actix")]
#[cfg(any(feature = "diesel", feature = "sqlx"))]
#[tracing::instrument]
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
#[cfg(any(feature = "diesel", feature = "sqlx"))]
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
#[cfg(any(feature = "diesel", feature = "sqlx"))]
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
