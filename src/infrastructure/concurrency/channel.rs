//! Channel Communication (mpsc/oneshot)

use tokio::sync::{mpsc, oneshot};

pub fn channel_mpsc<T>() -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
    mpsc::channel(32)
}

pub fn channel_oneshot<T>() -> (oneshot::Sender<T>, oneshot::Receiver<T>) {
    oneshot::channel()
}
