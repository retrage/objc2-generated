//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKSourceRevision")]
    pub struct HKSourceRevision;

    #[cfg(feature = "HealthKit_HKSourceRevision")]
    unsafe impl ClassType for HKSourceRevision {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKSourceRevision")]
unsafe impl NSCoding for HKSourceRevision {}

#[cfg(feature = "HealthKit_HKSourceRevision")]
unsafe impl NSCopying for HKSourceRevision {}

#[cfg(feature = "HealthKit_HKSourceRevision")]
unsafe impl NSObjectProtocol for HKSourceRevision {}

#[cfg(feature = "HealthKit_HKSourceRevision")]
unsafe impl NSSecureCoding for HKSourceRevision {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKSourceRevision")]
    unsafe impl HKSourceRevision {
        #[cfg(feature = "HealthKit_HKSource")]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Id<HKSource>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other version)]
        pub unsafe fn version(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other productType)]
        pub unsafe fn productType(&self) -> Option<Id<NSString>>;

        #[method(operatingSystemVersion)]
        pub unsafe fn operatingSystemVersion(&self) -> NSOperatingSystemVersion;

        #[cfg(all(feature = "Foundation_NSString", feature = "HealthKit_HKSource"))]
        #[method_id(@__retain_semantics Init initWithSource:version:productType:operatingSystemVersion:)]
        pub unsafe fn initWithSource_version_productType_operatingSystemVersion(
            this: Allocated<Self>,
            source: &HKSource,
            version: Option<&NSString>,
            product_type: Option<&NSString>,
            operating_system_version: NSOperatingSystemVersion,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "HealthKit_HKSource"))]
        #[method_id(@__retain_semantics Init initWithSource:version:)]
        pub unsafe fn initWithSource_version(
            this: Allocated<Self>,
            source: &HKSource,
            version: Option<&NSString>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKSourceRevision")]
    unsafe impl HKSourceRevision {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(HKSourceRevisionAnyVersion: &'static NSString);

extern_static!(HKSourceRevisionAnyProductType: &'static NSString);

extern_static!(HKSourceRevisionAnyOperatingSystem: NSOperatingSystemVersion);
