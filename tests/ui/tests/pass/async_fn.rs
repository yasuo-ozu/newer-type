// `async fn` in a trait (Rust 1.75) is forwarded with `.await`. This is a pass
// test of the UI crate, which needs a newer rustc than newer-type's MSRV.
#![allow(async_fn_in_trait)]

use newer_type_test_facade::newer_type::announce::announce;
use newer_type_test_facade::newer_type::implement;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

#[announce(crate = ::newer_type_test_facade::newer_type::announce)]
trait Counter {
    async fn get(&self) -> u8;
    async fn add(&mut self, n: u8) -> u8;
    async fn into_inner(self) -> u8;
    async fn with<T: ::core::ops::Add<u8, Output = u8>>(&self, t: T) -> u8;
}

struct Plain(u8);

impl Counter for Plain {
    async fn get(&self) -> u8 {
        self.0
    }

    async fn add(&mut self, n: u8) -> u8 {
        self.0 += n;
        self.0
    }

    async fn into_inner(self) -> u8 {
        self.0
    }

    async fn with<T: ::core::ops::Add<u8, Output = u8>>(&self, t: T) -> u8 {
        t + self.0
    }
}

#[implement(newer_type = ::newer_type_test_facade::newer_type, Counter)]
struct Wrap(Plain);

#[implement(newer_type = ::newer_type_test_facade::newer_type, Counter)]
struct Named {
    counter: Plain,
}

#[implement(newer_type = ::newer_type_test_facade::newer_type, Counter)]
enum Either {
    A(Plain),
    B(Plain),
}

fn block_on<F: Future>(future: F) -> F::Output {
    struct Noop;
    impl Wake for Noop {
        fn wake(self: Arc<Self>) {}
    }
    let waker = Waker::from(Arc::new(Noop));
    let mut cx = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    loop {
        if let Poll::Ready(output) = Pin::as_mut(&mut future).poll(&mut cx) {
            return output;
        }
    }
}

fn main() {
    let mut wrap = Wrap(Plain(1));
    assert_eq!(block_on(wrap.get()), 1);
    assert_eq!(block_on(wrap.add(2)), 3);
    assert_eq!(block_on(wrap.with(4u8)), 7);
    assert_eq!(block_on(wrap.into_inner()), 3);

    let mut named = Named { counter: Plain(5) };
    assert_eq!(block_on(named.add(1)), 6);
    assert_eq!(block_on(named.into_inner()), 6);

    let a = Either::A(Plain(10));
    let mut b = Either::B(Plain(20));
    assert_eq!(block_on(a.get()), 10);
    assert_eq!(block_on(b.add(1)), 21);
    assert_eq!(block_on(b.with(1u8)), 22);
    assert_eq!(block_on(a.into_inner()), 10);
    assert_eq!(block_on(b.into_inner()), 21);
}
