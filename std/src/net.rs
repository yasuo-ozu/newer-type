use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(newer_type_std::net::ToSocketAddrs)]
    #[slot(String)]
    #[target(alternative = ::std::net::ToSocketAddrs)]
    pub trait ToSocketAddrs {
        type Iter: ::core::iter::Iterator<Item = ::std::net::SocketAddr>;
        fn to_socket_addrs(&self) -> ::std::io::Result<Self::Iter>;
    }
}
