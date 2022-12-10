//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSWritingDirection {
        NSWritingDirectionNatural = -1,
        NSWritingDirectionLeftToRight = 0,
        NSWritingDirectionRightToLeft = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSText;

    unsafe impl ClassType for NSText {
        type Super = NSView;
    }
);

extern_methods!(
    unsafe impl NSText {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frameRect: NSRect,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Id<NSString, Shared>;

        #[method(setString:)]
        pub unsafe fn setString(&self, string: &NSString);

        #[method(replaceCharactersInRange:withString:)]
        pub unsafe fn replaceCharactersInRange_withString(&self, range: NSRange, string: &NSString);

        #[method(replaceCharactersInRange:withRTF:)]
        pub unsafe fn replaceCharactersInRange_withRTF(&self, range: NSRange, rtfData: &NSData);

        #[method(replaceCharactersInRange:withRTFD:)]
        pub unsafe fn replaceCharactersInRange_withRTFD(&self, range: NSRange, rtfdData: &NSData);

        #[method_id(@__retain_semantics Other RTFFromRange:)]
        pub unsafe fn RTFFromRange(&self, range: NSRange) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other RTFDFromRange:)]
        pub unsafe fn RTFDFromRange(&self, range: NSRange) -> Option<Id<NSData, Shared>>;

        #[method(writeRTFDToFile:atomically:)]
        pub unsafe fn writeRTFDToFile_atomically(&self, path: &NSString, flag: bool) -> bool;

        #[method(readRTFDFromFile:)]
        pub unsafe fn readRTFDFromFile(&self, path: &NSString) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSTextDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSTextDelegate>);

        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[method(isSelectable)]
        pub unsafe fn isSelectable(&self) -> bool;

        #[method(setSelectable:)]
        pub unsafe fn setSelectable(&self, selectable: bool);

        #[method(isRichText)]
        pub unsafe fn isRichText(&self) -> bool;

        #[method(setRichText:)]
        pub unsafe fn setRichText(&self, richText: bool);

        #[method(importsGraphics)]
        pub unsafe fn importsGraphics(&self) -> bool;

        #[method(setImportsGraphics:)]
        pub unsafe fn setImportsGraphics(&self, importsGraphics: bool);

        #[method(isFieldEditor)]
        pub unsafe fn isFieldEditor(&self) -> bool;

        #[method(setFieldEditor:)]
        pub unsafe fn setFieldEditor(&self, fieldEditor: bool);

        #[method(usesFontPanel)]
        pub unsafe fn usesFontPanel(&self) -> bool;

        #[method(setUsesFontPanel:)]
        pub unsafe fn setUsesFontPanel(&self, usesFontPanel: bool);

        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, drawsBackground: bool);

        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);

        #[method(isRulerVisible)]
        pub unsafe fn isRulerVisible(&self) -> bool;

        #[method(selectedRange)]
        pub unsafe fn selectedRange(&self) -> NSRange;

        #[method(setSelectedRange:)]
        pub unsafe fn setSelectedRange(&self, selectedRange: NSRange);

        #[method(scrollRangeToVisible:)]
        pub unsafe fn scrollRangeToVisible(&self, range: NSRange);

        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont, Shared>>;

        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[method_id(@__retain_semantics Other textColor)]
        pub unsafe fn textColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setTextColor:)]
        pub unsafe fn setTextColor(&self, textColor: Option<&NSColor>);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, baseWritingDirection: NSWritingDirection);

        #[method(setTextColor:range:)]
        pub unsafe fn setTextColor_range(&self, color: Option<&NSColor>, range: NSRange);

        #[method(setFont:range:)]
        pub unsafe fn setFont_range(&self, font: &NSFont, range: NSRange);

        #[method(maxSize)]
        pub unsafe fn maxSize(&self) -> NSSize;

        #[method(setMaxSize:)]
        pub unsafe fn setMaxSize(&self, maxSize: NSSize);

        #[method(minSize)]
        pub unsafe fn minSize(&self) -> NSSize;

        #[method(setMinSize:)]
        pub unsafe fn setMinSize(&self, minSize: NSSize);

        #[method(isHorizontallyResizable)]
        pub unsafe fn isHorizontallyResizable(&self) -> bool;

        #[method(setHorizontallyResizable:)]
        pub unsafe fn setHorizontallyResizable(&self, horizontallyResizable: bool);

        #[method(isVerticallyResizable)]
        pub unsafe fn isVerticallyResizable(&self) -> bool;

        #[method(setVerticallyResizable:)]
        pub unsafe fn setVerticallyResizable(&self, verticallyResizable: bool);

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(copy:)]
        pub unsafe fn copy(&self, sender: Option<&Object>);

        #[method(copyFont:)]
        pub unsafe fn copyFont(&self, sender: Option<&Object>);

        #[method(copyRuler:)]
        pub unsafe fn copyRuler(&self, sender: Option<&Object>);

        #[method(cut:)]
        pub unsafe fn cut(&self, sender: Option<&Object>);

        #[method(delete:)]
        pub unsafe fn delete(&self, sender: Option<&Object>);

        #[method(paste:)]
        pub unsafe fn paste(&self, sender: Option<&Object>);

        #[method(pasteFont:)]
        pub unsafe fn pasteFont(&self, sender: Option<&Object>);

        #[method(pasteRuler:)]
        pub unsafe fn pasteRuler(&self, sender: Option<&Object>);

        #[method(selectAll:)]
        pub unsafe fn selectAll(&self, sender: Option<&Object>);

        #[method(changeFont:)]
        pub unsafe fn changeFont(&self, sender: Option<&Object>);

        #[method(alignLeft:)]
        pub unsafe fn alignLeft(&self, sender: Option<&Object>);

        #[method(alignRight:)]
        pub unsafe fn alignRight(&self, sender: Option<&Object>);

        #[method(alignCenter:)]
        pub unsafe fn alignCenter(&self, sender: Option<&Object>);

        #[method(subscript:)]
        pub unsafe fn subscript(&self, sender: Option<&Object>);

        #[method(superscript:)]
        pub unsafe fn superscript(&self, sender: Option<&Object>);

        #[method(underline:)]
        pub unsafe fn underline(&self, sender: Option<&Object>);

        #[method(unscript:)]
        pub unsafe fn unscript(&self, sender: Option<&Object>);

        #[method(showGuessPanel:)]
        pub unsafe fn showGuessPanel(&self, sender: Option<&Object>);

        #[method(checkSpelling:)]
        pub unsafe fn checkSpelling(&self, sender: Option<&Object>);

        #[method(toggleRuler:)]
        pub unsafe fn toggleRuler(&self, sender: Option<&Object>);
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSEnterCharacter = 0x0003,
        NSBackspaceCharacter = 0x0008,
        NSTabCharacter = 0x0009,
        NSNewlineCharacter = 0x000a,
        NSFormFeedCharacter = 0x000c,
        NSCarriageReturnCharacter = 0x000d,
        NSBackTabCharacter = 0x0019,
        NSDeleteCharacter = 0x007f,
        NSLineSeparatorCharacter = 0x2028,
        NSParagraphSeparatorCharacter = 0x2029,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextMovement {
        NSTextMovementReturn = 0x10,
        NSTextMovementTab = 0x11,
        NSTextMovementBacktab = 0x12,
        NSTextMovementLeft = 0x13,
        NSTextMovementRight = 0x14,
        NSTextMovementUp = 0x15,
        NSTextMovementDown = 0x16,
        NSTextMovementCancel = 0x17,
        NSTextMovementOther = 0,
    }
);

