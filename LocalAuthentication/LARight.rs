//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum LARightState {
        LARightStateUnknown = 0,
        LARightStateAuthorizing = 1,
        LARightStateAuthorized = 2,
        LARightStateNotAuthorized = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LocalAuthentication_LARight")]
    pub struct LARight;

    #[cfg(feature = "LocalAuthentication_LARight")]
    unsafe impl ClassType for LARight {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "LocalAuthentication_LARight")]
unsafe impl NSObjectProtocol for LARight {}

extern_methods!(
    #[cfg(feature = "LocalAuthentication_LARight")]
    unsafe impl LARight {
        #[method(state)]
        pub unsafe fn state(&self) -> LARightState;

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "LocalAuthentication_LAAuthenticationRequirement")]
        #[method_id(@__retain_semantics Init initWithRequirement:)]
        pub unsafe fn initWithRequirement(
            this: Allocated<Self>,
            requirement: &LAAuthenticationRequirement,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(authorizeWithLocalizedReason:completion:)]
        pub unsafe fn authorizeWithLocalizedReason_completion(
            &self,
            localized_reason: &NSString,
            handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(checkCanAuthorizeWithCompletion:)]
        pub unsafe fn checkCanAuthorizeWithCompletion(&self, handler: &Block<(*mut NSError,), ()>);

        #[method(deauthorizeWithCompletion:)]
        pub unsafe fn deauthorizeWithCompletion(&self, handler: &Block<(), ()>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "LocalAuthentication_LARight")]
    unsafe impl LARight {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
