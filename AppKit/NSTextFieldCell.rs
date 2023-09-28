//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTextFieldBezelStyle {
        NSTextFieldSquareBezel = 0,
        NSTextFieldRoundedBezel = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextFieldCell")]
    pub struct NSTextFieldCell;

    #[cfg(feature = "AppKit_NSTextFieldCell")]
    unsafe impl ClassType for NSTextFieldCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSTextFieldCell")]
unsafe impl NSAccessibility for NSTextFieldCell {}

#[cfg(feature = "AppKit_NSTextFieldCell")]
unsafe impl NSAccessibilityElementProtocol for NSTextFieldCell {}

#[cfg(feature = "AppKit_NSTextFieldCell")]
unsafe impl NSCoding for NSTextFieldCell {}

#[cfg(feature = "AppKit_NSTextFieldCell")]
unsafe impl NSCopying for NSTextFieldCell {}

#[cfg(feature = "AppKit_NSTextFieldCell")]
unsafe impl NSObjectProtocol for NSTextFieldCell {}

#[cfg(feature = "AppKit_NSTextFieldCell")]
unsafe impl NSUserInterfaceItemIdentification for NSTextFieldCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextFieldCell")]
    unsafe impl NSTextFieldCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Allocated<Self>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(this: Allocated<Self>, image: Option<&NSImage>) -> Id<Self>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, text_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSText")]
        #[method_id(@__retain_semantics Other setUpFieldEditorAttributes:)]
        pub unsafe fn setUpFieldEditorAttributes(&self, text_obj: &NSText) -> Id<NSText>;

        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSTextFieldBezelStyle;

        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezel_style: NSTextFieldBezelStyle);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholder_string: Option<&NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholder_attributed_string: Option<&NSAttributedString>,
        );

        #[method(setWantsNotificationForMarkedText:)]
        pub unsafe fn setWantsNotificationForMarkedText(&self, flag: bool);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other allowedInputSourceLocales)]
        pub unsafe fn allowedInputSourceLocales(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setAllowedInputSourceLocales:)]
        pub unsafe fn setAllowedInputSourceLocales(
            &self,
            allowed_input_source_locales: Option<&NSArray<NSString>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSTextFieldCell")]
    unsafe impl NSTextFieldCell {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTextFieldCell")]
    unsafe impl NSTextFieldCell {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
