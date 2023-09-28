//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTableViewRowActionStyle {
        NSTableViewRowActionStyleRegular = 0,
        NSTableViewRowActionStyleDestructive = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTableViewRowAction")]
    pub struct NSTableViewRowAction;

    #[cfg(feature = "AppKit_NSTableViewRowAction")]
    unsafe impl ClassType for NSTableViewRowAction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTableViewRowAction")]
unsafe impl NSCopying for NSTableViewRowAction {}

#[cfg(feature = "AppKit_NSTableViewRowAction")]
unsafe impl NSObjectProtocol for NSTableViewRowAction {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTableViewRowAction")]
    unsafe impl NSTableViewRowAction {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other rowActionWithStyle:title:handler:)]
        pub unsafe fn rowActionWithStyle_title_handler(
            style: NSTableViewRowActionStyle,
            title: &NSString,
            handler: &Block<(NonNull<NSTableViewRowAction>, NSInteger), ()>,
        ) -> Id<Self>;

        #[method(style)]
        pub unsafe fn style(&self) -> NSTableViewRowActionStyle;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTableViewRowAction")]
    unsafe impl NSTableViewRowAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
