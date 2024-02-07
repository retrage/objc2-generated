//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSAlertStyle {
        NSAlertStyleWarning = 0,
        NSAlertStyleInformational = 1,
        NSAlertStyleCritical = 2,
    }
);

extern_static!(NSAlertFirstButtonReturn: NSModalResponse = 1000);

extern_static!(NSAlertSecondButtonReturn: NSModalResponse = 1001);

extern_static!(NSAlertThirdButtonReturn: NSModalResponse = 1002);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSAlert")]
    pub struct NSAlert;

    #[cfg(feature = "AppKit_NSAlert")]
    unsafe impl ClassType for NSAlert {
        type Super = NSObject;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSAlert")]
unsafe impl NSObjectProtocol for NSAlert {}

extern_methods!(
    #[cfg(feature = "AppKit_NSAlert")]
    unsafe impl NSAlert {
        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other alertWithError:)]
        pub unsafe fn alertWithError(error: &NSError, mtm: MainThreadMarker) -> Id<NSAlert>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other messageText)]
        pub unsafe fn messageText(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMessageText:)]
        pub unsafe fn setMessageText(&self, message_text: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other informativeText)]
        pub unsafe fn informativeText(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setInformativeText:)]
        pub unsafe fn setInformativeText(&self, informative_text: &NSString);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setIcon:)]
        pub unsafe fn setIcon(&self, icon: Option<&NSImage>);

        #[cfg(all(feature = "AppKit_NSButton", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other addButtonWithTitle:)]
        pub unsafe fn addButtonWithTitle(&self, title: &NSString) -> Id<NSButton>;

        #[cfg(all(feature = "AppKit_NSButton", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other buttons)]
        pub unsafe fn buttons(&self) -> Id<NSArray<NSButton>>;

        #[method(alertStyle)]
        pub unsafe fn alertStyle(&self) -> NSAlertStyle;

        #[method(setAlertStyle:)]
        pub unsafe fn setAlertStyle(&self, alert_style: NSAlertStyle);

        #[method(showsHelp)]
        pub unsafe fn showsHelp(&self) -> bool;

        #[method(setShowsHelp:)]
        pub unsafe fn setShowsHelp(&self, shows_help: bool);

        #[method_id(@__retain_semantics Other helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Id<NSHelpAnchorName>>;

        #[method(setHelpAnchor:)]
        pub unsafe fn setHelpAnchor(&self, help_anchor: Option<&NSHelpAnchorName>);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSAlertDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSAlertDelegate>>);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[method(layout)]
        pub unsafe fn layout(&self);

        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSModalResponse;

        #[method(showsSuppressionButton)]
        pub unsafe fn showsSuppressionButton(&self) -> bool;

        #[method(setShowsSuppressionButton:)]
        pub unsafe fn setShowsSuppressionButton(&self, shows_suppression_button: bool);

        #[cfg(feature = "AppKit_NSButton")]
        #[method_id(@__retain_semantics Other suppressionButton)]
        pub unsafe fn suppressionButton(&self) -> Option<Id<NSButton>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(beginSheetModalForWindow:completionHandler:)]
        pub unsafe fn beginSheetModalForWindow_completionHandler(
            &self,
            sheet_window: &NSWindow,
            handler: Option<&Block<dyn Fn(NSModalResponse)>>,
        );

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other window)]
        pub unsafe fn window(&self) -> Id<NSWindow>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSAlert")]
    unsafe impl NSAlert {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSAlertDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "AppKit_NSAlert")]
        #[optional]
        #[method(alertShowHelp:)]
        unsafe fn alertShowHelp(&self, alert: &NSAlert) -> bool;
    }

    unsafe impl ProtocolType for dyn NSAlertDelegate {}
);

extern_methods!(
    /// NSAlertDeprecated
    #[cfg(feature = "AppKit_NSAlert")]
    unsafe impl NSAlert {
        #[cfg(feature = "AppKit_NSWindow")]
        #[deprecated = "Use -beginSheetModalForWindow:completionHandler: instead"]
        #[method(beginSheetModalForWindow:modalDelegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetModalForWindow_modalDelegate_didEndSelector_contextInfo(
            &self,
            window: &NSWindow,
            delegate: Option<&AnyObject>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );
    }
);

extern_static!(NSWarningAlertStyle: NSAlertStyle = NSAlertStyle(NSAlertStyleWarning.0));

extern_static!(NSInformationalAlertStyle: NSAlertStyle = NSAlertStyle(NSAlertStyleInformational.0));

extern_static!(NSCriticalAlertStyle: NSAlertStyle = NSAlertStyle(NSAlertStyleCritical.0));
