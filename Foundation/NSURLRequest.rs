//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSURLRequestCachePolicy {
        NSURLRequestUseProtocolCachePolicy = 0,
        NSURLRequestReloadIgnoringLocalCacheData = 1,
        NSURLRequestReloadIgnoringLocalAndRemoteCacheData = 4,
        NSURLRequestReloadIgnoringCacheData = NSURLRequestReloadIgnoringLocalCacheData,
        NSURLRequestReturnCacheDataElseLoad = 2,
        NSURLRequestReturnCacheDataDontLoad = 3,
        NSURLRequestReloadRevalidatingCacheData = 5,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSURLRequestNetworkServiceType {
        NSURLNetworkServiceTypeDefault = 0,
        #[deprecated = "Use PushKit for VoIP control purposes"]
        NSURLNetworkServiceTypeVoIP = 1,
        NSURLNetworkServiceTypeVideo = 2,
        NSURLNetworkServiceTypeBackground = 3,
        NSURLNetworkServiceTypeVoice = 4,
        NSURLNetworkServiceTypeResponsiveData = 6,
        NSURLNetworkServiceTypeAVStreaming = 8,
        NSURLNetworkServiceTypeResponsiveAV = 9,
        NSURLNetworkServiceTypeCallSignaling = 11,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSURLRequestAttribution {
        NSURLRequestAttributionDeveloper = 0,
        NSURLRequestAttributionUser = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLRequest")]
    pub struct NSURLRequest;

    #[cfg(feature = "Foundation_NSURLRequest")]
    unsafe impl ClassType for NSURLRequest {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableURLRequest>;
    }
);

#[cfg(feature = "Foundation_NSURLRequest")]
unsafe impl NSCoding for NSURLRequest {}

#[cfg(feature = "Foundation_NSURLRequest")]
unsafe impl NSCopying for NSURLRequest {}

#[cfg(feature = "Foundation_NSURLRequest")]
unsafe impl NSMutableCopying for NSURLRequest {}

#[cfg(feature = "Foundation_NSURLRequest")]
unsafe impl NSObjectProtocol for NSURLRequest {}

#[cfg(feature = "Foundation_NSURLRequest")]
unsafe impl NSSecureCoding for NSURLRequest {}

extern_methods!(
    #[cfg(feature = "Foundation_NSURLRequest")]
    unsafe impl NSURLRequest {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other requestWithURL:)]
        pub unsafe fn requestWithURL(url: &NSURL) -> Id<Self>;

        #[method(supportsSecureCoding)]
        pub unsafe fn supportsSecureCoding() -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other requestWithURL:cachePolicy:timeoutInterval:)]
        pub unsafe fn requestWithURL_cachePolicy_timeoutInterval(
            url: &NSURL,
            cache_policy: NSURLRequestCachePolicy,
            timeout_interval: NSTimeInterval,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:cachePolicy:timeoutInterval:)]
        pub unsafe fn initWithURL_cachePolicy_timeoutInterval(
            this: Allocated<Self>,
            url: &NSURL,
            cache_policy: NSURLRequestCachePolicy,
            timeout_interval: NSTimeInterval,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[method(cachePolicy)]
        pub unsafe fn cachePolicy(&self) -> NSURLRequestCachePolicy;

        #[method(timeoutInterval)]
        pub unsafe fn timeoutInterval(&self) -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other mainDocumentURL)]
        pub unsafe fn mainDocumentURL(&self) -> Option<Id<NSURL>>;

        #[method(networkServiceType)]
        pub unsafe fn networkServiceType(&self) -> NSURLRequestNetworkServiceType;

        #[method(allowsCellularAccess)]
        pub unsafe fn allowsCellularAccess(&self) -> bool;

        #[method(allowsExpensiveNetworkAccess)]
        pub unsafe fn allowsExpensiveNetworkAccess(&self) -> bool;

        #[method(allowsConstrainedNetworkAccess)]
        pub unsafe fn allowsConstrainedNetworkAccess(&self) -> bool;

        #[method(assumesHTTP3Capable)]
        pub unsafe fn assumesHTTP3Capable(&self) -> bool;

        #[method(attribution)]
        pub unsafe fn attribution(&self) -> NSURLRequestAttribution;

        #[method(requiresDNSSECValidation)]
        pub unsafe fn requiresDNSSECValidation(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSURLRequest")]
    unsafe impl NSURLRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLRequest")]
    pub struct NSMutableURLRequest;

    #[cfg(feature = "Foundation_NSURLRequest")]
    unsafe impl ClassType for NSMutableURLRequest {
        #[inherits(NSObject)]
        type Super = NSURLRequest;
        type Mutability = MutableWithImmutableSuperclass<NSURLRequest>;
    }
);

