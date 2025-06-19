use newer_type::target;

macro_rules! impl_global_alloc {
    () => {
        /// This trait is empty declaration of [`::core::alloc::GlobalAlloc`] to be used
        /// with [`newer_type::implement`].
        ///
        /// # Example
        ///
        /// ```
        /// # use newer_type::implement;
        /// #[implement(newer_type_std::alloc::GlobalAlloc)]
        /// struct MyStruct {
        ///     slot: std::alloc::System
        /// }
        /// ```
        #[target(alternative = ::core::alloc::GlobalAlloc, newer_type = $crate::newer_type)]
        pub unsafe trait GlobalAlloc {
            unsafe fn alloc(&self, layout: ::core::alloc::Layout) -> *mut ::core::primitive::u8;

            unsafe fn dealloc(
                &self,
                ptr: *mut ::core::primitive::u8,
                layout: ::core::alloc::Layout,
            );

            unsafe fn alloc_zeroed(
                &self,
                layout: ::core::alloc::Layout,
            ) -> *mut ::core::primitive::u8;

            unsafe fn realloc(
                &self,
                ptr: *mut ::core::primitive::u8,
                layout: ::core::alloc::Layout,
                new_size: ::core::primitive::usize,
            ) -> *mut ::core::primitive::u8;
        }
    };
}

impl_global_alloc!();
