use std::{
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll, Waker},
};

struct FlagFuture {
    flag: Arc<AtomicBool>,
    waker_slot: Arc<Mutex<Option<Waker>>>,
}

impl FlagFuture {
    fn new() -> Self {
        Self {
            flag: Arc::new(AtomicBool::new(false)),
            waker_slot: Arc::new(Mutex::new(None)),
        }
    }
}

impl Future for FlagFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // check if the flag is true first thing
        if self.flag.load(Ordering::Acquire) {
            return Poll::Ready(());
        }

        // store/update waker to get notified
        let mut slot = self.waker_slot.lock().unwrap();
        *slot = Some(cx.waker().clone());

        // re-check after storing the waker to avoid a race
        // the flag could have been set between the first check
        // and storing the waker
        if self.flag.load(Ordering::Acquire) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

fn set_flag(flag: &AtomicBool, waker_slot: &Mutex<Option<Waker>>) {
    // store true in the flag
    flag.store(true, Ordering::Release);
    // wake up signal
    if let Some(waker) = waker_slot.lock().unwrap().take() {
        waker.wake();
    }
}

#[tokio::main]
async fn main() {}
