//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKStorefront")]
    pub struct SKStorefront;

    #[cfg(feature = "StoreKit_SKStorefront")]
    unsafe impl ClassType for SKStorefront {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKStorefront")]
unsafe impl Send for SKStorefront {}

#[cfg(feature = "StoreKit_SKStorefront")]
unsafe impl Sync for SKStorefront {}

#[cfg(feature = "StoreKit_SKStorefront")]
unsafe impl NSObjectProtocol for SKStorefront {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKStorefront")]
    unsafe impl SKStorefront {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other countryCode)]
        pub unsafe fn countryCode(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "StoreKit_SKStorefront")]
    unsafe impl SKStorefront {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
