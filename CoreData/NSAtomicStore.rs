//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSAtomicStore")]
    pub struct NSAtomicStore;

    #[cfg(feature = "CoreData_NSAtomicStore")]
    unsafe impl ClassType for NSAtomicStore {
        #[inherits(NSObject)]
        type Super = NSPersistentStore;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSAtomicStore")]
unsafe impl NSObjectProtocol for NSAtomicStore {}

extern_methods!(
    #[cfg(feature = "CoreData_NSAtomicStore")]
    unsafe impl NSAtomicStore {
        #[cfg(all(
            feature = "CoreData_NSPersistentStoreCoordinator",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithPersistentStoreCoordinator:configurationName:URL:options:)]
        pub unsafe fn initWithPersistentStoreCoordinator_configurationName_URL_options(
            this: Allocated<Self>,
            coordinator: Option<&NSPersistentStoreCoordinator>,
            configuration_name: Option<&NSString>,
            url: &NSURL,
            options: Option<&NSDictionary>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(load:_)]
        pub unsafe fn load(&self) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(save:_)]
        pub unsafe fn save(&self) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "CoreData_NSAtomicStoreCacheNode",
            feature = "CoreData_NSManagedObject"
        ))]
        #[method_id(@__retain_semantics New newCacheNodeForManagedObject:)]
        pub unsafe fn newCacheNodeForManagedObject(
            &self,
            managed_object: &NSManagedObject,
        ) -> Id<NSAtomicStoreCacheNode>;

        #[cfg(all(
            feature = "CoreData_NSAtomicStoreCacheNode",
            feature = "CoreData_NSManagedObject"
        ))]
        #[method(updateCacheNode:fromManagedObject:)]
        pub unsafe fn updateCacheNode_fromManagedObject(
            &self,
            node: &NSAtomicStoreCacheNode,
            managed_object: &NSManagedObject,
        );

        #[cfg(all(
            feature = "CoreData_NSAtomicStoreCacheNode",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other cacheNodes)]
        pub unsafe fn cacheNodes(&self) -> Id<NSSet<NSAtomicStoreCacheNode>>;

        #[cfg(all(
            feature = "CoreData_NSAtomicStoreCacheNode",
            feature = "Foundation_NSSet"
        ))]
        #[method(addCacheNodes:)]
        pub unsafe fn addCacheNodes(&self, cache_nodes: &NSSet<NSAtomicStoreCacheNode>);

        #[cfg(all(
            feature = "CoreData_NSAtomicStoreCacheNode",
            feature = "Foundation_NSSet"
        ))]
        #[method(willRemoveCacheNodes:)]
        pub unsafe fn willRemoveCacheNodes(&self, cache_nodes: &NSSet<NSAtomicStoreCacheNode>);

        #[cfg(all(
            feature = "CoreData_NSAtomicStoreCacheNode",
            feature = "CoreData_NSManagedObjectID"
        ))]
        #[method_id(@__retain_semantics Other cacheNodeForObjectID:)]
        pub unsafe fn cacheNodeForObjectID(
            &self,
            object_id: &NSManagedObjectID,
        ) -> Option<Id<NSAtomicStoreCacheNode>>;

        #[cfg(all(
            feature = "CoreData_NSEntityDescription",
            feature = "CoreData_NSManagedObjectID"
        ))]
        #[method_id(@__retain_semantics Other objectIDForEntity:referenceObject:)]
        pub unsafe fn objectIDForEntity_referenceObject(
            &self,
            entity: &NSEntityDescription,
            data: &AnyObject,
        ) -> Id<NSManagedObjectID>;

        #[cfg(feature = "CoreData_NSManagedObject")]
        #[method_id(@__retain_semantics New newReferenceObjectForManagedObject:)]
        pub unsafe fn newReferenceObjectForManagedObject(
            &self,
            managed_object: &NSManagedObject,
        ) -> Id<AnyObject>;

        #[cfg(feature = "CoreData_NSManagedObjectID")]
        #[method_id(@__retain_semantics Other referenceObjectForObjectID:)]
        pub unsafe fn referenceObjectForObjectID(
            &self,
            object_id: &NSManagedObjectID,
        ) -> Id<AnyObject>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSPersistentStore`
    #[cfg(feature = "CoreData_NSAtomicStore")]
    unsafe impl NSAtomicStore {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSAtomicStore")]
    unsafe impl NSAtomicStore {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
