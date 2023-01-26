//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::InputMethodKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "InputMethodKit_IMKServer")]
    pub struct IMKServer;

    #[cfg(feature = "InputMethodKit_IMKServer")]
    unsafe impl ClassType for IMKServer {
        type Super = NSObject;
    }
);

#[cfg(feature = "InputMethodKit_IMKServer")]
unsafe impl NSObjectProtocol for IMKServer {}

extern_methods!(
    #[cfg(feature = "InputMethodKit_IMKServer")]
    unsafe impl IMKServer {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:bundleIdentifier:)]
        pub unsafe fn initWithName_bundleIdentifier(
            this: Option<Allocated<Self>>,
            name: Option<&NSString>,
            bundle_identifier: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:controllerClass:delegateClass:)]
        pub unsafe fn initWithName_controllerClass_delegateClass(
            this: Option<Allocated<Self>>,
            name: Option<&NSString>,
            controller_class_id: Option<&Class>,
            delegate_class_id: Option<&Class>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Other bundle)]
        pub unsafe fn bundle(&self) -> Option<Id<NSBundle, Shared>>;

        #[method(paletteWillTerminate)]
        pub unsafe fn paletteWillTerminate(&self) -> bool;

        #[method(lastKeyEventWasDeadKey)]
        pub unsafe fn lastKeyEventWasDeadKey(&self) -> bool;
    }
);
