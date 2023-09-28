//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_static!(HKObjectQueryNoLimit: NSUInteger = 0);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKSampleQuery")]
    pub struct HKSampleQuery;

    #[cfg(feature = "HealthKit_HKSampleQuery")]
    unsafe impl ClassType for HKSampleQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKSampleQuery")]
unsafe impl NSObjectProtocol for HKSampleQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKSampleQuery")]
    unsafe impl HKSampleQuery {
        #[method(limit)]
        pub unsafe fn limit(&self) -> NSUInteger;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Id<NSArray<NSSortDescriptor>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "Foundation_NSSortDescriptor",
            feature = "HealthKit_HKSample",
            feature = "HealthKit_HKSampleType"
        ))]
        #[method_id(@__retain_semantics Init initWithSampleType:predicate:limit:sortDescriptors:resultsHandler:)]
        pub unsafe fn initWithSampleType_predicate_limit_sortDescriptors_resultsHandler(
            this: Allocated<Self>,
            sample_type: &HKSampleType,
            predicate: Option<&NSPredicate>,
            limit: NSUInteger,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
            results_handler: &Block<
                (NonNull<HKSampleQuery>, *mut NSArray<HKSample>, *mut NSError),
                (),
            >,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKQueryDescriptor",
            feature = "HealthKit_HKSample"
        ))]
        #[method_id(@__retain_semantics Init initWithQueryDescriptors:limit:resultsHandler:)]
        pub unsafe fn initWithQueryDescriptors_limit_resultsHandler(
            this: Allocated<Self>,
            query_descriptors: &NSArray<HKQueryDescriptor>,
            limit: NSInteger,
            results_handler: &Block<
                (NonNull<HKSampleQuery>, *mut NSArray<HKSample>, *mut NSError),
                (),
            >,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSSortDescriptor",
            feature = "HealthKit_HKQueryDescriptor",
            feature = "HealthKit_HKSample"
        ))]
        #[method_id(@__retain_semantics Init initWithQueryDescriptors:limit:sortDescriptors:resultsHandler:)]
        pub unsafe fn initWithQueryDescriptors_limit_sortDescriptors_resultsHandler(
            this: Allocated<Self>,
            query_descriptors: &NSArray<HKQueryDescriptor>,
            limit: NSInteger,
            sort_descriptors: &NSArray<NSSortDescriptor>,
            results_handler: &Block<
                (NonNull<HKSampleQuery>, *mut NSArray<HKSample>, *mut NSError),
                (),
            >,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HealthKit_HKSampleQuery")]
    unsafe impl HKSampleQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKSampleQuery")]
    unsafe impl HKSampleQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
