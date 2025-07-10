use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(for<Borrowed> newer_type_std::borrow::Borrow<Borrowed>)]
    #[slot(u8)]
    #[target(alternative = ::core::borrow::Borrow)]
    pub trait Borrow[Borrowed]
    where [Borrowed: ?::core::marker::Sized,]
    {
        // Required method
        fn borrow(&self) -> &Borrowed;
    }

    #[implement_of(for<Borrowed> newer_type_std::borrow::Borrow<Borrowed>)]
    #[implement_of(for<Borrowed> newer_type_std::borrow::BorrowMut<Borrowed>)]
    #[slot(u8)]
    #[target(alternative = ::core::borrow::BorrowMut)]
    pub trait BorrowMut[Borrowed]: [::core::borrow::Borrow<Borrowed>]
    where [Borrowed: ?::core::marker::Sized,]
    {
        fn borrow_mut(&mut self) -> &mut Borrowed;
    }
}

macro_rules! impl_to_owned {
    () => {
        #[target(alternative =  ::std::borrow::ToOwned, newer_type = $crate::newer_type, repeater = $crate::Repeater)]
        #[cfg(feature = "std")]
        /// This trait is empty declaration of [`::std::borrow::ToOwned`] to be used
        /// with [`newer_type::implement`].
        pub trait ToOwned {
            type Owned: ::core::borrow::Borrow<Self>;
            fn to_owned(&self) -> Self::Owned;

            fn clone_into(&self, target: &mut Self::Owned);
        }
    };
}
impl_to_owned! {}
