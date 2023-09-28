//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPathComponentCell")]
    pub struct NSPathComponentCell;

    #[cfg(feature = "AppKit_NSPathComponentCell")]
    unsafe impl ClassType for NSPathComponentCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSPathComponentCell")]
unsafe impl NSAccessibility for NSPathComponentCell {}

#[cfg(feature = "AppKit_NSPathComponentCell")]
unsafe impl NSAccessibilityElementProtocol for NSPathComponentCell {}

#[cfg(feature = "AppKit_NSPathComponentCell")]
unsafe impl NSCoding for NSPathComponentCell {}

#[cfg(feature = "AppKit_NSPathComponentCell")]
unsafe impl NSCopying for NSPathComponentCell {}

#[cfg(feature = "AppKit_NSPathComponentCell")]
unsafe impl NSObjectProtocol for NSPathComponentCell {}

#[cfg(feature = "AppKit_NSPathComponentCell")]
unsafe impl NSUserInterfaceItemIdentification for NSPathComponentCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPathComponentCell")]
    unsafe impl NSPathComponentCell {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSPathComponentCell")]
    unsafe impl NSPathComponentCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(this: Allocated<Self>, image: Option<&NSImage>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSPathComponentCell")]
    unsafe impl NSPathComponentCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSPathComponentCell")]
    unsafe impl NSPathComponentCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
