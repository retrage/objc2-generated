//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKServerChangeToken")]
    pub struct CKServerChangeToken;

    #[cfg(feature = "CloudKit_CKServerChangeToken")]
    unsafe impl ClassType for CKServerChangeToken {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "CloudKit_CKServerChangeToken")]
    unsafe impl CKServerChangeToken {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;
    }
);
