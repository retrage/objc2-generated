//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXAppRunTimeMetric")]
    pub struct MXAppRunTimeMetric;

    #[cfg(feature = "MetricKit_MXAppRunTimeMetric")]
    unsafe impl ClassType for MXAppRunTimeMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXAppRunTimeMetric")]
unsafe impl NSCoding for MXAppRunTimeMetric {}

#[cfg(feature = "MetricKit_MXAppRunTimeMetric")]
unsafe impl NSObjectProtocol for MXAppRunTimeMetric {}

#[cfg(feature = "MetricKit_MXAppRunTimeMetric")]
unsafe impl NSSecureCoding for MXAppRunTimeMetric {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXAppRunTimeMetric")]
    unsafe impl MXAppRunTimeMetric {
        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitDuration"
        ))]
        #[method_id(@__retain_semantics Other cumulativeForegroundTime)]
        pub unsafe fn cumulativeForegroundTime(&self) -> Id<NSMeasurement<NSUnitDuration>>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitDuration"
        ))]
        #[method_id(@__retain_semantics Other cumulativeBackgroundTime)]
        pub unsafe fn cumulativeBackgroundTime(&self) -> Id<NSMeasurement<NSUnitDuration>>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitDuration"
        ))]
        #[method_id(@__retain_semantics Other cumulativeBackgroundAudioTime)]
        pub unsafe fn cumulativeBackgroundAudioTime(&self) -> Id<NSMeasurement<NSUnitDuration>>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitDuration"
        ))]
        #[method_id(@__retain_semantics Other cumulativeBackgroundLocationTime)]
        pub unsafe fn cumulativeBackgroundLocationTime(&self) -> Id<NSMeasurement<NSUnitDuration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MetricKit_MXAppRunTimeMetric")]
    unsafe impl MXAppRunTimeMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
