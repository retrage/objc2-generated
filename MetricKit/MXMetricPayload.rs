//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXMetricPayload")]
    pub struct MXMetricPayload;

    #[cfg(feature = "MetricKit_MXMetricPayload")]
    unsafe impl ClassType for MXMetricPayload {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXMetricPayload")]
unsafe impl NSCoding for MXMetricPayload {}

#[cfg(feature = "MetricKit_MXMetricPayload")]
unsafe impl NSObjectProtocol for MXMetricPayload {}

#[cfg(feature = "MetricKit_MXMetricPayload")]
unsafe impl NSSecureCoding for MXMetricPayload {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXMetricPayload")]
    unsafe impl MXMetricPayload {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other latestApplicationVersion)]
        pub unsafe fn latestApplicationVersion(&self) -> Id<NSString>;

        #[method(includesMultipleApplicationVersions)]
        pub unsafe fn includesMultipleApplicationVersions(&self) -> bool;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other timeStampBegin)]
        pub unsafe fn timeStampBegin(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other timeStampEnd)]
        pub unsafe fn timeStampEnd(&self) -> Id<NSDate>;

        #[cfg(feature = "MetricKit_MXCPUMetric")]
        #[method_id(@__retain_semantics Other cpuMetrics)]
        pub unsafe fn cpuMetrics(&self) -> Option<Id<MXCPUMetric>>;

        #[cfg(feature = "MetricKit_MXGPUMetric")]
        #[method_id(@__retain_semantics Other gpuMetrics)]
        pub unsafe fn gpuMetrics(&self) -> Option<Id<MXGPUMetric>>;

        #[cfg(feature = "MetricKit_MXCellularConditionMetric")]
        #[method_id(@__retain_semantics Other cellularConditionMetrics)]
        pub unsafe fn cellularConditionMetrics(&self) -> Option<Id<MXCellularConditionMetric>>;

        #[cfg(feature = "MetricKit_MXAppRunTimeMetric")]
        #[method_id(@__retain_semantics Other applicationTimeMetrics)]
        pub unsafe fn applicationTimeMetrics(&self) -> Option<Id<MXAppRunTimeMetric>>;

        #[cfg(feature = "MetricKit_MXLocationActivityMetric")]
        #[method_id(@__retain_semantics Other locationActivityMetrics)]
        pub unsafe fn locationActivityMetrics(&self) -> Option<Id<MXLocationActivityMetric>>;

        #[cfg(feature = "MetricKit_MXNetworkTransferMetric")]
        #[method_id(@__retain_semantics Other networkTransferMetrics)]
        pub unsafe fn networkTransferMetrics(&self) -> Option<Id<MXNetworkTransferMetric>>;

        #[cfg(feature = "MetricKit_MXAppLaunchMetric")]
        #[method_id(@__retain_semantics Other applicationLaunchMetrics)]
        pub unsafe fn applicationLaunchMetrics(&self) -> Option<Id<MXAppLaunchMetric>>;

        #[cfg(feature = "MetricKit_MXAppResponsivenessMetric")]
        #[method_id(@__retain_semantics Other applicationResponsivenessMetrics)]
        pub unsafe fn applicationResponsivenessMetrics(
            &self,
        ) -> Option<Id<MXAppResponsivenessMetric>>;

        #[cfg(feature = "MetricKit_MXDiskIOMetric")]
        #[method_id(@__retain_semantics Other diskIOMetrics)]
        pub unsafe fn diskIOMetrics(&self) -> Option<Id<MXDiskIOMetric>>;

        #[cfg(feature = "MetricKit_MXMemoryMetric")]
        #[method_id(@__retain_semantics Other memoryMetrics)]
        pub unsafe fn memoryMetrics(&self) -> Option<Id<MXMemoryMetric>>;

        #[cfg(feature = "MetricKit_MXDisplayMetric")]
        #[method_id(@__retain_semantics Other displayMetrics)]
        pub unsafe fn displayMetrics(&self) -> Option<Id<MXDisplayMetric>>;

        #[cfg(feature = "MetricKit_MXAnimationMetric")]
        #[method_id(@__retain_semantics Other animationMetrics)]
        pub unsafe fn animationMetrics(&self) -> Option<Id<MXAnimationMetric>>;

        #[cfg(feature = "MetricKit_MXAppExitMetric")]
        #[method_id(@__retain_semantics Other applicationExitMetrics)]
        pub unsafe fn applicationExitMetrics(&self) -> Option<Id<MXAppExitMetric>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MetricKit_MXSignpostMetric"))]
        #[method_id(@__retain_semantics Other signpostMetrics)]
        pub unsafe fn signpostMetrics(&self) -> Option<Id<NSArray<MXSignpostMetric>>>;

        #[cfg(feature = "MetricKit_MXMetaData")]
        #[method_id(@__retain_semantics Other metaData)]
        pub unsafe fn metaData(&self) -> Option<Id<MXMetaData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[deprecated]
        #[method_id(@__retain_semantics Other DictionaryRepresentation)]
        pub unsafe fn DictionaryRepresentation(&self) -> Id<NSDictionary>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Id<NSDictionary>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MetricKit_MXMetricPayload")]
    unsafe impl MXMetricPayload {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
