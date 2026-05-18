use std::{
    sync::{Arc, Mutex},
    task::{Poll, Waker},
    thread,
    time::Duration,
};

struct TimerState {
    completed: bool,
    waker: Option<Waker>,
}

pub struct TimerFuture {
    state: Arc<Mutex<TimerState>>,
}

impl TimerFuture {
    fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(TimerState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = Arc::clone(&state);

        thread::spawn(move || {
            thread::sleep(duration);

            let mut state = thread_shared_state.lock().unwrap();
            state.completed = true;
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });

        Self { state }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        if state.completed {
            Poll::Ready(())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    println!("ready????");
    let timer = TimerFuture::new(Duration::from_secs(5));
    timer.await;
    println!("DONE!!!");
}
