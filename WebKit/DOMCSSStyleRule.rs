//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMCSSStyleRule")]
    #[deprecated]
    pub struct DOMCSSStyleRule;

    #[cfg(feature = "WebKit_DOMCSSStyleRule")]
    unsafe impl ClassType for DOMCSSStyleRule {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMCSSRule;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMCSSStyleRule")]
unsafe impl NSCopying for DOMCSSStyleRule {}

#[cfg(feature = "WebKit_DOMCSSStyleRule")]
unsafe impl NSObjectProtocol for DOMCSSStyleRule {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMCSSStyleRule")]
    unsafe impl DOMCSSStyleRule {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other selectorText)]
        pub unsafe fn selectorText(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setSelectorText:)]
        pub unsafe fn setSelectorText(&self, selector_text: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMCSSStyleDeclaration")]
        #[deprecated]
        #[method_id(@__retain_semantics Other style)]
        pub unsafe fn style(&self) -> Option<Id<DOMCSSStyleDeclaration>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMCSSStyleRule")]
    unsafe impl DOMCSSStyleRule {
        #[deprecated]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMCSSStyleRule")]
    unsafe impl DOMCSSStyleRule {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
