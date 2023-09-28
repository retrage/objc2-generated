//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMDocumentType")]
    #[deprecated]
    pub struct DOMDocumentType;

    #[cfg(feature = "WebKit_DOMDocumentType")]
    unsafe impl ClassType for DOMDocumentType {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMNode;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMDocumentType")]
unsafe impl DOMEventTarget for DOMDocumentType {}

#[cfg(feature = "WebKit_DOMDocumentType")]
unsafe impl NSCopying for DOMDocumentType {}

#[cfg(feature = "WebKit_DOMDocumentType")]
unsafe impl NSObjectProtocol for DOMDocumentType {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMDocumentType")]
    unsafe impl DOMDocumentType {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "WebKit_DOMNamedNodeMap")]
        #[deprecated]
        #[method_id(@__retain_semantics Other entities)]
        pub unsafe fn entities(&self) -> Option<Id<DOMNamedNodeMap>>;

        #[cfg(feature = "WebKit_DOMNamedNodeMap")]
        #[deprecated]
        #[method_id(@__retain_semantics Other notations)]
        pub unsafe fn notations(&self) -> Option<Id<DOMNamedNodeMap>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other publicId)]
        pub unsafe fn publicId(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other systemId)]
        pub unsafe fn systemId(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other internalSubset)]
        pub unsafe fn internalSubset(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMDocumentType")]
    unsafe impl DOMDocumentType {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMDocumentType")]
    unsafe impl DOMDocumentType {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
