#![no_std]

extern crate alloc;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(feature = "buckle")]
pub mod buckle;
#[cfg(feature = "dclabel")]
pub mod dclabel;

pub trait Label {
    fn lub(self, rhs: Self) -> Self;
    fn glb(self, rhs: Self) -> Self;
    fn can_flow_to(&self, rhs: &Self) -> bool;
}

pub trait HasPrivilege {
    type Privilege;

    fn downgrade(self, privilege: &Self::Privilege) -> Self;
    fn downgrade_to(self, target: Self, privilege: &Self::Privilege) -> Self;
    fn can_flow_to_with_privilege(&self, rhs: &Self, privilege: &Self::Privilege) -> bool;
}
