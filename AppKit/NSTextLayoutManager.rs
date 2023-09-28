//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextLayoutManagerSegmentType {
        NSTextLayoutManagerSegmentTypeStandard = 0,
        NSTextLayoutManagerSegmentTypeSelection = 1,
        NSTextLayoutManagerSegmentTypeHighlight = 2,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextLayoutManagerSegmentOptions {
        NSTextLayoutManagerSegmentOptionsNone = 0,
        NSTextLayoutManagerSegmentOptionsRangeNotRequired = 1 << 0,
        NSTextLayoutManagerSegmentOptionsMiddleFragmentsExcluded = 1 << 1,
        NSTextLayoutManagerSegmentOptionsHeadSegmentExtended = 1 << 2,
        NSTextLayoutManagerSegmentOptionsTailSegmentExtended = 1 << 3,
        NSTextLayoutManagerSegmentOptionsUpstreamAffinity = 1 << 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextLayoutManager")]
    pub struct NSTextLayoutManager;

    #[cfg(feature = "AppKit_NSTextLayoutManager")]
    unsafe impl ClassType for NSTextLayoutManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSTextLayoutManager")]
unsafe impl NSCoding for NSTextLayoutManager {}

#[cfg(feature = "AppKit_NSTextLayoutManager")]
unsafe impl NSObjectProtocol for NSTextLayoutManager {}

#[cfg(feature = "AppKit_NSTextLayoutManager")]
unsafe impl NSSecureCoding for NSTextLayoutManager {}

#[cfg(feature = "AppKit_NSTextLayoutManager")]
unsafe impl NSTextSelectionDataSource for NSTextLayoutManager {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextLayoutManager")]
    unsafe impl NSTextLayoutManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Allocated<Self>, coder: &NSCoder) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTextLayoutManagerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextLayoutManagerDelegate>>,
        );

        #[method(usesFontLeading)]
        pub unsafe fn usesFontLeading(&self) -> bool;

        #[method(setUsesFontLeading:)]
        pub unsafe fn setUsesFontLeading(&self, uses_font_leading: bool);

        #[method(limitsLayoutForSuspiciousContents)]
        pub unsafe fn limitsLayoutForSuspiciousContents(&self) -> bool;

        #[method(setLimitsLayoutForSuspiciousContents:)]
        pub unsafe fn setLimitsLayoutForSuspiciousContents(
            &self,
            limits_layout_for_suspicious_contents: bool,
        );

        #[method(usesHyphenation)]
        pub unsafe fn usesHyphenation(&self) -> bool;

        #[method(setUsesHyphenation:)]
        pub unsafe fn setUsesHyphenation(&self, uses_hyphenation: bool);

        #[cfg(feature = "AppKit_NSTextContentManager")]
        #[method_id(@__retain_semantics Other textContentManager)]
        pub unsafe fn textContentManager(&self) -> Option<Id<NSTextContentManager>>;

        #[cfg(feature = "AppKit_NSTextContentManager")]
        #[method(replaceTextContentManager:)]
        pub unsafe fn replaceTextContentManager(&self, text_content_manager: &NSTextContentManager);

        #[cfg(feature = "AppKit_NSTextContainer")]
        #[method_id(@__retain_semantics Other textContainer)]
        pub unsafe fn textContainer(&self) -> Option<Id<NSTextContainer>>;

        #[cfg(feature = "AppKit_NSTextContainer")]
        #[method(setTextContainer:)]
        pub unsafe fn setTextContainer(&self, text_container: Option<&NSTextContainer>);

        #[method(usageBoundsForTextContainer)]
        pub unsafe fn usageBoundsForTextContainer(&self) -> CGRect;

        #[cfg(feature = "AppKit_NSTextViewportLayoutController")]
        #[method_id(@__retain_semantics Other textViewportLayoutController)]
        pub unsafe fn textViewportLayoutController(&self) -> Id<NSTextViewportLayoutController>;

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method_id(@__retain_semantics Other layoutQueue)]
        pub unsafe fn layoutQueue(&self) -> Option<Id<NSOperationQueue>>;

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method(setLayoutQueue:)]
        pub unsafe fn setLayoutQueue(&self, layout_queue: Option<&NSOperationQueue>);

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method(ensureLayoutForRange:)]
        pub unsafe fn ensureLayoutForRange(&self, range: &NSTextRange);

        #[method(ensureLayoutForBounds:)]
        pub unsafe fn ensureLayoutForBounds(&self, bounds: CGRect);

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method(invalidateLayoutForRange:)]
        pub unsafe fn invalidateLayoutForRange(&self, range: &NSTextRange);

        #[cfg(feature = "AppKit_NSTextLayoutFragment")]
        #[method_id(@__retain_semantics Other textLayoutFragmentForPosition:)]
        pub unsafe fn textLayoutFragmentForPosition(
            &self,
            position: CGPoint,
        ) -> Option<Id<NSTextLayoutFragment>>;

        #[cfg(feature = "AppKit_NSTextLayoutFragment")]
        #[method_id(@__retain_semantics Other textLayoutFragmentForLocation:)]
        pub unsafe fn textLayoutFragmentForLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Id<NSTextLayoutFragment>>;

        #[cfg(feature = "AppKit_NSTextLayoutFragment")]
        #[method_id(@__retain_semantics Other enumerateTextLayoutFragmentsFromLocation:options:usingBlock:)]
        pub unsafe fn enumerateTextLayoutFragmentsFromLocation_options_usingBlock(
            &self,
            location: Option<&ProtocolObject<dyn NSTextLocation>>,
            options: NSTextLayoutFragmentEnumerationOptions,
            block: &Block<(NonNull<NSTextLayoutFragment>,), Bool>,
        ) -> Option<Id<ProtocolObject<dyn NSTextLocation>>>;

        #[cfg(all(feature = "AppKit_NSTextSelection", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textSelections)]
        pub unsafe fn textSelections(&self) -> Id<NSArray<NSTextSelection>>;

        #[cfg(all(feature = "AppKit_NSTextSelection", feature = "Foundation_NSArray"))]
        #[method(setTextSelections:)]
        pub unsafe fn setTextSelections(&self, text_selections: &NSArray<NSTextSelection>);

        #[cfg(feature = "AppKit_NSTextSelectionNavigation")]
        #[method_id(@__retain_semantics Other textSelectionNavigation)]
        pub unsafe fn textSelectionNavigation(&self) -> Id<NSTextSelectionNavigation>;

        #[cfg(feature = "AppKit_NSTextSelectionNavigation")]
        #[method(setTextSelectionNavigation:)]
        pub unsafe fn setTextSelectionNavigation(
            &self,
            text_selection_navigation: &NSTextSelectionNavigation,
        );

        #[cfg(all(feature = "AppKit_NSTextRange", feature = "Foundation_NSDictionary"))]
        #[method(enumerateRenderingAttributesFromLocation:reverse:usingBlock:)]
        pub unsafe fn enumerateRenderingAttributesFromLocation_reverse_usingBlock(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            reverse: bool,
            block: &Block<
                (
                    NonNull<NSTextLayoutManager>,
                    NonNull<NSDictionary<NSAttributedStringKey, AnyObject>>,
                    NonNull<NSTextRange>,
                ),
                Bool,
            >,
        );

        #[cfg(all(feature = "AppKit_NSTextRange", feature = "Foundation_NSDictionary"))]
        #[method(setRenderingAttributes:forTextRange:)]
        pub unsafe fn setRenderingAttributes_forTextRange(
            &self,
            rendering_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
            text_range: &NSTextRange,
        );

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method(addRenderingAttribute:value:forTextRange:)]
        pub unsafe fn addRenderingAttribute_value_forTextRange(
            &self,
            rendering_attribute: &NSAttributedStringKey,
            value: Option<&AnyObject>,
            text_range: &NSTextRange,
        );

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method(removeRenderingAttribute:forTextRange:)]
        pub unsafe fn removeRenderingAttribute_forTextRange(
            &self,
            rendering_attribute: &NSAttributedStringKey,
            text_range: &NSTextRange,
        );

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method(invalidateRenderingAttributesForTextRange:)]
        pub unsafe fn invalidateRenderingAttributesForTextRange(&self, text_range: &NSTextRange);

        #[cfg(feature = "AppKit_NSTextLayoutFragment")]
        #[method(renderingAttributesValidator)]
        pub unsafe fn renderingAttributesValidator(
            &self,
        ) -> *mut Block<(NonNull<NSTextLayoutManager>, NonNull<NSTextLayoutFragment>), ()>;

        #[cfg(feature = "AppKit_NSTextLayoutFragment")]
        #[method(setRenderingAttributesValidator:)]
        pub unsafe fn setRenderingAttributesValidator(
            &self,
            rendering_attributes_validator: Option<
                &Block<(NonNull<NSTextLayoutManager>, NonNull<NSTextLayoutFragment>), ()>,
            >,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other linkRenderingAttributes)]
        pub unsafe fn linkRenderingAttributes() -> Id<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other renderingAttributesForLink:atLocation:)]
        pub unsafe fn renderingAttributesForLink_atLocation(
            &self,
            link: &AnyObject,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Id<NSDictionary<NSAttributedStringKey, AnyObject>>;

        #[cfg(all(feature = "AppKit_NSTextContainer", feature = "AppKit_NSTextRange"))]
        #[method(enumerateTextSegmentsInRange:type:options:usingBlock:)]
        pub unsafe fn enumerateTextSegmentsInRange_type_options_usingBlock(
            &self,
            text_range: &NSTextRange,
            r#type: NSTextLayoutManagerSegmentType,
            options: NSTextLayoutManagerSegmentOptions,
            block: &Block<(*mut NSTextRange, CGRect, CGFloat, NonNull<NSTextContainer>), Bool>,
        );

        #[cfg(all(
            feature = "AppKit_NSTextElement",
            feature = "AppKit_NSTextRange",
            feature = "Foundation_NSArray"
        ))]
        #[method(replaceContentsInRange:withTextElements:)]
        pub unsafe fn replaceContentsInRange_withTextElements(
            &self,
            range: &NSTextRange,
            text_elements: &NSArray<NSTextElement>,
        );

        #[cfg(all(
            feature = "AppKit_NSTextRange",
            feature = "Foundation_NSAttributedString"
        ))]
        #[method(replaceContentsInRange:withAttributedString:)]
        pub unsafe fn replaceContentsInRange_withAttributedString(
            &self,
            range: &NSTextRange,
            attributed_string: &NSAttributedString,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSTextLayoutManager")]
    unsafe impl NSTextLayoutManager {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSTextLayoutManagerDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSTextElement",
            feature = "AppKit_NSTextLayoutFragment",
            feature = "AppKit_NSTextLayoutManager"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textLayoutManager:textLayoutFragmentForLocation:inTextElement:)]
        unsafe fn textLayoutManager_textLayoutFragmentForLocation_inTextElement(
            &self,
            text_layout_manager: &NSTextLayoutManager,
            location: &ProtocolObject<dyn NSTextLocation>,
            text_element: &NSTextElement,
        ) -> Id<NSTextLayoutFragment>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[optional]
        #[method(textLayoutManager:shouldBreakLineBeforeLocation:hyphenating:)]
        unsafe fn textLayoutManager_shouldBreakLineBeforeLocation_hyphenating(
            &self,
            text_layout_manager: &NSTextLayoutManager,
            location: &ProtocolObject<dyn NSTextLocation>,
            hyphenating: bool,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSTextLayoutManager",
            feature = "Foundation_NSDictionary"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textLayoutManager:renderingAttributesForLink:atLocation:defaultAttributes:)]
        unsafe fn textLayoutManager_renderingAttributesForLink_atLocation_defaultAttributes(
            &self,
            text_layout_manager: &NSTextLayoutManager,
            link: &AnyObject,
            location: &ProtocolObject<dyn NSTextLocation>,
            rendering_attributes: &NSDictionary<NSAttributedStringKey, AnyObject>,
        ) -> Option<Id<NSDictionary<NSAttributedStringKey, AnyObject>>>;
    }

    unsafe impl ProtocolType for dyn NSTextLayoutManagerDelegate {}
);
