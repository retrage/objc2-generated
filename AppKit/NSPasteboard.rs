//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSPasteboardType = NSString;

extern_static!(NSPasteboardTypeString: &'static NSPasteboardType);

extern_static!(NSPasteboardTypePDF: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeTIFF: &'static NSPasteboardType);

extern_static!(NSPasteboardTypePNG: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeRTF: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeRTFD: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeHTML: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeTabularText: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeFont: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeRuler: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeColor: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeSound: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeMultipleTextSelection: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeTextFinderOptions: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeURL: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeFileURL: &'static NSPasteboardType);

pub type NSPasteboardName = NSString;

extern_static!(NSPasteboardNameGeneral: &'static NSPasteboardName);

extern_static!(NSPasteboardNameFont: &'static NSPasteboardName);

extern_static!(NSPasteboardNameRuler: &'static NSPasteboardName);

extern_static!(NSPasteboardNameFind: &'static NSPasteboardName);

extern_static!(NSPasteboardNameDrag: &'static NSPasteboardName);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPasteboardContentsOptions {
        NSPasteboardContentsCurrentHostOnly = 1 << 0,
    }
);

pub type NSPasteboardReadingOptionKey = NSString;

extern_static!(NSPasteboardURLReadingFileURLsOnlyKey: &'static NSPasteboardReadingOptionKey);

extern_static!(
    NSPasteboardURLReadingContentsConformToTypesKey: &'static NSPasteboardReadingOptionKey
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSPasteboard;

    unsafe impl ClassType for NSPasteboard {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPasteboard {
        #[method_id(@__retain_semantics Other generalPasteboard)]
        pub unsafe fn generalPasteboard() -> Id<NSPasteboard, Shared>;

        #[method_id(@__retain_semantics Other pasteboardWithName:)]
        pub unsafe fn pasteboardWithName(name: &NSPasteboardName) -> Id<NSPasteboard, Shared>;

        #[method_id(@__retain_semantics Other pasteboardWithUniqueName)]
        pub unsafe fn pasteboardWithUniqueName() -> Id<NSPasteboard, Shared>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSPasteboardName, Shared>;

        #[method(changeCount)]
        pub unsafe fn changeCount(&self) -> NSInteger;

        #[method(prepareForNewContentsWithOptions:)]
        pub unsafe fn prepareForNewContentsWithOptions(
            &self,
            options: NSPasteboardContentsOptions,
        ) -> NSInteger;

        #[method(clearContents)]
        pub unsafe fn clearContents(&self) -> NSInteger;

        #[method(writeObjects:)]
        pub unsafe fn writeObjects(&self, objects: &NSArray<NSPasteboardWriting>) -> bool;

        #[method_id(@__retain_semantics Other readObjectsForClasses:options:)]
        pub unsafe fn readObjectsForClasses_options(
            &self,
            classArray: &NSArray<TodoClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, Object>>,
        ) -> Option<Id<NSArray, Shared>>;

        #[method_id(@__retain_semantics Other pasteboardItems)]
        pub unsafe fn pasteboardItems(&self) -> Option<Id<NSArray<NSPasteboardItem>, Shared>>;

        #[method(indexOfPasteboardItem:)]
        pub unsafe fn indexOfPasteboardItem(&self, pasteboardItem: &NSPasteboardItem)
            -> NSUInteger;

        #[method(canReadItemWithDataConformingToTypes:)]
        pub unsafe fn canReadItemWithDataConformingToTypes(
            &self,
            types: &NSArray<NSString>,
        ) -> bool;

        #[method(canReadObjectForClasses:options:)]
        pub unsafe fn canReadObjectForClasses_options(
            &self,
            classArray: &NSArray<TodoClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, Object>>,
        ) -> bool;

        #[method(declareTypes:owner:)]
        pub unsafe fn declareTypes_owner(
            &self,
            newTypes: &NSArray<NSPasteboardType>,
            newOwner: Option<&Object>,
        ) -> NSInteger;

        #[method(addTypes:owner:)]
        pub unsafe fn addTypes_owner(
            &self,
            newTypes: &NSArray<NSPasteboardType>,
            newOwner: Option<&Object>,
        ) -> NSInteger;

        #[method_id(@__retain_semantics Other types)]
        pub unsafe fn types(&self) -> Option<Id<NSArray<NSPasteboardType>, Shared>>;

        #[method_id(@__retain_semantics Other availableTypeFromArray:)]
        pub unsafe fn availableTypeFromArray(
            &self,
            types: &NSArray<NSPasteboardType>,
        ) -> Option<Id<NSPasteboardType, Shared>>;

        #[method(setData:forType:)]
        pub unsafe fn setData_forType(
            &self,
            data: Option<&NSData>,
            dataType: &NSPasteboardType,
        ) -> bool;

        #[method(setPropertyList:forType:)]
        pub unsafe fn setPropertyList_forType(
            &self,
            plist: &Object,
            dataType: &NSPasteboardType,
        ) -> bool;

        #[method(setString:forType:)]
        pub unsafe fn setString_forType(
            &self,
            string: &NSString,
            dataType: &NSPasteboardType,
        ) -> bool;

        #[method_id(@__retain_semantics Other dataForType:)]
        pub unsafe fn dataForType(&self, dataType: &NSPasteboardType)
            -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other propertyListForType:)]
        pub unsafe fn propertyListForType(
            &self,
            dataType: &NSPasteboardType,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other stringForType:)]
        pub unsafe fn stringForType(
            &self,
            dataType: &NSPasteboardType,
        ) -> Option<Id<NSString, Shared>>;
    }
);

extern_methods!(
    /// FilterServices
    unsafe impl NSPasteboard {
        #[method_id(@__retain_semantics Other typesFilterableTo:)]
        pub unsafe fn typesFilterableTo(
            type_: &NSPasteboardType,
        ) -> Id<NSArray<NSPasteboardType>, Shared>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringFile:)]
        pub unsafe fn pasteboardByFilteringFile(filename: &NSString) -> Id<NSPasteboard, Shared>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringData:ofType:)]
        pub unsafe fn pasteboardByFilteringData_ofType(
            data: &NSData,
            type_: &NSPasteboardType,
        ) -> Id<NSPasteboard, Shared>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringTypesInPasteboard:)]
        pub unsafe fn pasteboardByFilteringTypesInPasteboard(
            pboard: &NSPasteboard,
        ) -> Id<NSPasteboard, Shared>;
    }
);

