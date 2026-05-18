use std::{pin::Pin, task::Poll};

struct Join<A, B>
where
    A: Future,
    B: Future,
{
    a: A,
    b: B,
}

impl<A, B> Join<A, B>
where
    A: Future,
    B: Future,
{
    fn new(a: A, b: B) -> Join<A, B> {
        Self { a, b }
    }
}

impl<A, B> Unpin for Join<A, B>
where
    A: Future + Unpin,
    B: Future + Unpin,
{
}

impl<A, B> Future for Join<A, B>
where
    A: Future + Unpin,
    B: Future + Unpin,
{
    type Output = (A::Output, B::Output);

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();

        if let Poll::Ready(result_a) = Pin::new(&mut this.a).poll(cx) {}
    }
}

// async fn main() {
// let joined = Join::new(
//
//
// )
// }
