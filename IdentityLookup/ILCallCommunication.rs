//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILCallCommunication")]
    pub struct ILCallCommunication;

    #[cfg(feature = "IdentityLookup_ILCallCommunication")]
    unsafe impl ClassType for ILCallCommunication {
        #[inherits(NSObject)]
        type Super = ILCommunication;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "IdentityLookup_ILCallCommunication")]
unsafe impl NSCoding for ILCallCommunication {}

#[cfg(feature = "IdentityLookup_ILCallCommunication")]
unsafe impl NSObjectProtocol for ILCallCommunication {}

#[cfg(feature = "IdentityLookup_ILCallCommunication")]
unsafe impl NSSecureCoding for ILCallCommunication {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILCallCommunication")]
    unsafe impl ILCallCommunication {
        #[method(isEqualToCallCommunication:)]
        pub unsafe fn isEqualToCallCommunication(
            &self,
            communication: &ILCallCommunication,
        ) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "IdentityLookup_ILCallCommunication")]
    unsafe impl ILCallCommunication {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
