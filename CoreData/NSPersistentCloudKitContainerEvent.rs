//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPersistentCloudKitContainerEventType {
        NSPersistentCloudKitContainerEventTypeSetup = 0,
        NSPersistentCloudKitContainerEventTypeImport = 1,
        NSPersistentCloudKitContainerEventTypeExport = 2,
    }
);

extern_static!(NSPersistentCloudKitContainerEventChangedNotification: &'static NSNotificationName);

extern_static!(NSPersistentCloudKitContainerEventUserInfoKey: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEvent")]
    pub struct NSPersistentCloudKitContainerEvent;

    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEvent")]
    unsafe impl ClassType for NSPersistentCloudKitContainerEvent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEvent")]
unsafe impl NSCopying for NSPersistentCloudKitContainerEvent {}

#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEvent")]
unsafe impl NSObjectProtocol for NSPersistentCloudKitContainerEvent {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentCloudKitContainerEvent")]
    unsafe impl NSPersistentCloudKitContainerEvent {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other storeIdentifier)]
        pub unsafe fn storeIdentifier(&self) -> Id<NSString>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> NSPersistentCloudKitContainerEventType;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Option<Id<NSDate>>;

        #[method(succeeded)]
        pub unsafe fn succeeded(&self) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
