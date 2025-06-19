use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[target(alternative = ::std::task::Wake)]
    pub trait Wake {
        fn wake(self: ::std::sync::Arc<Self>);
        fn wake_by_ref(self: &::std::sync::Arc<Self>);
    }
}
