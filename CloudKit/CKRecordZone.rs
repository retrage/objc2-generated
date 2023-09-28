//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum CKRecordZoneCapabilities {
        CKRecordZoneCapabilityFetchChanges = 1 << 0,
        CKRecordZoneCapabilityAtomic = 1 << 1,
        CKRecordZoneCapabilitySharing = 1 << 2,
        CKRecordZoneCapabilityZoneWideSharing = 1 << 3,
    }
);

extern_static!(CKRecordZoneDefaultName: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKRecordZone")]
    pub struct CKRecordZone;

    #[cfg(feature = "CloudKit_CKRecordZone")]
    unsafe impl ClassType for CKRecordZone {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKRecordZone")]
unsafe impl NSCoding for CKRecordZone {}

#[cfg(feature = "CloudKit_CKRecordZone")]
unsafe impl NSCopying for CKRecordZone {}

#[cfg(feature = "CloudKit_CKRecordZone")]
unsafe impl NSObjectProtocol for CKRecordZone {}

#[cfg(feature = "CloudKit_CKRecordZone")]
unsafe impl NSSecureCoding for CKRecordZone {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKRecordZone")]
    unsafe impl CKRecordZone {
        #[method_id(@__retain_semantics Other defaultRecordZone)]
        pub unsafe fn defaultRecordZone() -> Id<CKRecordZone>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithZoneName:)]
        pub unsafe fn initWithZoneName(this: Allocated<Self>, zone_name: &NSString) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Init initWithZoneID:)]
        pub unsafe fn initWithZoneID(this: Allocated<Self>, zone_id: &CKRecordZoneID) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Id<CKRecordZoneID>;

        #[method(capabilities)]
        pub unsafe fn capabilities(&self) -> CKRecordZoneCapabilities;

        #[cfg(feature = "CloudKit_CKReference")]
        #[method_id(@__retain_semantics Other share)]
        pub unsafe fn share(&self) -> Option<Id<CKReference>>;
    }
);
