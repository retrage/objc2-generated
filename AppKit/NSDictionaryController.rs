//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
    pub struct NSDictionaryControllerKeyValuePair;

    #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
    unsafe impl ClassType for NSDictionaryControllerKeyValuePair {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
unsafe impl NSObjectProtocol for NSDictionaryControllerKeyValuePair {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
    unsafe impl NSDictionaryControllerKeyValuePair {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: Option<&NSString>);

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Id<AnyObject>>;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&AnyObject>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedKey)]
        pub unsafe fn localizedKey(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedKey:)]
        pub unsafe fn setLocalizedKey(&self, localized_key: Option<&NSString>);

        #[method(isExplicitlyIncluded)]
        pub unsafe fn isExplicitlyIncluded(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
    unsafe impl NSDictionaryControllerKeyValuePair {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDictionaryController")]
    pub struct NSDictionaryController;

    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl ClassType for NSDictionaryController {
        #[inherits(NSObjectController, NSController, NSObject)]
        type Super = NSArrayController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSCoding for NSDictionaryController {}

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSEditor for NSDictionaryController {}

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSEditorRegistration for NSDictionaryController {}

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSObjectProtocol for NSDictionaryController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl NSDictionaryController {
        #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
        #[method_id(@__retain_semantics New newObject)]
        pub unsafe fn newObject(&self) -> Id<NSDictionaryControllerKeyValuePair>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other initialKey)]
        pub unsafe fn initialKey(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setInitialKey:)]
        pub unsafe fn setInitialKey(&self, initial_key: &NSString);

        #[method_id(@__retain_semantics Other initialValue)]
        pub unsafe fn initialValue(&self) -> Id<AnyObject>;

        #[method(setInitialValue:)]
        pub unsafe fn setInitialValue(&self, initial_value: &AnyObject);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other includedKeys)]
        pub unsafe fn includedKeys(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setIncludedKeys:)]
        pub unsafe fn setIncludedKeys(&self, included_keys: &NSArray<NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other excludedKeys)]
        pub unsafe fn excludedKeys(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setExcludedKeys:)]
        pub unsafe fn setExcludedKeys(&self, excluded_keys: &NSArray<NSString>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other localizedKeyDictionary)]
        pub unsafe fn localizedKeyDictionary(&self) -> Id<NSDictionary<NSString, NSString>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setLocalizedKeyDictionary:)]
        pub unsafe fn setLocalizedKeyDictionary(
            &self,
            localized_key_dictionary: &NSDictionary<NSString, NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedKeyTable)]
        pub unsafe fn localizedKeyTable(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedKeyTable:)]
        pub unsafe fn setLocalizedKeyTable(&self, localized_key_table: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObjectController`
    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl NSDictionaryController {
        #[method_id(@__retain_semantics Init initWithContent:)]
        pub unsafe fn initWithContent(
            this: Allocated<Self>,
            content: Option<&AnyObject>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSController`
    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl NSDictionaryController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl NSDictionaryController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
