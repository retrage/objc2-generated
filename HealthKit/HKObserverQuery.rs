//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

pub type HKObserverQueryCompletionHandler = *mut Block<(), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKObserverQuery")]
    pub struct HKObserverQuery;

    #[cfg(feature = "HealthKit_HKObserverQuery")]
    unsafe impl ClassType for HKObserverQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKObserverQuery")]
unsafe impl NSObjectProtocol for HKObserverQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKObserverQuery")]
    unsafe impl HKObserverQuery {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "HealthKit_HKSampleType"
        ))]
        #[method_id(@__retain_semantics Init initWithSampleType:predicate:updateHandler:)]
        pub unsafe fn initWithSampleType_predicate_updateHandler(
            this: Allocated<Self>,
            sample_type: &HKSampleType,
            predicate: Option<&NSPredicate>,
            update_handler: &Block<
                (
                    NonNull<HKObserverQuery>,
                    HKObserverQueryCompletionHandler,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSSet",
            feature = "HealthKit_HKQueryDescriptor",
            feature = "HealthKit_HKSampleType"
        ))]
        #[method_id(@__retain_semantics Init initWithQueryDescriptors:updateHandler:)]
        pub unsafe fn initWithQueryDescriptors_updateHandler(
            this: Allocated<Self>,
            query_descriptors: &NSArray<HKQueryDescriptor>,
            update_handler: &Block<
                (
                    NonNull<HKObserverQuery>,
                    *mut NSSet<HKSampleType>,
                    HKObserverQueryCompletionHandler,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `HKQuery`
    #[cfg(feature = "HealthKit_HKObserverQuery")]
    unsafe impl HKObserverQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKObserverQuery")]
    unsafe impl HKObserverQuery {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
