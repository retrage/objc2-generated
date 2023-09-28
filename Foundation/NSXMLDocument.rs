//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSXMLDocumentContentKind {
        NSXMLDocumentXMLKind = 0,
        NSXMLDocumentXHTMLKind = 1,
        NSXMLDocumentHTMLKind = 2,
        NSXMLDocumentTextKind = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSXMLDocument")]
    pub struct NSXMLDocument;

    #[cfg(feature = "Foundation_NSXMLDocument")]
    unsafe impl ClassType for NSXMLDocument {
        #[inherits(NSObject)]
        type Super = NSXMLNode;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Foundation_NSXMLDocument")]
unsafe impl NSCopying for NSXMLDocument {}

#[cfg(feature = "Foundation_NSXMLDocument")]
unsafe impl NSObjectProtocol for NSXMLDocument {}

extern_methods!(
    #[cfg(feature = "Foundation_NSXMLDocument")]
    unsafe impl NSXMLDocument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithXMLString:options:error:_)]
        pub unsafe fn initWithXMLString_options_error(
            this: Allocated<Self>,
            string: &NSString,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Init initWithData:options:error:_)]
        pub unsafe fn initWithData_options_error(
            this: Allocated<Self>,
            data: &NSData,
            mask: NSXMLNodeOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSXMLElement")]
        #[method_id(@__retain_semantics Init initWithRootElement:)]
        pub unsafe fn initWithRootElement(
            this: Allocated<Self>,
            element: Option<&NSXMLElement>,
        ) -> Id<Self>;

        #[method(replacementClassForClass:)]
        pub unsafe fn replacementClassForClass(cls: &AnyClass) -> &'static AnyClass;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other characterEncoding)]
        pub unsafe fn characterEncoding(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCharacterEncoding:)]
        pub unsafe fn setCharacterEncoding(&self, character_encoding: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other version)]
        pub unsafe fn version(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setVersion:)]
        pub unsafe fn setVersion(&self, version: Option<&NSString>);

        #[method(isStandalone)]
        pub unsafe fn isStandalone(&self) -> bool;

        #[method(setStandalone:)]
        pub unsafe fn setStandalone(&self, standalone: bool);

        #[method(documentContentKind)]
        pub unsafe fn documentContentKind(&self) -> NSXMLDocumentContentKind;

        #[method(setDocumentContentKind:)]
        pub unsafe fn setDocumentContentKind(
            &self,
            document_content_kind: NSXMLDocumentContentKind,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other MIMEType)]
        pub unsafe fn MIMEType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMIMEType:)]
        pub unsafe fn setMIMEType(&self, mime_type: Option<&NSString>);

        #[cfg(feature = "Foundation_NSXMLDTD")]
        #[method_id(@__retain_semantics Other DTD)]
        pub unsafe fn DTD(&self) -> Option<Id<NSXMLDTD>>;

        #[cfg(feature = "Foundation_NSXMLDTD")]
        #[method(setDTD:)]
        pub unsafe fn setDTD(&self, dtd: Option<&NSXMLDTD>);

        #[cfg(feature = "Foundation_NSXMLElement")]
        #[method(setRootElement:)]
        pub unsafe fn setRootElement(&self, root: &NSXMLElement);

        #[cfg(feature = "Foundation_NSXMLElement")]
        #[method_id(@__retain_semantics Other rootElement)]
        pub unsafe fn rootElement(&self) -> Option<Id<NSXMLElement>>;

        #[method(insertChild:atIndex:)]
        pub unsafe fn insertChild_atIndex(&self, child: &NSXMLNode, index: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(insertChildren:atIndex:)]
        pub unsafe fn insertChildren_atIndex(
            &self,
            children: &NSArray<NSXMLNode>,
            index: NSUInteger,
        );

        #[method(removeChildAtIndex:)]
        pub unsafe fn removeChildAtIndex(&self, index: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setChildren:)]
        pub unsafe fn setChildren(&self, children: Option<&NSArray<NSXMLNode>>);

        #[method(addChild:)]
        pub unsafe fn addChild(&self, child: &NSXMLNode);

        #[method(replaceChildAtIndex:withNode:)]
        pub unsafe fn replaceChildAtIndex_withNode(&self, index: NSUInteger, node: &NSXMLNode);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other XMLData)]
        pub unsafe fn XMLData(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other XMLDataWithOptions:)]
        pub unsafe fn XMLDataWithOptions(&self, options: NSXMLNodeOptions) -> Id<NSData>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other objectByApplyingXSLT:arguments:error:_)]
        pub unsafe fn objectByApplyingXSLT_arguments_error(
            &self,
            xslt: &NSData,
            arguments: Option<&NSDictionary<NSString, NSString>>,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other objectByApplyingXSLTString:arguments:error:_)]
        pub unsafe fn objectByApplyingXSLTString_arguments_error(
            &self,
            xslt: &NSString,
            arguments: Option<&NSDictionary<NSString, NSString>>,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other objectByApplyingXSLTAtURL:arguments:error:_)]
        pub unsafe fn objectByApplyingXSLTAtURL_arguments_error(
            &self,
            xslt_url: &NSURL,
            argument: Option<&NSDictionary<NSString, NSString>>,
        ) -> Result<Id<AnyObject>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(validateAndReturnError:_)]
        pub unsafe fn validateAndReturnError(&self) -> Result<(), Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSXMLNode`
    #[cfg(feature = "Foundation_NSXMLDocument")]
    unsafe impl NSXMLDocument {
        #[method_id(@__retain_semantics Init initWithKind:)]
        pub unsafe fn initWithKind(this: Allocated<Self>, kind: NSXMLNodeKind) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithKind:options:)]
        pub unsafe fn initWithKind_options(
            this: Allocated<Self>,
            kind: NSXMLNodeKind,
            options: NSXMLNodeOptions,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSXMLDocument")]
    unsafe impl NSXMLDocument {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
