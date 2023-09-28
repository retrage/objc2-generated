//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKDocumentSample")]
    pub struct HKDocumentSample;

    #[cfg(feature = "HealthKit_HKDocumentSample")]
    unsafe impl ClassType for HKDocumentSample {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKDocumentSample")]
unsafe impl NSCoding for HKDocumentSample {}

#[cfg(feature = "HealthKit_HKDocumentSample")]
unsafe impl NSObjectProtocol for HKDocumentSample {}

#[cfg(feature = "HealthKit_HKDocumentSample")]
unsafe impl NSSecureCoding for HKDocumentSample {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKDocumentSample")]
    unsafe impl HKDocumentSample {
        #[cfg(feature = "HealthKit_HKDocumentType")]
        #[method_id(@__retain_semantics Other documentType)]
        pub unsafe fn documentType(&self) -> Id<HKDocumentType>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(feature = "HealthKit_HKDocumentSample")]
    unsafe impl HKDocumentSample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKDocumentSample")]
    unsafe impl HKDocumentSample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
