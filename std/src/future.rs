use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[target(alternative = ::core::future::Future)]
    pub trait Future {
        type Output;
        fn poll(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<Self::Output>;
    }

    #[target(alternative = ::core::future::IntoFuture)]
    pub trait IntoFuture {
        type Output;
        type IntoFuture: ::core::future::Future<Output = Self::Output>;
        fn into_future(self) -> Self::IntoFuture;
    }
}