#[cfg(feature = "Foundation_NSMutableURLRequest")]
unsafe impl NSCoding for NSMutableURLRequest {}

#[cfg(feature = "Foundation_NSMutableURLRequest")]
unsafe impl NSCopying for NSMutableURLRequest {}

#[cfg(feature = "Foundation_NSMutableURLRequest")]
unsafe impl NSMutableCopying for NSMutableURLRequest {}

#[cfg(feature = "Foundation_NSMutableURLRequest")]
unsafe impl NSObjectProtocol for NSMutableURLRequest {}

#[cfg(feature = "Foundation_NSMutableURLRequest")]
unsafe impl NSSecureCoding for NSMutableURLRequest {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMutableURLRequest")]
    unsafe impl NSMutableURLRequest {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setURL:)]
        pub unsafe fn setURL(&mut self, url: Option<&NSURL>);

        #[method(cachePolicy)]
        pub unsafe fn cachePolicy(&self) -> NSURLRequestCachePolicy;

        #[method(setCachePolicy:)]
        pub unsafe fn setCachePolicy(&mut self, cache_policy: NSURLRequestCachePolicy);

        #[method(timeoutInterval)]
        pub unsafe fn timeoutInterval(&self) -> NSTimeInterval;

        #[method(setTimeoutInterval:)]
        pub unsafe fn setTimeoutInterval(&mut self, timeout_interval: NSTimeInterval);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other mainDocumentURL)]
        pub unsafe fn mainDocumentURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setMainDocumentURL:)]
        pub unsafe fn setMainDocumentURL(&mut self, main_document_url: Option<&NSURL>);

        #[method(networkServiceType)]
        pub unsafe fn networkServiceType(&self) -> NSURLRequestNetworkServiceType;

        #[method(setNetworkServiceType:)]
        pub unsafe fn setNetworkServiceType(
            &mut self,
            network_service_type: NSURLRequestNetworkServiceType,
        );

        #[method(allowsCellularAccess)]
        pub unsafe fn allowsCellularAccess(&self) -> bool;

        #[method(setAllowsCellularAccess:)]
        pub unsafe fn setAllowsCellularAccess(&mut self, allows_cellular_access: bool);

        #[method(allowsExpensiveNetworkAccess)]
        pub unsafe fn allowsExpensiveNetworkAccess(&self) -> bool;

        #[method(setAllowsExpensiveNetworkAccess:)]
        pub unsafe fn setAllowsExpensiveNetworkAccess(
            &mut self,
            allows_expensive_network_access: bool,
        );

        #[method(allowsConstrainedNetworkAccess)]
        pub unsafe fn allowsConstrainedNetworkAccess(&self) -> bool;

        #[method(setAllowsConstrainedNetworkAccess:)]
        pub unsafe fn setAllowsConstrainedNetworkAccess(
            &mut self,
            allows_constrained_network_access: bool,
        );

        #[method(assumesHTTP3Capable)]
        pub unsafe fn assumesHTTP3Capable(&self) -> bool;

        #[method(setAssumesHTTP3Capable:)]
        pub unsafe fn setAssumesHTTP3Capable(&mut self, assumes_http3_capable: bool);

        #[method(attribution)]
        pub unsafe fn attribution(&self) -> NSURLRequestAttribution;

        #[method(setAttribution:)]
        pub unsafe fn setAttribution(&mut self, attribution: NSURLRequestAttribution);

        #[method(requiresDNSSECValidation)]
        pub unsafe fn requiresDNSSECValidation(&self) -> bool;

        #[method(setRequiresDNSSECValidation:)]
        pub unsafe fn setRequiresDNSSECValidation(&mut self, requires_dnssec_validation: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSURLRequest`
    #[cfg(feature = "Foundation_NSMutableURLRequest")]
    unsafe impl NSMutableURLRequest {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other requestWithURL:)]
        pub unsafe fn requestWithURL(url: &NSURL) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other requestWithURL:cachePolicy:timeoutInterval:)]
        pub unsafe fn requestWithURL_cachePolicy_timeoutInterval(
            url: &NSURL,
            cache_policy: NSURLRequestCachePolicy,
            timeout_interval: NSTimeInterval,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Allocated<Self>, url: &NSURL) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:cachePolicy:timeoutInterval:)]
        pub unsafe fn initWithURL_cachePolicy_timeoutInterval(
            this: Allocated<Self>,
            url: &NSURL,
            cache_policy: NSURLRequestCachePolicy,
            timeout_interval: NSTimeInterval,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMutableURLRequest")]
    unsafe impl NSMutableURLRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// NSHTTPURLRequest
    #[cfg(feature = "Foundation_NSURLRequest")]
    unsafe impl NSURLRequest {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other HTTPMethod)]
        pub unsafe fn HTTPMethod(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other allHTTPHeaderFields)]
        pub unsafe fn allHTTPHeaderFields(&self) -> Option<Id<NSDictionary<NSString, NSString>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForHTTPHeaderField:)]
        pub unsafe fn valueForHTTPHeaderField(&self, field: &NSString) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other HTTPBody)]
        pub unsafe fn HTTPBody(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSInputStream")]
        #[method_id(@__retain_semantics Other HTTPBodyStream)]
        pub unsafe fn HTTPBodyStream(&self) -> Option<Id<NSInputStream>>;

        #[method(HTTPShouldHandleCookies)]
        pub unsafe fn HTTPShouldHandleCookies(&self) -> bool;

        #[method(HTTPShouldUsePipelining)]
        pub unsafe fn HTTPShouldUsePipelining(&self) -> bool;
    }
);

extern_methods!(
    /// NSMutableHTTPURLRequest
    #[cfg(feature = "Foundation_NSMutableURLRequest")]
    unsafe impl NSMutableURLRequest {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other HTTPMethod)]
        pub unsafe fn HTTPMethod(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHTTPMethod:)]
        pub unsafe fn setHTTPMethod(&mut self, http_method: &NSString);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other allHTTPHeaderFields)]
        pub unsafe fn allHTTPHeaderFields(&self) -> Option<Id<NSDictionary<NSString, NSString>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setAllHTTPHeaderFields:)]
        pub unsafe fn setAllHTTPHeaderFields(
            &mut self,
            all_http_header_fields: Option<&NSDictionary<NSString, NSString>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:forHTTPHeaderField:)]
        pub unsafe fn setValue_forHTTPHeaderField(
            &mut self,
            value: Option<&NSString>,
            field: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(addValue:forHTTPHeaderField:)]
        pub unsafe fn addValue_forHTTPHeaderField(&mut self, value: &NSString, field: &NSString);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other HTTPBody)]
        pub unsafe fn HTTPBody(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setHTTPBody:)]
        pub unsafe fn setHTTPBody(&mut self, http_body: Option<&NSData>);

        #[cfg(feature = "Foundation_NSInputStream")]
        #[method_id(@__retain_semantics Other HTTPBodyStream)]
        pub unsafe fn HTTPBodyStream(&self) -> Option<Id<NSInputStream>>;

        #[cfg(feature = "Foundation_NSInputStream")]
        #[method(setHTTPBodyStream:)]
        pub unsafe fn setHTTPBodyStream(&mut self, http_body_stream: Option<&NSInputStream>);

        #[method(HTTPShouldHandleCookies)]
        pub unsafe fn HTTPShouldHandleCookies(&self) -> bool;

        #[method(setHTTPShouldHandleCookies:)]
        pub unsafe fn setHTTPShouldHandleCookies(&mut self, http_should_handle_cookies: bool);

        #[method(HTTPShouldUsePipelining)]
        pub unsafe fn HTTPShouldUsePipelining(&self) -> bool;

        #[method(setHTTPShouldUsePipelining:)]
        pub unsafe fn setHTTPShouldUsePipelining(&mut self, http_should_use_pipelining: bool);
    }
);
