//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSColorPanelMode {
        NSColorPanelModeNone = -1,
        NSColorPanelModeGray = 0,
        NSColorPanelModeRGB = 1,
        NSColorPanelModeCMYK = 2,
        NSColorPanelModeHSB = 3,
        NSColorPanelModeCustomPalette = 4,
        NSColorPanelModeColorList = 5,
        NSColorPanelModeWheel = 6,
        NSColorPanelModeCrayon = 7,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSColorPanelOptions {
        NSColorPanelGrayModeMask = 0x00000001,
        NSColorPanelRGBModeMask = 0x00000002,
        NSColorPanelCMYKModeMask = 0x00000004,
        NSColorPanelHSBModeMask = 0x00000008,
        NSColorPanelCustomPaletteModeMask = 0x00000010,
        NSColorPanelColorListModeMask = 0x00000020,
        NSColorPanelWheelModeMask = 0x00000040,
        NSColorPanelCrayonModeMask = 0x00000080,
        NSColorPanelAllModesMask = 0x0000ffff,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSColorPanel")]
    pub struct NSColorPanel;

    #[cfg(feature = "AppKit_NSColorPanel")]
    unsafe impl ClassType for NSColorPanel {
        #[inherits(NSWindow, NSResponder, NSObject)]
        type Super = NSPanel;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSColorPanel")]
unsafe impl NSAccessibility for NSColorPanel {}

#[cfg(feature = "AppKit_NSColorPanel")]
unsafe impl NSAccessibilityElementProtocol for NSColorPanel {}

#[cfg(feature = "AppKit_NSColorPanel")]
unsafe impl NSAnimatablePropertyContainer for NSColorPanel {}

#[cfg(feature = "AppKit_NSColorPanel")]
unsafe impl NSAppearanceCustomization for NSColorPanel {}

#[cfg(feature = "AppKit_NSColorPanel")]
unsafe impl NSCoding for NSColorPanel {}

#[cfg(feature = "AppKit_NSColorPanel")]
unsafe impl NSMenuItemValidation for NSColorPanel {}

#[cfg(feature = "AppKit_NSColorPanel")]
unsafe impl NSObjectProtocol for NSColorPanel {}

#[cfg(feature = "AppKit_NSColorPanel")]
unsafe impl NSUserInterfaceItemIdentification for NSColorPanel {}

#[cfg(feature = "AppKit_NSColorPanel")]
unsafe impl NSUserInterfaceValidations for NSColorPanel {}

extern_methods!(
    #[cfg(feature = "AppKit_NSColorPanel")]
    unsafe impl NSColorPanel {
        #[method_id(@__retain_semantics Other sharedColorPanel)]
        pub unsafe fn sharedColorPanel(mtm: MainThreadMarker) -> Id<NSColorPanel>;

        #[method(sharedColorPanelExists)]
        pub unsafe fn sharedColorPanelExists(mtm: MainThreadMarker) -> bool;

        #[cfg(all(
            feature = "AppKit_NSColor",
            feature = "AppKit_NSEvent",
            feature = "AppKit_NSView"
        ))]
        #[method(dragColor:withEvent:fromView:)]
        pub unsafe fn dragColor_withEvent_fromView(
            color: &NSColor,
            event: &NSEvent,
            source_view: &NSView,
        ) -> bool;

        #[method(setPickerMask:)]
        pub unsafe fn setPickerMask(mask: NSColorPanelOptions, mtm: MainThreadMarker);

        #[method(setPickerMode:)]
        pub unsafe fn setPickerMode(mode: NSColorPanelMode, mtm: MainThreadMarker);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[method(showsAlpha)]
        pub unsafe fn showsAlpha(&self) -> bool;

        #[method(setShowsAlpha:)]
        pub unsafe fn setShowsAlpha(&self, shows_alpha: bool);

        #[method(mode)]
        pub unsafe fn mode(&self) -> NSColorPanelMode;

        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSColorPanelMode);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);

        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, selector: Option<Sel>);

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&AnyObject>);

        #[cfg(feature = "AppKit_NSColorList")]
        #[method(attachColorList:)]
        pub unsafe fn attachColorList(&self, color_list: &NSColorList);

        #[cfg(feature = "AppKit_NSColorList")]
        #[method(detachColorList:)]
        pub unsafe fn detachColorList(&self, color_list: &NSColorList);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(feature = "AppKit_NSColorPanel")]
    unsafe impl NSColorPanel {
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            this: Allocated<Self>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSScreen")]
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            this: Allocated<Self>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
            screen: Option<&NSScreen>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            content_view_controller: &NSViewController,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSColorPanel")]
    unsafe impl NSColorPanel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSColorPanel")]
    unsafe impl NSColorPanel {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_methods!(
    /// NSColorPanel
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(orderFrontColorPanel:)]
        pub unsafe fn orderFrontColorPanel(&self, sender: Option<&AnyObject>);
    }
);

extern_protocol!(
    pub unsafe trait NSColorChanging: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSColorPanel")]
        #[method(changeColor:)]
        unsafe fn changeColor(&self, sender: Option<&NSColorPanel>);
    }

    unsafe impl ProtocolType for dyn NSColorChanging {}
);

extern_static!(NSColorPanelColorDidChangeNotification: &'static NSNotificationName);

extern_static!(NSNoModeColorPanel: NSColorPanelMode = NSColorPanelModeNone);

extern_static!(NSGrayModeColorPanel: NSColorPanelMode = NSColorPanelModeGray);

extern_static!(NSRGBModeColorPanel: NSColorPanelMode = NSColorPanelModeRGB);

extern_static!(NSCMYKModeColorPanel: NSColorPanelMode = NSColorPanelModeCMYK);

extern_static!(NSHSBModeColorPanel: NSColorPanelMode = NSColorPanelModeHSB);

extern_static!(NSCustomPaletteModeColorPanel: NSColorPanelMode = NSColorPanelModeCustomPalette);

extern_static!(NSColorListModeColorPanel: NSColorPanelMode = NSColorPanelModeColorList);

extern_static!(NSWheelModeColorPanel: NSColorPanelMode = NSColorPanelModeWheel);

extern_static!(NSCrayonModeColorPanel: NSColorPanelMode = NSColorPanelModeCrayon);
