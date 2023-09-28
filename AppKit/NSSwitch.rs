//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSwitch")]
    pub struct NSSwitch;

    #[cfg(feature = "AppKit_NSSwitch")]
    unsafe impl ClassType for NSSwitch {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSSwitch")]
unsafe impl NSAccessibility for NSSwitch {}

#[cfg(feature = "AppKit_NSSwitch")]
unsafe impl NSAccessibilityButton for NSSwitch {}

#[cfg(feature = "AppKit_NSSwitch")]
unsafe impl NSAccessibilityElementProtocol for NSSwitch {}

#[cfg(feature = "AppKit_NSSwitch")]
unsafe impl NSAccessibilitySwitch for NSSwitch {}

#[cfg(feature = "AppKit_NSSwitch")]
unsafe impl NSAnimatablePropertyContainer for NSSwitch {}

#[cfg(feature = "AppKit_NSSwitch")]
unsafe impl NSAppearanceCustomization for NSSwitch {}

#[cfg(feature = "AppKit_NSSwitch")]
unsafe impl NSCoding for NSSwitch {}

#[cfg(feature = "AppKit_NSSwitch")]
unsafe impl NSDraggingDestination for NSSwitch {}

#[cfg(feature = "AppKit_NSSwitch")]
unsafe impl NSObjectProtocol for NSSwitch {}

#[cfg(feature = "AppKit_NSSwitch")]
unsafe impl NSUserInterfaceItemIdentification for NSSwitch {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSwitch")]
    unsafe impl NSSwitch {
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSSwitch")]
    unsafe impl NSSwitch {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Allocated<Self>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSSwitch")]
    unsafe impl NSSwitch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSSwitch")]
    unsafe impl NSSwitch {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);