extern_static!(NSTextDidBeginEditingNotification: &'static NSNotificationName);

extern_static!(NSTextDidEndEditingNotification: &'static NSNotificationName);

extern_static!(NSTextDidChangeNotification: &'static NSNotificationName);

extern_static!(NSTextMovementUserInfoKey: &'static NSString);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSIllegalTextMovement = 0,
        NSReturnTextMovement = 0x10,
        NSTabTextMovement = 0x11,
        NSBacktabTextMovement = 0x12,
        NSLeftTextMovement = 0x13,
        NSRightTextMovement = 0x14,
        NSUpTextMovement = 0x15,
        NSDownTextMovement = 0x16,
        NSCancelTextMovement = 0x17,
        NSOtherTextMovement = 0,
    }
);

extern_protocol!(
    pub struct NSTextDelegate;

    unsafe impl ProtocolType for NSTextDelegate {
        #[optional]
        #[method(textShouldBeginEditing:)]
        pub unsafe fn textShouldBeginEditing(&self, textObject: &NSText) -> bool;

        #[optional]
        #[method(textShouldEndEditing:)]
        pub unsafe fn textShouldEndEditing(&self, textObject: &NSText) -> bool;

        #[optional]
        #[method(textDidBeginEditing:)]
        pub unsafe fn textDidBeginEditing(&self, notification: &NSNotification);

        #[optional]
        #[method(textDidEndEditing:)]
        pub unsafe fn textDidEndEditing(&self, notification: &NSNotification);

        #[optional]
        #[method(textDidChange:)]
        pub unsafe fn textDidChange(&self, notification: &NSNotification);
    }
);

extern_enum!(
    #[underlying(c_uint)]
    pub enum {
        NSTextWritingDirectionEmbedding = 0<<1,
        NSTextWritingDirectionOverride = 1<<1,
    }
);

extern_static!(NSLeftTextAlignment: NSTextAlignment = NSTextAlignmentLeft);

extern_static!(NSRightTextAlignment: NSTextAlignment = NSTextAlignmentRight);

extern_static!(NSCenterTextAlignment: NSTextAlignment = NSTextAlignmentCenter);

extern_static!(NSJustifiedTextAlignment: NSTextAlignment = NSTextAlignmentJustified);

extern_static!(NSNaturalTextAlignment: NSTextAlignment = NSTextAlignmentNatural);
