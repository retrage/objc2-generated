//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum SKPaymentTransactionState {
        SKPaymentTransactionStatePurchasing = 0,
        SKPaymentTransactionStatePurchased = 1,
        SKPaymentTransactionStateFailed = 2,
        SKPaymentTransactionStateRestored = 3,
        SKPaymentTransactionStateDeferred = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKPaymentTransaction")]
    pub struct SKPaymentTransaction;

    #[cfg(feature = "StoreKit_SKPaymentTransaction")]
    unsafe impl ClassType for SKPaymentTransaction {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKPaymentTransaction")]
unsafe impl Send for SKPaymentTransaction {}

#[cfg(feature = "StoreKit_SKPaymentTransaction")]
unsafe impl Sync for SKPaymentTransaction {}

#[cfg(feature = "StoreKit_SKPaymentTransaction")]
unsafe impl NSObjectProtocol for SKPaymentTransaction {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKPaymentTransaction")]
    unsafe impl SKPaymentTransaction {
        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError>>;

        #[method_id(@__retain_semantics Other originalTransaction)]
        pub unsafe fn originalTransaction(&self) -> Option<Id<SKPaymentTransaction>>;

        #[cfg(feature = "StoreKit_SKPayment")]
        #[method_id(@__retain_semantics Other payment)]
        pub unsafe fn payment(&self) -> Id<SKPayment>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "StoreKit_SKDownload"))]
        #[deprecated = "Hosted content is no longer supported"]
        #[method_id(@__retain_semantics Other downloads)]
        pub unsafe fn downloads(&self) -> Id<NSArray<SKDownload>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other transactionDate)]
        pub unsafe fn transactionDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other transactionIdentifier)]
        pub unsafe fn transactionIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSData")]
        #[deprecated]
        #[method_id(@__retain_semantics Other transactionReceipt)]
        pub unsafe fn transactionReceipt(&self) -> Option<Id<NSData>>;

        #[method(transactionState)]
        pub unsafe fn transactionState(&self) -> SKPaymentTransactionState;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "StoreKit_SKPaymentTransaction")]
    unsafe impl SKPaymentTransaction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
