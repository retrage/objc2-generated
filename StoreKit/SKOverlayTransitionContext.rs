//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKOverlayTransitionContext")]
    pub struct SKOverlayTransitionContext;

    #[cfg(feature = "StoreKit_SKOverlayTransitionContext")]
    unsafe impl ClassType for SKOverlayTransitionContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKOverlayTransitionContext")]
unsafe impl NSObjectProtocol for SKOverlayTransitionContext {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKOverlayTransitionContext")]
    unsafe impl SKOverlayTransitionContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(addAnimationBlock:)]
        pub unsafe fn addAnimationBlock(&self, block: &Block<(), ()>);

        #[method(startFrame)]
        pub unsafe fn startFrame(&self) -> CGRect;

        #[method(endFrame)]
        pub unsafe fn endFrame(&self) -> CGRect;
    }
);
