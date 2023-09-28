//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXSetHeldCallAction")]
    pub struct CXSetHeldCallAction;

    #[cfg(feature = "CallKit_CXSetHeldCallAction")]
    unsafe impl ClassType for CXSetHeldCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXSetHeldCallAction")]
unsafe impl NSCoding for CXSetHeldCallAction {}

#[cfg(feature = "CallKit_CXSetHeldCallAction")]
unsafe impl NSCopying for CXSetHeldCallAction {}

#[cfg(feature = "CallKit_CXSetHeldCallAction")]
unsafe impl NSObjectProtocol for CXSetHeldCallAction {}

#[cfg(feature = "CallKit_CXSetHeldCallAction")]
unsafe impl NSSecureCoding for CXSetHeldCallAction {}

extern_methods!(
    #[cfg(feature = "CallKit_CXSetHeldCallAction")]
    unsafe impl CXSetHeldCallAction {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:onHold:)]
        pub unsafe fn initWithCallUUID_onHold(
            this: Allocated<Self>,
            call_uuid: &NSUUID,
            on_hold: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, a_decoder: &NSCoder)
            -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(this: Allocated<Self>, call_uuid: &NSUUID) -> Id<Self>;

        #[method(isOnHold)]
        pub unsafe fn isOnHold(&self) -> bool;

        #[method(setOnHold:)]
        pub unsafe fn setOnHold(&self, on_hold: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `CXCallAction`
    #[cfg(feature = "CallKit_CXSetHeldCallAction")]
    unsafe impl CXSetHeldCallAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CallKit_CXSetHeldCallAction")]
    unsafe impl CXSetHeldCallAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
