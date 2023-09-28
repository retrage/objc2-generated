//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKUserLocation")]
    pub struct MKUserLocation;

    #[cfg(feature = "MapKit_MKUserLocation")]
    unsafe impl ClassType for MKUserLocation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKUserLocation")]
unsafe impl MKAnnotation for MKUserLocation {}

#[cfg(feature = "MapKit_MKUserLocation")]
unsafe impl NSObjectProtocol for MKUserLocation {}

extern_methods!(
    #[cfg(feature = "MapKit_MKUserLocation")]
    unsafe impl MKUserLocation {
        #[method(isUpdating)]
        pub unsafe fn isUpdating(&self) -> bool;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Id<CLLocation>>;

        #[cfg(feature = "CoreLocation_CLHeading")]
        #[method_id(@__retain_semantics Other heading)]
        pub unsafe fn heading(&self) -> Option<Id<CLHeading>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKUserLocation")]
    unsafe impl MKUserLocation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