extern_protocol!(
    pub struct NSPasteboardTypeOwner;

    unsafe impl ProtocolType for NSPasteboardTypeOwner {
        #[method(pasteboard:provideDataForType:)]
        pub unsafe fn pasteboard_provideDataForType(
            &self,
            sender: &NSPasteboard,
            type_: &NSPasteboardType,
        );

        #[optional]
        #[method(pasteboardChangedOwner:)]
        pub unsafe fn pasteboardChangedOwner(&self, sender: &NSPasteboard);
    }
);

extern_methods!(
    /// NSPasteboardOwner
    unsafe impl NSObject {
        #[method(pasteboard:provideDataForType:)]
        pub unsafe fn pasteboard_provideDataForType(
            &self,
            sender: &NSPasteboard,
            type_: &NSPasteboardType,
        );

        #[method(pasteboardChangedOwner:)]
        pub unsafe fn pasteboardChangedOwner(&self, sender: &NSPasteboard);
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPasteboardWritingOptions {
        NSPasteboardWritingPromised = 1 << 9,
    }
);

extern_protocol!(
    pub struct NSPasteboardWriting;

    unsafe impl ProtocolType for NSPasteboardWriting {
        #[method_id(@__retain_semantics Other writableTypesForPasteboard:)]
        pub unsafe fn writableTypesForPasteboard(
            &self,
            pasteboard: &NSPasteboard,
        ) -> Id<NSArray<NSPasteboardType>, Shared>;

        #[optional]
        #[method(writingOptionsForType:pasteboard:)]
        pub unsafe fn writingOptionsForType_pasteboard(
            &self,
            type_: &NSPasteboardType,
            pasteboard: &NSPasteboard,
        ) -> NSPasteboardWritingOptions;

        #[method_id(@__retain_semantics Other pasteboardPropertyListForType:)]
        pub unsafe fn pasteboardPropertyListForType(
            &self,
            type_: &NSPasteboardType,
        ) -> Option<Id<Object, Shared>>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPasteboardReadingOptions {
        NSPasteboardReadingAsData = 0,
        NSPasteboardReadingAsString = 1 << 0,
        NSPasteboardReadingAsPropertyList = 1 << 1,
        NSPasteboardReadingAsKeyedArchive = 1 << 2,
    }
);

extern_protocol!(
    pub struct NSPasteboardReading;

    unsafe impl ProtocolType for NSPasteboardReading {
        #[method_id(@__retain_semantics Other readableTypesForPasteboard:)]
        pub unsafe fn readableTypesForPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Id<NSArray<NSPasteboardType>, Shared>;

        #[optional]
        #[method(readingOptionsForType:pasteboard:)]
        pub unsafe fn readingOptionsForType_pasteboard(
            type_: &NSPasteboardType,
            pasteboard: &NSPasteboard,
        ) -> NSPasteboardReadingOptions;

        #[optional]
        #[method_id(@__retain_semantics Init initWithPasteboardPropertyList:ofType:)]
        pub unsafe fn initWithPasteboardPropertyList_ofType(
            this: Option<Allocated<Self>>,
            propertyList: &Object,
            type_: &NSPasteboardType,
        ) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSPasteboardSupport
    unsafe impl NSURL {
        #[method_id(@__retain_semantics Other URLFromPasteboard:)]
        pub unsafe fn URLFromPasteboard(pasteBoard: &NSPasteboard) -> Option<Id<NSURL, Shared>>;

        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, pasteBoard: &NSPasteboard);
    }
);

extern_methods!(
    /// NSPasteboardSupport
    unsafe impl NSString {}
);

extern_methods!(
    /// NSFileContents
    unsafe impl NSPasteboard {
        #[method(writeFileContents:)]
        pub unsafe fn writeFileContents(&self, filename: &NSString) -> bool;

        #[method_id(@__retain_semantics Other readFileContentsType:toFile:)]
        pub unsafe fn readFileContentsType_toFile(
            &self,
            type_: Option<&NSPasteboardType>,
            filename: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method(writeFileWrapper:)]
        pub unsafe fn writeFileWrapper(&self, wrapper: &NSFileWrapper) -> bool;

        #[method_id(@__retain_semantics Other readFileWrapper)]
        pub unsafe fn readFileWrapper(&self) -> Option<Id<NSFileWrapper, Shared>>;
    }
);

extern_static!(NSFileContentsPboardType: &'static NSPasteboardType);

extern_fn!(
    pub unsafe fn NSCreateFilenamePboardType(fileType: &NSString) -> *mut NSPasteboardType;
);

extern_fn!(
    pub unsafe fn NSCreateFileContentsPboardType(fileType: &NSString) -> *mut NSPasteboardType;
);

extern_fn!(
    pub unsafe fn NSGetFileType(pboardType: &NSPasteboardType) -> *mut NSString;
);

extern_fn!(
    pub unsafe fn NSGetFileTypes(pboardTypes: &NSArray<NSPasteboardType>)
        -> *mut NSArray<NSString>;
);

extern_static!(NSStringPboardType: &'static NSPasteboardType);

extern_static!(NSFilenamesPboardType: &'static NSPasteboardType);

extern_static!(NSTIFFPboardType: &'static NSPasteboardType);

extern_static!(NSRTFPboardType: &'static NSPasteboardType);

extern_static!(NSTabularTextPboardType: &'static NSPasteboardType);

extern_static!(NSFontPboardType: &'static NSPasteboardType);

extern_static!(NSRulerPboardType: &'static NSPasteboardType);

extern_static!(NSColorPboardType: &'static NSPasteboardType);

extern_static!(NSRTFDPboardType: &'static NSPasteboardType);

extern_static!(NSHTMLPboardType: &'static NSPasteboardType);

extern_static!(NSURLPboardType: &'static NSPasteboardType);

extern_static!(NSPDFPboardType: &'static NSPasteboardType);

extern_static!(NSMultipleTextSelectionPboardType: &'static NSPasteboardType);

extern_static!(NSPostScriptPboardType: &'static NSPasteboardType);

extern_static!(NSVCardPboardType: &'static NSPasteboardType);

extern_static!(NSInkTextPboardType: &'static NSPasteboardType);

extern_static!(NSFilesPromisePboardType: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeFindPanelSearchOptions: &'static NSPasteboardType);

extern_static!(NSGeneralPboard: &'static NSPasteboardName);

extern_static!(NSFontPboard: &'static NSPasteboardName);

extern_static!(NSRulerPboard: &'static NSPasteboardName);

extern_static!(NSFindPboard: &'static NSPasteboardName);

extern_static!(NSDragPboard: &'static NSPasteboardName);

extern_static!(NSPICTPboardType: &'static NSPasteboardType);
