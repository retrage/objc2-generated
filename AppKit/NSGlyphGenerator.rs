//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSShowControlGlyphs = 1<<0,
        NSShowInvisibleGlyphs = 1<<1,
        NSWantsBidiLevels = 1<<2,
    }
);

extern_protocol!(
    pub struct NSGlyphStorage;

    unsafe impl ProtocolType for NSGlyphStorage {
        #[method(insertGlyphs:length:forStartingGlyphAtIndex:characterIndex:)]
        pub unsafe fn insertGlyphs_length_forStartingGlyphAtIndex_characterIndex(
            &self,
            glyphs: NonNull<NSGlyph>,
            length: NSUInteger,
            glyphIndex: NSUInteger,
            charIndex: NSUInteger,
        );

        #[method(setIntAttribute:value:forGlyphAtIndex:)]
        pub unsafe fn setIntAttribute_value_forGlyphAtIndex(
            &self,
            attributeTag: NSInteger,
            val: NSInteger,
            glyphIndex: NSUInteger,
        );

        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Id<NSAttributedString, Shared>;

        #[method(layoutOptions)]
        pub unsafe fn layoutOptions(&self) -> NSUInteger;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSGlyphGenerator;

    unsafe impl ClassType for NSGlyphGenerator {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSGlyphGenerator {
        #[method(generateGlyphsForGlyphStorage:desiredNumberOfCharacters:glyphIndex:characterIndex:)]
        pub unsafe fn generateGlyphsForGlyphStorage_desiredNumberOfCharacters_glyphIndex_characterIndex(
            &self,
            glyphStorage: &NSGlyphStorage,
            nChars: NSUInteger,
            glyphIndex: *mut NSUInteger,
            charIndex: *mut NSUInteger,
        );

        #[method_id(@__retain_semantics Other sharedGlyphGenerator)]
        pub unsafe fn sharedGlyphGenerator() -> Id<NSGlyphGenerator, Shared>;
    }
);
