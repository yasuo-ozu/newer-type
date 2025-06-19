use crate::emit_traits;
use newer_type::target;

emit_traits! {
    #[implement_of(newer_type_std::ops::Deref)]
    #[slot(Box<usize>)]
    #[target(alternative = ::core::ops::Deref)]
    pub trait Deref {
        type Target: ?::core::marker::Sized;
        fn deref(&self) -> &Self::Target;
    }

    #[implement_of(
        newer_type_std::ops::Deref,
        newer_type_std::ops::DerefMut
    )]
    #[slot(Box<u8>)]
    #[target(alternative = ::core::ops::DerefMut)]
    pub trait DerefMut: [::core::ops::Deref] {
        fn deref_mut(&mut self) -> &mut Self::Target;
    }

    #[implement_of(for<Idx> newer_type_std::ops::Index<Idx>)]
    #[slot(Vec<u8>)]
    #[target(alternative = ::core::ops::Index)]
    pub trait Index[Idx: ?::core::marker::Sized] {
        type Output: ?::core::marker::Sized;
        fn index(&self, index: Idx) -> &Self::Output;
    }

    #[implement_of(for<Idx> newer_type_std::ops::Index<Idx>)]
    #[implement_of(for<Idx> newer_type_std::ops::IndexMut<Idx>)]
    #[slot(Vec<u8>)]
    #[target(alternative = ::core::ops::IndexMut)]
    pub trait IndexMut[Idx: ?::core::marker::Sized]: [::core::ops::Index<Idx>] {
        fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
    }

    #[implement_of(newer_type_std::ops::Not)]
    #[slot(bool)]
    #[target(alternative = ::core::ops::Not)]
    pub trait Not {
        type Output;
        fn not(self) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::BitAnd<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::BitAnd)]
    pub trait BitAnd[Rhs = Self] {
        type Output;
        fn bitand(self, rhs: Rhs) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::BitOr<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::BitOr)]
    pub trait BitOr[Rhs = Self] {
        type Output;
        fn bitor(self, rhs: Rhs) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::BitXor<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::BitXor)]
    pub trait BitXor[Rhs = Self] {
        type Output;
        fn bitxor(self, rhs: Rhs) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::Shl<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::Shl)]
    pub trait Shl[Rhs = Self] {
        type Output;
        fn shl(self, rhs: Rhs) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::Shr<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::Shr)]
    pub trait Shr[Rhs = Self] {
        type Output;
        fn shr(self, rhs: Rhs) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::BitAndAssign<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::BitAndAssign)]
    pub trait BitAndAssign[Rhs = Self] {
        fn bitand_assign(&mut self, rhs: Rhs);
    }
    #[implement_of(for<Rhs> newer_type_std::ops::BitOrAssign<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::BitOrAssign)]
    pub trait BitOrAssign[Rhs = Self] {
        fn bitor_assign(&mut self, rhs: Rhs);
    }
    #[implement_of(for<Rhs> newer_type_std::ops::BitXorAssign<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::BitXorAssign)]
    pub trait BitXorAssign[Rhs = Self] {
        fn bitxor_assign(&mut self, rhs: Rhs);
    }
    #[implement_of(for<Rhs> newer_type_std::ops::ShlAssign<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::ShlAssign)]
    pub trait ShlAssign[Rhs = Self] {
        fn shl_assign(&mut self, rhs: Rhs);
    }
    #[implement_of(for<Rhs> newer_type_std::ops::ShrAssign<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::ShrAssign)]
    pub trait ShrAssign[Rhs = Self] {
        fn shr_assign(&mut self, rhs: Rhs);
    }
    #[implement_of(for<Rhs> newer_type_std::ops::Add<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::Add)]
    pub trait Add[Rhs = Self] {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::Sub<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::Sub)]
    pub trait Sub[Rhs = Self] {
        type Output;
        fn sub(self, rhs: Rhs) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::Mul<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::Mul)]
    pub trait Mul[Rhs = Self] {
        type Output;
        fn mul(self, rhs: Rhs) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::Div<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::Div)]
    pub trait Div[Rhs = Self] {
        type Output;
        fn div(self, rhs: Rhs) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::Rem<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::Rem)]
    pub trait Rem[Rhs = Self] {
        type Output;
        fn rem(self, rhs: Rhs) -> Self::Output;
    }
    #[implement_of(newer_type_std::ops::Neg)]
    #[slot(isize)]
    #[target(alternative = ::core::ops::Neg)]
    pub trait Neg {
        type Output;
        fn neg(self) -> Self::Output;
    }
    #[implement_of(for<Rhs> newer_type_std::ops::AddAssign<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::AddAssign)]
    pub trait AddAssign[Rhs = Self] {
        fn add_assign(&mut self, rhs: Rhs);
    }
    #[implement_of(for<Rhs> newer_type_std::ops::SubAssign<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::SubAssign)]
    pub trait SubAssign[Rhs = Self] {
        fn sub_assign(&mut self, rhs: Rhs);
    }
    #[implement_of(for<Rhs> newer_type_std::ops::MulAssign<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::MulAssign)]
    pub trait MulAssign[Rhs = Self] {
        fn mul_assign(&mut self, rhs: Rhs);
    }
    #[implement_of(for<Rhs> newer_type_std::ops::DivAssign<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::DivAssign)]
    pub trait DivAssign[Rhs = Self] {
        fn div_assign(&mut self, rhs: Rhs);
    }
    #[implement_of(for<Rhs> newer_type_std::ops::RemAssign<Rhs>)]
    #[slot(usize)]
    #[target(alternative = ::core::ops::RemAssign)]
    pub trait RemAssign[Rhs = Self] {
        fn rem_assign(&mut self, rhs: Rhs);
    }
}
