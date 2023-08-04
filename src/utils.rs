use time::{OffsetDateTime, PrimitiveDateTime};

pub fn current_time() -> PrimitiveDateTime {
    let utc_date_time = OffsetDateTime::now_utc();
    PrimitiveDateTime::new(utc_date_time.date(), utc_date_time.time())
}

pub async fn instrument<F, Fut, B>(func: F, ix: i8) -> B
where
    F: FnOnce() -> Fut + Send,
    Fut: futures::Future<Output = B> + Send,
{
    println!("pi entering {}", ix);
    let start_time = current_time();
    let result = func().await;
    let end_time = current_time();
    println!("{}", end_time - start_time);
    println!("pi exit {}", ix);
    result
}
