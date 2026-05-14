use std::{
    pin::Pin,
    task::{Context, Poll},
};

struct CountdownFuture {
    count: usize,
}

impl CountdownFuture {
    fn new(count: usize) -> Self {
        Self { count }
    }
}

impl Future for CountdownFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // that's all folks!
        if self.count == 0 {
            println!("Liftoff!");
            Poll::Ready(())
        } else {
            println!("{}!", self.count);
            self.count -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {}
