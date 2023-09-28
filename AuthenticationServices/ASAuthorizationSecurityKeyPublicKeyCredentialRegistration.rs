//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration"
    )]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialRegistration;

    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration"
    )]
    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration")]
unsafe impl ASAuthorizationCredential
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration
{
}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration")]
unsafe impl ASAuthorizationPublicKeyCredentialRegistration
    for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration
{
}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration")]
unsafe impl ASPublicKeyCredential for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration")]
unsafe impl NSCoding for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration")]
unsafe impl NSCopying for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration")]
unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration")]
unsafe impl NSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration"
    )]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistration"
    )]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialRegistration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
