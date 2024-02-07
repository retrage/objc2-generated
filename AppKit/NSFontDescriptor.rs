//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSFontSymbolicTraits = u32;

ns_options!(
    #[underlying(u32)]
    pub enum NSFontDescriptorSymbolicTraits {
        NSFontDescriptorTraitItalic = 1 << 0,
        NSFontDescriptorTraitBold = 1 << 1,
        NSFontDescriptorTraitExpanded = 1 << 5,
        NSFontDescriptorTraitCondensed = 1 << 6,
        NSFontDescriptorTraitMonoSpace = 1 << 10,
        NSFontDescriptorTraitVertical = 1 << 11,
        NSFontDescriptorTraitUIOptimized = 1 << 12,
        NSFontDescriptorTraitTightLeading = 1 << 15,
        NSFontDescriptorTraitLooseLeading = 1 << 16,
        NSFontDescriptorTraitEmphasized = NSFontDescriptorTraitBold.0,
        NSFontDescriptorClassMask = 0xF0000000,
        NSFontDescriptorClassUnknown = 0 << 28,
        NSFontDescriptorClassOldStyleSerifs = 1 << 28,
        NSFontDescriptorClassTransitionalSerifs = 2 << 28,
        NSFontDescriptorClassModernSerifs = 3 << 28,
        NSFontDescriptorClassClarendonSerifs = 4 << 28,
        NSFontDescriptorClassSlabSerifs = 5 << 28,
        NSFontDescriptorClassFreeformSerifs = 7 << 28,
        NSFontDescriptorClassSansSerif = 8 << 28,
        NSFontDescriptorClassOrnamentals = 9 << 28,
        NSFontDescriptorClassScripts = 10 << 28,
        NSFontDescriptorClassSymbolic = 12 << 28,
    }
);

typed_extensible_enum!(
    pub type NSFontDescriptorAttributeName = NSString;
);

typed_enum!(
    pub type NSFontDescriptorTraitKey = NSString;
);

typed_enum!(
    pub type NSFontDescriptorVariationKey = NSString;
);

typed_extensible_enum!(
    pub type NSFontDescriptorFeatureKey = NSString;
);

typed_extensible_enum!(
    pub type NSFontWeight = CGFloat;
);

typed_extensible_enum!(
    pub type NSFontWidth = CGFloat;
);

typed_enum!(
    pub type NSFontDescriptorSystemDesign = NSString;
);

typed_enum!(
    pub type NSFontTextStyle = NSString;
);

