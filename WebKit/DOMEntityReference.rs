//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMEntityReference")]
    #[deprecated]
    pub struct DOMEntityReference;

    #[cfg(feature = "WebKit_DOMEntityReference")]
    unsafe impl ClassType for DOMEntityReference {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMNode;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMEntityReference")]
unsafe impl DOMEventTarget for DOMEntityReference {}

#[cfg(feature = "WebKit_DOMEntityReference")]
unsafe impl NSCopying for DOMEntityReference {}

#[cfg(feature = "WebKit_DOMEntityReference")]
unsafe impl NSObjectProtocol for DOMEntityReference {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMEntityReference")]
    unsafe impl DOMEntityReference {}
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMEntityReference")]
    unsafe impl DOMEntityReference {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMEntityReference")]
    unsafe impl DOMEntityReference {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
