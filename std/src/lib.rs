#![cfg_attr(not(feature = "std"), no_std)]

pub mod alloc;
pub mod borrow;
pub mod cmp;
pub mod convert;
pub mod error;
pub mod fmt;
pub mod future;
pub mod hash;
#[cfg(feature = "std")]
pub mod io;
pub mod iter;
#[cfg(feature = "std")]
pub mod net;
pub mod ops;
#[cfg(feature = "std")]
pub mod process;
#[cfg(feature = "std")]
pub mod string;
#[cfg(feature = "std")]
pub mod task;

pub use newer_type;

macro_rules! emit_traits {
    () => {};
    (
        $(#[doc = $doc0:literal])*
        $(
            $(#[implement_of($($implement_of:tt)*)])+
            #[slot($($slot_ty:tt)*)]
        )?
        $(#[doc = $doc1:literal])*
        #[target(alternative = $alternative:path)]
        $(#[$($other_attr:tt)*])*
        pub trait $trait_name:ident $([$($trait_params:tt)+])? $(: [$($supertraits:tt)*])?
        $(where [$($where_clause:tt)*])?
        {$($trait_contents:tt)*}
        $($t:tt)*
    ) => {
        $(#[$($other_attr)*])*
        #[target(alternative = $alternative, newer_type = $crate::newer_type)]
        $(
            #[doc = $doc0]
            #[doc = ""]
        )*
        #[doc = concat!("This trait is empty declaration of [`", stringify!($alternative), "`] to be used")]
        #[doc = "with [`newer_type::implement`]."]
        $(
            #[doc = ""]
            #[doc = "# Example"]
            #[doc = ""]
            #[doc = "```"]
            #[doc = "# use newer_type::implement;"]
            $(#[doc = concat!("#[implement(", stringify!($($implement_of)*) ,")]")])+
            #[doc = "struct MyStruct {"]
            #[doc = concat!("    slot: ", stringify!($($slot_ty)*))]
            #[doc = "}"]
            #[doc = "```"]
        )?
        $(
            #[doc = ""]
            #[doc = $doc1]
        )*
        pub trait $trait_name $(< $($trait_params)+>)? $(:$($supertraits)*)?
        $(where $($where_clause)*)?
        {$($trait_contents)*}
        emit_traits!{ $($t)* }
    };
}
use emit_traits;
