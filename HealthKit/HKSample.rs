//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKSample")]
    pub struct HKSample;

    #[cfg(feature = "HealthKit_HKSample")]
    unsafe impl ClassType for HKSample {
        #[inherits(NSObject)]
        type Super = HKObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKSample")]
unsafe impl NSCoding for HKSample {}

#[cfg(feature = "HealthKit_HKSample")]
unsafe impl NSObjectProtocol for HKSample {}

#[cfg(feature = "HealthKit_HKSample")]
unsafe impl NSSecureCoding for HKSample {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKSample")]
    unsafe impl HKSample {
        #[cfg(feature = "HealthKit_HKSampleType")]
        #[method_id(@__retain_semantics Other sampleType)]
        pub unsafe fn sampleType(&self) -> Id<HKSampleType>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Id<NSDate>;

        #[method(hasUndeterminedDuration)]
        pub unsafe fn hasUndeterminedDuration(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(feature = "HealthKit_HKSample")]
    unsafe impl HKSample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKSample")]
    unsafe impl HKSample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(HKSampleSortIdentifierStartDate: &'static NSString);

extern_static!(HKSampleSortIdentifierEndDate: &'static NSString);

extern_static!(HKPredicateKeyPathStartDate: &'static NSString);

extern_static!(HKPredicateKeyPathEndDate: &'static NSString);
