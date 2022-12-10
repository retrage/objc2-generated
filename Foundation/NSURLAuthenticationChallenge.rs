//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct NSURLAuthenticationChallengeSender;

    unsafe impl ProtocolType for NSURLAuthenticationChallengeSender {
        #[method(useCredential:forAuthenticationChallenge:)]
        pub unsafe fn useCredential_forAuthenticationChallenge(
            &self,
            credential: &NSURLCredential,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[method(continueWithoutCredentialForAuthenticationChallenge:)]
        pub unsafe fn continueWithoutCredentialForAuthenticationChallenge(
            &self,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[method(cancelAuthenticationChallenge:)]
        pub unsafe fn cancelAuthenticationChallenge(
            &self,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[optional]
        #[method(performDefaultHandlingForAuthenticationChallenge:)]
        pub unsafe fn performDefaultHandlingForAuthenticationChallenge(
            &self,
            challenge: &NSURLAuthenticationChallenge,
        );

        #[optional]
        #[method(rejectProtectionSpaceAndContinueWithChallenge:)]
        pub unsafe fn rejectProtectionSpaceAndContinueWithChallenge(
            &self,
            challenge: &NSURLAuthenticationChallenge,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSURLAuthenticationChallenge;

    unsafe impl ClassType for NSURLAuthenticationChallenge {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSURLAuthenticationChallenge {
        #[method_id(@__retain_semantics Init initWithProtectionSpace:proposedCredential:previousFailureCount:failureResponse:error:sender:)]
        pub unsafe fn initWithProtectionSpace_proposedCredential_previousFailureCount_failureResponse_error_sender(
            this: Option<Allocated<Self>>,
            space: &NSURLProtectionSpace,
            credential: Option<&NSURLCredential>,
            previousFailureCount: NSInteger,
            response: Option<&NSURLResponse>,
            error: Option<&NSError>,
            sender: &NSURLAuthenticationChallengeSender,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithAuthenticationChallenge:sender:)]
        pub unsafe fn initWithAuthenticationChallenge_sender(
            this: Option<Allocated<Self>>,
            challenge: &NSURLAuthenticationChallenge,
            sender: &NSURLAuthenticationChallengeSender,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other protectionSpace)]
        pub unsafe fn protectionSpace(&self) -> Id<NSURLProtectionSpace, Shared>;

        #[method_id(@__retain_semantics Other proposedCredential)]
        pub unsafe fn proposedCredential(&self) -> Option<Id<NSURLCredential, Shared>>;

        #[method(previousFailureCount)]
        pub unsafe fn previousFailureCount(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other failureResponse)]
        pub unsafe fn failureResponse(&self) -> Option<Id<NSURLResponse, Shared>>;

        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Other sender)]
        pub unsafe fn sender(&self) -> Option<Id<NSURLAuthenticationChallengeSender, Shared>>;
    }
);
