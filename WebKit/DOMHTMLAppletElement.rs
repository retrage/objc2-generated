//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLAppletElement")]
    #[deprecated]
    pub struct DOMHTMLAppletElement;

    #[cfg(feature = "WebKit_DOMHTMLAppletElement")]
    unsafe impl ClassType for DOMHTMLAppletElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLAppletElement")]
unsafe impl DOMEventTarget for DOMHTMLAppletElement {}

#[cfg(feature = "WebKit_DOMHTMLAppletElement")]
unsafe impl NSCopying for DOMHTMLAppletElement {}

#[cfg(feature = "WebKit_DOMHTMLAppletElement")]
unsafe impl NSObjectProtocol for DOMHTMLAppletElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLAppletElement")]
    unsafe impl DOMHTMLAppletElement {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other alt)]
        pub unsafe fn alt(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setAlt:)]
        pub unsafe fn setAlt(&self, alt: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other archive)]
        pub unsafe fn archive(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setArchive:)]
        pub unsafe fn setArchive(&self, archive: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other code)]
        pub unsafe fn code(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setCode:)]
        pub unsafe fn setCode(&self, code: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other codeBase)]
        pub unsafe fn codeBase(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setCodeBase:)]
        pub unsafe fn setCodeBase(&self, code_base: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other height)]
        pub unsafe fn height(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: Option<&NSString>);

        #[deprecated]
        #[method(hspace)]
        pub unsafe fn hspace(&self) -> c_int;

        #[deprecated]
        #[method(setHspace:)]
        pub unsafe fn setHspace(&self, hspace: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other object)]
        pub unsafe fn object(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setObject:)]
        pub unsafe fn setObject(&self, object: Option<&NSString>);

        #[deprecated]
        #[method(vspace)]
        pub unsafe fn vspace(&self) -> c_int;

        #[deprecated]
        #[method(setVspace:)]
        pub unsafe fn setVspace(&self, vspace: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other width)]
        pub unsafe fn width(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLAppletElement")]
    unsafe impl DOMHTMLAppletElement {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLAppletElement")]
    unsafe impl DOMHTMLAppletElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
