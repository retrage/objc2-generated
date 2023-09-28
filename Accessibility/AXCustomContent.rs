//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Accessibility::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum AXCustomContentImportance {
        AXCustomContentImportanceDefault = 0,
        AXCustomContentImportanceHigh = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Accessibility_AXCustomContent")]
    pub struct AXCustomContent;

    #[cfg(feature = "Accessibility_AXCustomContent")]
    unsafe impl ClassType for AXCustomContent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Accessibility_AXCustomContent")]
unsafe impl NSCoding for AXCustomContent {}

#[cfg(feature = "Accessibility_AXCustomContent")]
unsafe impl NSCopying for AXCustomContent {}

#[cfg(feature = "Accessibility_AXCustomContent")]
unsafe impl NSObjectProtocol for AXCustomContent {}

#[cfg(feature = "Accessibility_AXCustomContent")]
unsafe impl NSSecureCoding for AXCustomContent {}

extern_methods!(
    #[cfg(feature = "Accessibility_AXCustomContent")]
    unsafe impl AXCustomContent {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customContentWithLabel:value:)]
        pub unsafe fn customContentWithLabel_value(label: &NSString, value: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other customContentWithAttributedLabel:attributedValue:)]
        pub unsafe fn customContentWithAttributedLabel_attributedValue(
            label: &NSAttributedString,
            value: &NSAttributedString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedLabel)]
        pub unsafe fn attributedLabel(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedValue)]
        pub unsafe fn attributedValue(&self) -> Id<NSAttributedString>;

        #[method(importance)]
        pub unsafe fn importance(&self) -> AXCustomContentImportance;

        #[method(setImportance:)]
        pub unsafe fn setImportance(&self, importance: AXCustomContentImportance);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait AXCustomContentProvider: NSObjectProtocol {
        #[cfg(all(
            feature = "Accessibility_AXCustomContent",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other accessibilityCustomContent)]
        unsafe fn accessibilityCustomContent(&self) -> Id<NSArray<AXCustomContent>>;

        #[cfg(all(
            feature = "Accessibility_AXCustomContent",
            feature = "Foundation_NSArray"
        ))]
        #[method(setAccessibilityCustomContent:)]
        unsafe fn setAccessibilityCustomContent(
            &self,
            accessibility_custom_content: Option<&NSArray<AXCustomContent>>,
        );
    }

    unsafe impl ProtocolType for dyn AXCustomContentProvider {}
);
