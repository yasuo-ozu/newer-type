use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[target(alternative = ::core::future::Future)]
    pub trait Future {
        type Output;
        fn poll(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<Self::Output>;
    }
}

// `core::future::IntoFuture` was stabilized in Rust 1.64; omit the forwarding
// declaration on older toolchains where the target trait does not exist.
#[rustversion::since(1.64)]
emit_traits! {
    #[target(alternative = ::core::future::IntoFuture)]
    pub trait IntoFuture {
        type Output;
        type IntoFuture: ::core::future::Future<Output = Self::Output>;
        fn into_future(self) -> Self::IntoFuture;
    }
}
