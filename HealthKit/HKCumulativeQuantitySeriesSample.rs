//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKCumulativeQuantitySeriesSample")]
    #[deprecated]
    pub struct HKCumulativeQuantitySeriesSample;

    #[cfg(feature = "HealthKit_HKCumulativeQuantitySeriesSample")]
    unsafe impl ClassType for HKCumulativeQuantitySeriesSample {
        #[inherits(HKQuantitySample, HKSample, HKObject, NSObject)]
        type Super = HKCumulativeQuantitySample;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKCumulativeQuantitySeriesSample")]
unsafe impl NSCoding for HKCumulativeQuantitySeriesSample {}

#[cfg(feature = "HealthKit_HKCumulativeQuantitySeriesSample")]
unsafe impl NSObjectProtocol for HKCumulativeQuantitySeriesSample {}

#[cfg(feature = "HealthKit_HKCumulativeQuantitySeriesSample")]
unsafe impl NSSecureCoding for HKCumulativeQuantitySeriesSample {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKCumulativeQuantitySeriesSample")]
    unsafe impl HKCumulativeQuantitySeriesSample {
        #[cfg(feature = "HealthKit_HKQuantity")]
        #[deprecated]
        #[method_id(@__retain_semantics Other sum)]
        pub unsafe fn sum(&self) -> Id<HKQuantity>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuantitySample`
    #[cfg(feature = "HealthKit_HKCumulativeQuantitySeriesSample")]
    unsafe impl HKCumulativeQuantitySeriesSample {
        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "HealthKit_HKQuantity",
            feature = "HealthKit_HKQuantityType"
        ))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKQuantity",
            feature = "HealthKit_HKQuantityType"
        ))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:metadata:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate_metadata(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKDevice",
            feature = "HealthKit_HKQuantity",
            feature = "HealthKit_HKQuantityType"
        ))]
        #[method_id(@__retain_semantics Other quantitySampleWithType:quantity:startDate:endDate:device:metadata:)]
        pub unsafe fn quantitySampleWithType_quantity_startDate_endDate_device_metadata(
            quantity_type: &HKQuantityType,
            quantity: &HKQuantity,
            start_date: &NSDate,
            end_date: &NSDate,
            device: Option<&HKDevice>,
            metadata: Option<&NSDictionary<NSString, AnyObject>>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKObject`
    #[cfg(feature = "HealthKit_HKCumulativeQuantitySeriesSample")]
    unsafe impl HKCumulativeQuantitySeriesSample {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKCumulativeQuantitySeriesSample")]
    unsafe impl HKCumulativeQuantitySeriesSample {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
