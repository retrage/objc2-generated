//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSFetchIndexDescription")]
    pub struct NSFetchIndexDescription;

    #[cfg(feature = "CoreData_NSFetchIndexDescription")]
    unsafe impl ClassType for NSFetchIndexDescription {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSFetchIndexDescription")]
unsafe impl NSCoding for NSFetchIndexDescription {}

#[cfg(feature = "CoreData_NSFetchIndexDescription")]
unsafe impl NSCopying for NSFetchIndexDescription {}

#[cfg(feature = "CoreData_NSFetchIndexDescription")]
unsafe impl NSObjectProtocol for NSFetchIndexDescription {}

extern_methods!(
    #[cfg(feature = "CoreData_NSFetchIndexDescription")]
    unsafe impl NSFetchIndexDescription {
        #[cfg(all(
            feature = "CoreData_NSFetchIndexElementDescription",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithName:elements:)]
        pub unsafe fn initWithName_elements(
            this: Allocated<Self>,
            name: &NSString,
            elements: Option<&NSArray<NSFetchIndexElementDescription>>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[cfg(all(
            feature = "CoreData_NSFetchIndexElementDescription",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other elements)]
        pub unsafe fn elements(&self) -> Id<NSArray<NSFetchIndexElementDescription>>;

        #[cfg(all(
            feature = "CoreData_NSFetchIndexElementDescription",
            feature = "Foundation_NSArray"
        ))]
        #[method(setElements:)]
        pub unsafe fn setElements(&self, elements: &NSArray<NSFetchIndexElementDescription>);

        #[cfg(feature = "CoreData_NSEntityDescription")]
        #[method_id(@__retain_semantics Other entity)]
        pub unsafe fn entity(&self) -> Option<Id<NSEntityDescription>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other partialIndexPredicate)]
        pub unsafe fn partialIndexPredicate(&self) -> Option<Id<NSPredicate>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method(setPartialIndexPredicate:)]
        pub unsafe fn setPartialIndexPredicate(
            &self,
            partial_index_predicate: Option<&NSPredicate>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSFetchIndexDescription")]
    unsafe impl NSFetchIndexDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
