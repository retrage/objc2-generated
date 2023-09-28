//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNMutablePostalAddress")]
    pub struct CNMutablePostalAddress;

    #[cfg(feature = "Contacts_CNMutablePostalAddress")]
    unsafe impl ClassType for CNMutablePostalAddress {
        #[inherits(NSObject)]
        type Super = CNPostalAddress;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Contacts_CNMutablePostalAddress")]
unsafe impl NSCoding for CNMutablePostalAddress {}

#[cfg(feature = "Contacts_CNMutablePostalAddress")]
unsafe impl NSCopying for CNMutablePostalAddress {}

#[cfg(feature = "Contacts_CNMutablePostalAddress")]
unsafe impl NSMutableCopying for CNMutablePostalAddress {}

#[cfg(feature = "Contacts_CNMutablePostalAddress")]
unsafe impl NSObjectProtocol for CNMutablePostalAddress {}

#[cfg(feature = "Contacts_CNMutablePostalAddress")]
unsafe impl NSSecureCoding for CNMutablePostalAddress {}

extern_methods!(
    #[cfg(feature = "Contacts_CNMutablePostalAddress")]
    unsafe impl CNMutablePostalAddress {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other street)]
        pub unsafe fn street(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setStreet:)]
        pub unsafe fn setStreet(&self, street: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subLocality)]
        pub unsafe fn subLocality(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubLocality:)]
        pub unsafe fn setSubLocality(&self, sub_locality: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other city)]
        pub unsafe fn city(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCity:)]
        pub unsafe fn setCity(&self, city: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subAdministrativeArea)]
        pub unsafe fn subAdministrativeArea(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubAdministrativeArea:)]
        pub unsafe fn setSubAdministrativeArea(&self, sub_administrative_area: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setState:)]
        pub unsafe fn setState(&self, state: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other postalCode)]
        pub unsafe fn postalCode(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPostalCode:)]
        pub unsafe fn setPostalCode(&self, postal_code: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other country)]
        pub unsafe fn country(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCountry:)]
        pub unsafe fn setCountry(&self, country: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ISOCountryCode)]
        pub unsafe fn ISOCountryCode(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setISOCountryCode:)]
        pub unsafe fn setISOCountryCode(&self, iso_country_code: &NSString);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Contacts_CNMutablePostalAddress")]
    unsafe impl CNMutablePostalAddress {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
