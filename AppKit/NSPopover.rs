//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPopoverAppearance {
        NSPopoverAppearanceMinimal = 0,
        NSPopoverAppearanceHUD = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPopoverBehavior {
        NSPopoverBehaviorApplicationDefined = 0,
        NSPopoverBehaviorTransient = 1,
        NSPopoverBehaviorSemitransient = 2,
    }
);

extern_methods!(
    unsafe impl NSPopover {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSPopoverDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSPopoverDelegate>);

        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSPopoverBehavior;

        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: NSPopoverBehavior);

        #[method(animates)]
        pub unsafe fn animates(&self) -> bool;

        #[method(setAnimates:)]
        pub unsafe fn setAnimates(&self, animates: bool);

        #[method_id(@__retain_semantics Other contentViewController)]
        pub unsafe fn contentViewController(&self) -> Option<Id<NSViewController, Shared>>;

        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(
            &self,
            contentViewController: Option<&NSViewController>,
        );

        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;

        #[method(setContentSize:)]
        pub unsafe fn setContentSize(&self, contentSize: NSSize);

        #[method(isShown)]
        pub unsafe fn isShown(&self) -> bool;

        #[method(isDetached)]
        pub unsafe fn isDetached(&self) -> bool;

        #[method(positioningRect)]
        pub unsafe fn positioningRect(&self) -> NSRect;

        #[method(setPositioningRect:)]
        pub unsafe fn setPositioningRect(&self, positioningRect: NSRect);

        #[method(showRelativeToRect:ofView:preferredEdge:)]
        pub unsafe fn showRelativeToRect_ofView_preferredEdge(
            &self,
            positioningRect: NSRect,
            positioningView: &NSView,
            preferredEdge: NSRectEdge,
        );

        #[method(performClose:)]
        pub unsafe fn performClose(&self, sender: Option<&Object>);

        #[method(close)]
        pub unsafe fn close(&self);
    }
);

extern_static!(NSPopoverCloseReasonKey: &'static NSString);

pub type NSPopoverCloseReasonValue = NSString;

extern_static!(NSPopoverCloseReasonStandard: &'static NSPopoverCloseReasonValue);

extern_static!(NSPopoverCloseReasonDetachToWindow: &'static NSPopoverCloseReasonValue);

extern_static!(NSPopoverWillShowNotification: &'static NSNotificationName);

extern_static!(NSPopoverDidShowNotification: &'static NSNotificationName);

extern_static!(NSPopoverWillCloseNotification: &'static NSNotificationName);

extern_static!(NSPopoverDidCloseNotification: &'static NSNotificationName);

extern_protocol!(
    pub struct NSPopoverDelegate;

    unsafe impl ProtocolType for NSPopoverDelegate {
        #[optional]
        #[method(popoverShouldClose:)]
        pub unsafe fn popoverShouldClose(&self, popover: &NSPopover) -> bool;

        #[optional]
        #[method(popoverShouldDetach:)]
        pub unsafe fn popoverShouldDetach(&self, popover: &NSPopover) -> bool;

        #[optional]
        #[method(popoverDidDetach:)]
        pub unsafe fn popoverDidDetach(&self, popover: &NSPopover);

        #[optional]
        #[method_id(@__retain_semantics Other detachableWindowForPopover:)]
        pub unsafe fn detachableWindowForPopover(
            &self,
            popover: &NSPopover,
        ) -> Option<Id<NSWindow, Shared>>;

        #[optional]
        #[method(popoverWillShow:)]
        pub unsafe fn popoverWillShow(&self, notification: &NSNotification);

        #[optional]
        #[method(popoverDidShow:)]
        pub unsafe fn popoverDidShow(&self, notification: &NSNotification);

        #[optional]
        #[method(popoverWillClose:)]
        pub unsafe fn popoverWillClose(&self, notification: &NSNotification);

        #[optional]
        #[method(popoverDidClose:)]
        pub unsafe fn popoverDidClose(&self, notification: &NSNotification);
    }
);
