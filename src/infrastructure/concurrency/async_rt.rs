//! Async Runtime utils (Tokio/async-std)

pub fn spawn_async<F>(fut: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    tokio::spawn(fut);
}
