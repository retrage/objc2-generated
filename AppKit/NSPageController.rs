//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSPageControllerObjectIdentifier = NSString;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPageControllerTransitionStyle {
        NSPageControllerTransitionStyleStackHistory = 0,
        NSPageControllerTransitionStyleStackBook = 1,
        NSPageControllerTransitionStyleHorizontalStrip = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPageController")]
    pub struct NSPageController;

    #[cfg(feature = "AppKit_NSPageController")]
    unsafe impl ClassType for NSPageController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSPageController")]
unsafe impl NSAnimatablePropertyContainer for NSPageController {}

#[cfg(feature = "AppKit_NSPageController")]
unsafe impl NSCoding for NSPageController {}

#[cfg(feature = "AppKit_NSPageController")]
unsafe impl NSEditor for NSPageController {}

#[cfg(feature = "AppKit_NSPageController")]
unsafe impl NSObjectProtocol for NSPageController {}

#[cfg(feature = "AppKit_NSPageController")]
unsafe impl NSSeguePerforming for NSPageController {}

#[cfg(feature = "AppKit_NSPageController")]
unsafe impl NSUserInterfaceItemIdentification for NSPageController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPageController")]
    unsafe impl NSPageController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSPageControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSPageControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Other selectedViewController)]
        pub unsafe fn selectedViewController(&self) -> Option<Id<NSViewController>>;

        #[method(transitionStyle)]
        pub unsafe fn transitionStyle(&self) -> NSPageControllerTransitionStyle;

        #[method(setTransitionStyle:)]
        pub unsafe fn setTransitionStyle(&self, transition_style: NSPageControllerTransitionStyle);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other arrangedObjects)]
        pub unsafe fn arrangedObjects(&self) -> Id<NSArray>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setArrangedObjects:)]
        pub unsafe fn setArrangedObjects(&self, arranged_objects: &NSArray);

        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSInteger);

        #[method(navigateForwardToObject:)]
        pub unsafe fn navigateForwardToObject(&self, object: &AnyObject);

        #[method(completeTransition)]
        pub unsafe fn completeTransition(&self);

        #[method(navigateBack:)]
        pub unsafe fn navigateBack(&self, sender: Option<&AnyObject>);

        #[method(navigateForward:)]
        pub unsafe fn navigateForward(&self, sender: Option<&AnyObject>);

        #[method(takeSelectedIndexFrom:)]
        pub unsafe fn takeSelectedIndexFrom(&self, sender: Option<&AnyObject>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "AppKit_NSPageController")]
    unsafe impl NSPageController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Allocated<Self>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSPageController")]
    unsafe impl NSPageController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSPageController")]
    unsafe impl NSPageController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSPageControllerDelegate: NSObjectProtocol + IsMainThreadOnly {
        #[cfg(feature = "AppKit_NSPageController")]
        #[optional]
        #[method_id(@__retain_semantics Other pageController:identifierForObject:)]
        unsafe fn pageController_identifierForObject(
            &self,
            page_controller: &NSPageController,
            object: &AnyObject,
        ) -> Id<NSPageControllerObjectIdentifier>;

        #[cfg(all(
            feature = "AppKit_NSPageController",
            feature = "AppKit_NSViewController"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other pageController:viewControllerForIdentifier:)]
        unsafe fn pageController_viewControllerForIdentifier(
            &self,
            page_controller: &NSPageController,
            identifier: &NSPageControllerObjectIdentifier,
        ) -> Id<NSViewController>;

        #[cfg(feature = "AppKit_NSPageController")]
        #[optional]
        #[method(pageController:frameForObject:)]
        unsafe fn pageController_frameForObject(
            &self,
            page_controller: &NSPageController,
            object: Option<&AnyObject>,
        ) -> NSRect;

        #[cfg(all(
            feature = "AppKit_NSPageController",
            feature = "AppKit_NSViewController"
        ))]
        #[optional]
        #[method(pageController:prepareViewController:withObject:)]
        unsafe fn pageController_prepareViewController_withObject(
            &self,
            page_controller: &NSPageController,
            view_controller: &NSViewController,
            object: Option<&AnyObject>,
        );

        #[cfg(feature = "AppKit_NSPageController")]
        #[optional]
        #[method(pageController:didTransitionToObject:)]
        unsafe fn pageController_didTransitionToObject(
            &self,
            page_controller: &NSPageController,
            object: &AnyObject,
        );

        #[cfg(feature = "AppKit_NSPageController")]
        #[optional]
        #[method(pageControllerWillStartLiveTransition:)]
        unsafe fn pageControllerWillStartLiveTransition(&self, page_controller: &NSPageController);

        #[cfg(feature = "AppKit_NSPageController")]
        #[optional]
        #[method(pageControllerDidEndLiveTransition:)]
        unsafe fn pageControllerDidEndLiveTransition(&self, page_controller: &NSPageController);
    }

    unsafe impl ProtocolType for dyn NSPageControllerDelegate {}
);
