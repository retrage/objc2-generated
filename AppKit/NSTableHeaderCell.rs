//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    pub struct NSTableHeaderCell;

    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    unsafe impl ClassType for NSTableHeaderCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSAccessibility for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSAccessibilityElementProtocol for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSCoding for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSCopying for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSObjectProtocol for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSUserInterfaceItemIdentification for NSTableHeaderCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    unsafe impl NSTableHeaderCell {
        #[cfg(feature = "AppKit_NSView")]
        #[method(drawSortIndicatorWithFrame:inView:ascending:priority:)]
        pub unsafe fn drawSortIndicatorWithFrame_inView_ascending_priority(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
            ascending: bool,
            priority: NSInteger,
        );

        #[method(sortIndicatorRectForBounds:)]
        pub unsafe fn sortIndicatorRectForBounds(&self, rect: NSRect) -> NSRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    unsafe impl NSTableHeaderCell {
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
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    unsafe impl NSTableHeaderCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    unsafe impl NSTableHeaderCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
