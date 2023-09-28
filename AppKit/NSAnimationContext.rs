//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSAnimationContext")]
    pub struct NSAnimationContext;

    #[cfg(feature = "AppKit_NSAnimationContext")]
    unsafe impl ClassType for NSAnimationContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSAnimationContext")]
unsafe impl NSObjectProtocol for NSAnimationContext {}

extern_methods!(
    #[cfg(feature = "AppKit_NSAnimationContext")]
    unsafe impl NSAnimationContext {
        #[method(runAnimationGroup:completionHandler:)]
        pub unsafe fn runAnimationGroup_completionHandler(
            changes: &Block<(NonNull<NSAnimationContext>,), ()>,
            completion_handler: Option<&Block<(), ()>>,
        );

        #[method(runAnimationGroup:)]
        pub unsafe fn runAnimationGroup(changes: &Block<(NonNull<NSAnimationContext>,), ()>);

        #[method(beginGrouping)]
        pub unsafe fn beginGrouping();

        #[method(endGrouping)]
        pub unsafe fn endGrouping();

        #[method_id(@__retain_semantics Other currentContext)]
        pub unsafe fn currentContext() -> Id<NSAnimationContext>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(setDuration:)]
        pub unsafe fn setDuration(&self, duration: NSTimeInterval);

        #[method(completionHandler)]
        pub unsafe fn completionHandler(&self) -> *mut Block<(), ()>;

        #[method(setCompletionHandler:)]
        pub unsafe fn setCompletionHandler(&self, completion_handler: Option<&Block<(), ()>>);

        #[method(allowsImplicitAnimation)]
        pub unsafe fn allowsImplicitAnimation(&self) -> bool;

        #[method(setAllowsImplicitAnimation:)]
        pub unsafe fn setAllowsImplicitAnimation(&self, allows_implicit_animation: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSAnimationContext")]
    unsafe impl NSAnimationContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
