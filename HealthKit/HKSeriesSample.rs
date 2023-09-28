//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKSeriesSample")]
    pub struct HKSeriesSample;

    #[cfg(feature = "HealthKit_HKSeriesSample")]
    unsafe impl ClassType for HKSeriesSample {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKSeriesSample")]
unsafe impl NSCoding for HKSeriesSample {}

#[cfg(feature = "HealthKit_HKSeriesSample")]
unsafe impl NSObjectProtocol for HKSeriesSample {}

#[cfg(feature = "HealthKit_HKSeriesSample")]
unsafe impl NSSecureCoding for HKSeriesSample {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKSeriesSample")]
    unsafe impl HKSeriesSample {
        #[method(count)]
        pub unsafe fn count(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(feature = "HealthKit_HKSeriesSample")]
    unsafe impl HKSeriesSample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKSeriesSample")]
    unsafe impl HKSeriesSample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