typed_enum!(
    pub type NSFontTextStyleOptionKey = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSFontDescriptor")]
    pub struct NSFontDescriptor;

    #[cfg(feature = "AppKit_NSFontDescriptor")]
    unsafe impl ClassType for NSFontDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSFontDescriptor")]
unsafe impl NSCoding for NSFontDescriptor {}

#[cfg(feature = "AppKit_NSFontDescriptor")]
unsafe impl NSCopying for NSFontDescriptor {}

#[cfg(feature = "AppKit_NSFontDescriptor")]
unsafe impl NSObjectProtocol for NSFontDescriptor {}

#[cfg(feature = "AppKit_NSFontDescriptor")]
unsafe impl NSSecureCoding for NSFontDescriptor {}

extern_methods!(
    #[cfg(feature = "AppKit_NSFontDescriptor")]
    unsafe impl NSFontDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other postscriptName)]
        pub unsafe fn postscriptName(&self) -> Option<Id<NSString>>;

        #[method(pointSize)]
        pub unsafe fn pointSize(&self) -> CGFloat;

        #[cfg(feature = "Foundation_NSAffineTransform")]
        #[method_id(@__retain_semantics Other matrix)]
        pub unsafe fn matrix(&self) -> Option<Id<NSAffineTransform>>;

        #[method(symbolicTraits)]
        pub unsafe fn symbolicTraits(&self) -> NSFontDescriptorSymbolicTraits;

        #[method(requiresFontAssetRequest)]
        pub unsafe fn requiresFontAssetRequest(&self) -> bool;

        #[method_id(@__retain_semantics Other objectForKey:)]
        pub unsafe fn objectForKey(
            &self,
            attribute: &NSFontDescriptorAttributeName,
        ) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other fontAttributes)]
        pub unsafe fn fontAttributes(
            &self,
        ) -> Id<NSDictionary<NSFontDescriptorAttributeName, AnyObject>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other fontDescriptorWithFontAttributes:)]
        pub unsafe fn fontDescriptorWithFontAttributes(
            attributes: Option<&NSDictionary<NSFontDescriptorAttributeName, AnyObject>>,
        ) -> Id<NSFontDescriptor>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fontDescriptorWithName:size:)]
        pub unsafe fn fontDescriptorWithName_size(
            font_name: &NSString,
            size: CGFloat,
        ) -> Id<NSFontDescriptor>;

        #[cfg(all(
            feature = "Foundation_NSAffineTransform",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other fontDescriptorWithName:matrix:)]
        pub unsafe fn fontDescriptorWithName_matrix(
            font_name: &NSString,
            matrix: &NSAffineTransform,
        ) -> Id<NSFontDescriptor>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Init initWithFontAttributes:)]
        pub unsafe fn initWithFontAttributes(
            this: Allocated<Self>,
            attributes: Option<&NSDictionary<NSFontDescriptorAttributeName, AnyObject>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSSet"))]
        #[method_id(@__retain_semantics Other matchingFontDescriptorsWithMandatoryKeys:)]
        pub unsafe fn matchingFontDescriptorsWithMandatoryKeys(
            &self,
            mandatory_keys: Option<&NSSet<NSFontDescriptorAttributeName>>,
        ) -> Id<NSArray<NSFontDescriptor>>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other matchingFontDescriptorWithMandatoryKeys:)]
        pub unsafe fn matchingFontDescriptorWithMandatoryKeys(
            &self,
            mandatory_keys: Option<&NSSet<NSFontDescriptorAttributeName>>,
        ) -> Option<Id<NSFontDescriptor>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other fontDescriptorByAddingAttributes:)]
        pub unsafe fn fontDescriptorByAddingAttributes(
            &self,
            attributes: &NSDictionary<NSFontDescriptorAttributeName, AnyObject>,
        ) -> Id<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithSymbolicTraits:)]
        pub unsafe fn fontDescriptorWithSymbolicTraits(
            &self,
            symbolic_traits: NSFontDescriptorSymbolicTraits,
        ) -> Id<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithSize:)]
        pub unsafe fn fontDescriptorWithSize(
            &self,
            new_point_size: CGFloat,
        ) -> Id<NSFontDescriptor>;

        #[cfg(feature = "Foundation_NSAffineTransform")]
        #[method_id(@__retain_semantics Other fontDescriptorWithMatrix:)]
        pub unsafe fn fontDescriptorWithMatrix(
            &self,
            matrix: &NSAffineTransform,
        ) -> Id<NSFontDescriptor>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fontDescriptorWithFace:)]
        pub unsafe fn fontDescriptorWithFace(&self, new_face: &NSString) -> Id<NSFontDescriptor>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fontDescriptorWithFamily:)]
        pub unsafe fn fontDescriptorWithFamily(
            &self,
            new_family: &NSString,
        ) -> Id<NSFontDescriptor>;

        #[method_id(@__retain_semantics Other fontDescriptorWithDesign:)]
        pub unsafe fn fontDescriptorWithDesign(
            &self,
            design: &NSFontDescriptorSystemDesign,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSFontDescriptor")]
    unsafe impl NSFontDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSFontFamilyAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontNameAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontFaceAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontSizeAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontVisibleNameAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontMatrixAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontVariationAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontCharacterSetAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontCascadeListAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontTraitsAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontFixedAdvanceAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontFeatureSettingsAttribute: &'static NSFontDescriptorAttributeName);

extern_static!(NSFontSymbolicTrait: &'static NSFontDescriptorTraitKey);

extern_static!(NSFontWeightTrait: &'static NSFontDescriptorTraitKey);

extern_static!(NSFontWidthTrait: &'static NSFontDescriptorTraitKey);

extern_static!(NSFontSlantTrait: &'static NSFontDescriptorTraitKey);

extern_static!(NSFontVariationAxisIdentifierKey: &'static NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisMinimumValueKey: &'static NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisMaximumValueKey: &'static NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisDefaultValueKey: &'static NSFontDescriptorVariationKey);

extern_static!(NSFontVariationAxisNameKey: &'static NSFontDescriptorVariationKey);

extern_static!(NSFontFeatureTypeIdentifierKey: &'static NSFontDescriptorFeatureKey);

extern_static!(NSFontFeatureSelectorIdentifierKey: &'static NSFontDescriptorFeatureKey);

extern_static!(NSFontWeightUltraLight: NSFontWeight);

extern_static!(NSFontWeightThin: NSFontWeight);

extern_static!(NSFontWeightLight: NSFontWeight);

extern_static!(NSFontWeightRegular: NSFontWeight);

extern_static!(NSFontWeightMedium: NSFontWeight);

extern_static!(NSFontWeightSemibold: NSFontWeight);

extern_static!(NSFontWeightBold: NSFontWeight);

extern_static!(NSFontWeightHeavy: NSFontWeight);

extern_static!(NSFontWeightBlack: NSFontWeight);

extern_static!(NSFontWidthCompressed: NSFontWidth);

extern_static!(NSFontWidthCondensed: NSFontWidth);

extern_static!(NSFontWidthStandard: NSFontWidth);

extern_static!(NSFontWidthExpanded: NSFontWidth);

extern_static!(NSFontDescriptorSystemDesignDefault: &'static NSFontDescriptorSystemDesign);

extern_static!(NSFontDescriptorSystemDesignSerif: &'static NSFontDescriptorSystemDesign);

extern_static!(NSFontDescriptorSystemDesignMonospaced: &'static NSFontDescriptorSystemDesign);

extern_static!(NSFontDescriptorSystemDesignRounded: &'static NSFontDescriptorSystemDesign);

extern_static!(NSFontTextStyleLargeTitle: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleTitle1: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleTitle2: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleTitle3: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleHeadline: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleSubheadline: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleBody: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleCallout: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleFootnote: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleCaption1: &'static NSFontTextStyle);

extern_static!(NSFontTextStyleCaption2: &'static NSFontTextStyle);

pub type NSFontFamilyClass = u32;

pub const NSFontUnknownClass: c_int = 0 << 28;
pub const NSFontOldStyleSerifsClass: c_int = 1 << 28;
pub const NSFontTransitionalSerifsClass: c_int = 2 << 28;
pub const NSFontModernSerifsClass: c_int = 3 << 28;
pub const NSFontClarendonSerifsClass: c_int = 4 << 28;
pub const NSFontSlabSerifsClass: c_int = 5 << 28;
pub const NSFontFreeformSerifsClass: c_int = 7 << 28;
pub const NSFontSansSerifClass: c_int = 8 << 28;
pub const NSFontOrnamentalsClass: c_int = 9 << 28;
pub const NSFontScriptsClass: c_int = 10 << 28;
pub const NSFontSymbolicClass: c_int = 12 << 28;

pub const NSFontFamilyClassMask: c_uint = 0xF0000000;

pub const NSFontItalicTrait: c_uint = 1 << 0;
pub const NSFontBoldTrait: c_uint = 1 << 1;
pub const NSFontExpandedTrait: c_uint = 1 << 5;
pub const NSFontCondensedTrait: c_uint = 1 << 6;
pub const NSFontMonoSpaceTrait: c_uint = 1 << 10;
pub const NSFontVerticalTrait: c_uint = 1 << 11;
pub const NSFontUIOptimizedTrait: c_uint = 1 << 12;

extern_static!(NSFontColorAttribute: &'static NSString);

extern_methods!(
    /// NSFontDescriptor_TextStyles
    #[cfg(feature = "AppKit_NSFontDescriptor")]
    unsafe impl NSFontDescriptor {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other preferredFontDescriptorForTextStyle:options:)]
        pub unsafe fn preferredFontDescriptorForTextStyle_options(
            style: &NSFontTextStyle,
            options: &NSDictionary<NSFontTextStyleOptionKey, AnyObject>,
        ) -> Id<NSFontDescriptor>;
    }
);
