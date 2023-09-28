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
    #[cfg(feature = "MapKit_MKPolygonRenderer")]
    pub struct MKPolygonRenderer;

    #[cfg(feature = "MapKit_MKPolygonRenderer")]
    unsafe impl ClassType for MKPolygonRenderer {
        #[inherits(MKOverlayRenderer, NSObject)]
        type Super = MKOverlayPathRenderer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKPolygonRenderer")]
unsafe impl NSObjectProtocol for MKPolygonRenderer {}

extern_methods!(
    #[cfg(feature = "MapKit_MKPolygonRenderer")]
    unsafe impl MKPolygonRenderer {
        #[cfg(feature = "MapKit_MKPolygon")]
        #[method_id(@__retain_semantics Init initWithPolygon:)]
        pub unsafe fn initWithPolygon(this: Allocated<Self>, polygon: &MKPolygon) -> Id<Self>;

        #[cfg(feature = "MapKit_MKPolygon")]
        #[method_id(@__retain_semantics Other polygon)]
        pub unsafe fn polygon(&self) -> Id<MKPolygon>;

        #[method(strokeStart)]
        pub unsafe fn strokeStart(&self) -> CGFloat;

        #[method(setStrokeStart:)]
        pub unsafe fn setStrokeStart(&self, stroke_start: CGFloat);

        #[method(strokeEnd)]
        pub unsafe fn strokeEnd(&self) -> CGFloat;

        #[method(setStrokeEnd:)]
        pub unsafe fn setStrokeEnd(&self, stroke_end: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(feature = "MapKit_MKPolygonRenderer")]
    unsafe impl MKPolygonRenderer {
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Allocated<Self>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKPolygonRenderer")]
    unsafe impl MKPolygonRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
