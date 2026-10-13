#![cfg_attr(not(feature = "std"), no_std)]
//! std/core traits for [`newer_type::implement`].
//!
//! The traits come from [`announce_std`]: every
//! `newer_type_std::<module>::<Trait>` is the `announce_std` trampoline of that
//! trait (its entry META carries `mock(::core::..::Trait)`, which
//! `#[implement]` uses as the trait to implement), and in the type namespace
//! the real `core` trait.

macro_rules! reexport {
    ($($(#[$m:meta])* $name:ident),* $(,)?) => {$(
        $(#[$m])*
        pub mod $name {
            #[doc(inline)]
            pub use ::announce_std::$name::*;
        }
    )*};
}

reexport! {
    /// ```
    /// # use newer_type::implement;
    /// #[implement(newer_type_std::alloc::GlobalAlloc)]
    /// struct MyStruct {
    ///     slot: std::alloc::System
    /// }
    /// ```
    alloc,
    borrow,
    cmp,
    convert,
    error,
    fmt,
    future,
    hash,
    #[cfg(feature = "std")] io,
    iter,
    #[cfg(feature = "std")] net,
    ops,
    #[cfg(feature = "std")] process,
    #[cfg(feature = "std")] string,
    #[cfg(feature = "std")] task,
}

pub use newer_type;

pub trait Repeater<T: ?Sized, const TRAIT_ID: u64, const NTH: usize> {
    type Type: ?Sized;
}
