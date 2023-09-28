//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

pub type CLGeocodeCompletionHandler = *mut Block<(*mut NSArray<CLPlacemark>, *mut NSError), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLGeocoder")]
    pub struct CLGeocoder;

    #[cfg(feature = "CoreLocation_CLGeocoder")]
    unsafe impl ClassType for CLGeocoder {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreLocation_CLGeocoder")]
unsafe impl NSObjectProtocol for CLGeocoder {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLGeocoder")]
    unsafe impl CLGeocoder {
        #[method(isGeocoding)]
        pub unsafe fn isGeocoding(&self) -> bool;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method(reverseGeocodeLocation:completionHandler:)]
        pub unsafe fn reverseGeocodeLocation_completionHandler(
            &self,
            location: &CLLocation,
            completion_handler: CLGeocodeCompletionHandler,
        );

        #[cfg(all(feature = "CoreLocation_CLLocation", feature = "Foundation_NSLocale"))]
        #[method(reverseGeocodeLocation:preferredLocale:completionHandler:)]
        pub unsafe fn reverseGeocodeLocation_preferredLocale_completionHandler(
            &self,
            location: &CLLocation,
            locale: Option<&NSLocale>,
            completion_handler: CLGeocodeCompletionHandler,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[deprecated = "Use -geocodePostalAddress:completionHandler:"]
        #[method(geocodeAddressDictionary:completionHandler:)]
        pub unsafe fn geocodeAddressDictionary_completionHandler(
            &self,
            address_dictionary: &NSDictionary,
            completion_handler: CLGeocodeCompletionHandler,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(geocodeAddressString:completionHandler:)]
        pub unsafe fn geocodeAddressString_completionHandler(
            &self,
            address_string: &NSString,
            completion_handler: CLGeocodeCompletionHandler,
        );

        #[cfg(all(feature = "CoreLocation_CLRegion", feature = "Foundation_NSString"))]
        #[method(geocodeAddressString:inRegion:completionHandler:)]
        pub unsafe fn geocodeAddressString_inRegion_completionHandler(
            &self,
            address_string: &NSString,
            region: Option<&CLRegion>,
            completion_handler: CLGeocodeCompletionHandler,
        );

        #[cfg(all(
            feature = "CoreLocation_CLRegion",
            feature = "Foundation_NSLocale",
            feature = "Foundation_NSString"
        ))]
        #[method(geocodeAddressString:inRegion:preferredLocale:completionHandler:)]
        pub unsafe fn geocodeAddressString_inRegion_preferredLocale_completionHandler(
            &self,
            address_string: &NSString,
            region: Option<&CLRegion>,
            locale: Option<&NSLocale>,
            completion_handler: CLGeocodeCompletionHandler,
        );

        #[method(cancelGeocode)]
        pub unsafe fn cancelGeocode(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreLocation_CLGeocoder")]
    unsafe impl CLGeocoder {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// ContactsAdditions
    #[cfg(feature = "CoreLocation_CLGeocoder")]
    unsafe impl CLGeocoder {
        #[cfg(feature = "Contacts_CNPostalAddress")]
        #[method(geocodePostalAddress:completionHandler:)]
        pub unsafe fn geocodePostalAddress_completionHandler(
            &self,
            postal_address: &CNPostalAddress,
            completion_handler: CLGeocodeCompletionHandler,
        );

        #[cfg(all(feature = "Contacts_CNPostalAddress", feature = "Foundation_NSLocale"))]
        #[method(geocodePostalAddress:preferredLocale:completionHandler:)]
        pub unsafe fn geocodePostalAddress_preferredLocale_completionHandler(
            &self,
            postal_address: &CNPostalAddress,
            locale: Option<&NSLocale>,
            completion_handler: CLGeocodeCompletionHandler,
        );
    }
);
