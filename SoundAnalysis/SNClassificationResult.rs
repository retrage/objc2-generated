//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::SoundAnalysis::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "SoundAnalysis_SNClassification")]
    pub struct SNClassification;

    #[cfg(feature = "SoundAnalysis_SNClassification")]
    unsafe impl ClassType for SNClassification {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "SoundAnalysis_SNClassification")]
unsafe impl NSObjectProtocol for SNClassification {}

extern_methods!(
    #[cfg(feature = "SoundAnalysis_SNClassification")]
    unsafe impl SNClassification {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[method(confidence)]
        pub unsafe fn confidence(&self) -> c_double;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "SoundAnalysis_SNClassificationResult")]
    pub struct SNClassificationResult;

    #[cfg(feature = "SoundAnalysis_SNClassificationResult")]
    unsafe impl ClassType for SNClassificationResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "SoundAnalysis_SNClassificationResult")]
unsafe impl NSObjectProtocol for SNClassificationResult {}

#[cfg(feature = "SoundAnalysis_SNClassificationResult")]
unsafe impl SNResult for SNClassificationResult {}

extern_methods!(
    #[cfg(feature = "SoundAnalysis_SNClassificationResult")]
    unsafe impl SNClassificationResult {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "SoundAnalysis_SNClassification"
        ))]
        #[method_id(@__retain_semantics Other classifications)]
        pub unsafe fn classifications(&self) -> Id<NSArray<SNClassification>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "SoundAnalysis_SNClassification"
        ))]
        #[method_id(@__retain_semantics Other classificationForIdentifier:)]
        pub unsafe fn classificationForIdentifier(
            &self,
            identifier: &NSString,
        ) -> Option<Id<SNClassification>>;
    }
);
