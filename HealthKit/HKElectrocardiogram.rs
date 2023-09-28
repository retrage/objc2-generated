//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKElectrocardiogramLead {
        HKElectrocardiogramLeadAppleWatchSimilarToLeadI = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKElectrocardiogramClassification {
        HKElectrocardiogramClassificationNotSet = 0,
        HKElectrocardiogramClassificationSinusRhythm = 1,
        HKElectrocardiogramClassificationAtrialFibrillation = 2,
        HKElectrocardiogramClassificationInconclusiveLowHeartRate = 3,
        HKElectrocardiogramClassificationInconclusiveHighHeartRate = 4,
        HKElectrocardiogramClassificationInconclusivePoorReading = 5,
        HKElectrocardiogramClassificationInconclusiveOther = 6,
        HKElectrocardiogramClassificationUnrecognized = 100,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKElectrocardiogramSymptomsStatus {
        HKElectrocardiogramSymptomsStatusNotSet = 0,
        HKElectrocardiogramSymptomsStatusNone = 1,
        HKElectrocardiogramSymptomsStatusPresent = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    pub struct HKElectrocardiogram;

    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    unsafe impl ClassType for HKElectrocardiogram {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKElectrocardiogram")]
unsafe impl NSCoding for HKElectrocardiogram {}

#[cfg(feature = "HealthKit_HKElectrocardiogram")]
unsafe impl NSObjectProtocol for HKElectrocardiogram {}

#[cfg(feature = "HealthKit_HKElectrocardiogram")]
unsafe impl NSSecureCoding for HKElectrocardiogram {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    unsafe impl HKElectrocardiogram {
        #[method(numberOfVoltageMeasurements)]
        pub unsafe fn numberOfVoltageMeasurements(&self) -> NSInteger;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other samplingFrequency)]
        pub unsafe fn samplingFrequency(&self) -> Option<Id<HKQuantity>>;

        #[method(classification)]
        pub unsafe fn classification(&self) -> HKElectrocardiogramClassification;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other averageHeartRate)]
        pub unsafe fn averageHeartRate(&self) -> Option<Id<HKQuantity>>;

        #[method(symptomsStatus)]
        pub unsafe fn symptomsStatus(&self) -> HKElectrocardiogramSymptomsStatus;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    unsafe impl HKElectrocardiogram {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKElectrocardiogram")]
    unsafe impl HKElectrocardiogram {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(HKPredicateKeyPathAverageHeartRate: &'static NSString);

extern_static!(HKPredicateKeyPathECGClassification: &'static NSString);

extern_static!(HKPredicateKeyPathECGSymptomsStatus: &'static NSString);
