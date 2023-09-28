//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNSocialProfile")]
    pub struct CNSocialProfile;

    #[cfg(feature = "Contacts_CNSocialProfile")]
    unsafe impl ClassType for CNSocialProfile {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Contacts_CNSocialProfile")]
unsafe impl NSCoding for CNSocialProfile {}

#[cfg(feature = "Contacts_CNSocialProfile")]
unsafe impl NSCopying for CNSocialProfile {}

#[cfg(feature = "Contacts_CNSocialProfile")]
unsafe impl NSObjectProtocol for CNSocialProfile {}

#[cfg(feature = "Contacts_CNSocialProfile")]
unsafe impl NSSecureCoding for CNSocialProfile {}

extern_methods!(
    #[cfg(feature = "Contacts_CNSocialProfile")]
    unsafe impl CNSocialProfile {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other urlString)]
        pub unsafe fn urlString(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other username)]
        pub unsafe fn username(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other userIdentifier)]
        pub unsafe fn userIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other service)]
        pub unsafe fn service(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithUrlString:username:userIdentifier:service:)]
        pub unsafe fn initWithUrlString_username_userIdentifier_service(
            this: Allocated<Self>,
            url_string: Option<&NSString>,
            username: Option<&NSString>,
            user_identifier: Option<&NSString>,
            service: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForKey:)]
        pub unsafe fn localizedStringForKey(key: &NSString) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedStringForService:)]
        pub unsafe fn localizedStringForService(service: &NSString) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Contacts_CNSocialProfile")]
    unsafe impl CNSocialProfile {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(CNSocialProfileURLStringKey: &'static NSString);

extern_static!(CNSocialProfileUsernameKey: &'static NSString);

extern_static!(CNSocialProfileUserIdentifierKey: &'static NSString);

extern_static!(CNSocialProfileServiceKey: &'static NSString);

extern_static!(CNSocialProfileServiceFacebook: &'static NSString);

extern_static!(CNSocialProfileServiceFlickr: &'static NSString);

extern_static!(CNSocialProfileServiceLinkedIn: &'static NSString);

extern_static!(CNSocialProfileServiceMySpace: &'static NSString);

extern_static!(CNSocialProfileServiceSinaWeibo: &'static NSString);

extern_static!(CNSocialProfileServiceTencentWeibo: &'static NSString);

extern_static!(CNSocialProfileServiceTwitter: &'static NSString);

extern_static!(CNSocialProfileServiceYelp: &'static NSString);

extern_static!(CNSocialProfileServiceGameCenter: &'static NSString);
