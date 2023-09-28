//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::SoundAnalysis::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "SoundAnalysis_SNClassifySoundRequest")]
    pub struct SNClassifySoundRequest;

    #[cfg(feature = "SoundAnalysis_SNClassifySoundRequest")]
    unsafe impl ClassType for SNClassifySoundRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "SoundAnalysis_SNClassifySoundRequest")]
unsafe impl NSObjectProtocol for SNClassifySoundRequest {}

#[cfg(feature = "SoundAnalysis_SNClassifySoundRequest")]
unsafe impl SNRequest for SNClassifySoundRequest {}

extern_methods!(
    #[cfg(feature = "SoundAnalysis_SNClassifySoundRequest")]
    unsafe impl SNClassifySoundRequest {
        #[method(overlapFactor)]
        pub unsafe fn overlapFactor(&self) -> c_double;

        #[method(setOverlapFactor:)]
        pub unsafe fn setOverlapFactor(&self, overlap_factor: c_double);

        #[cfg(feature = "SoundAnalysis_SNTimeDurationConstraint")]
        #[method_id(@__retain_semantics Other windowDurationConstraint)]
        pub unsafe fn windowDurationConstraint(&self) -> Id<SNTimeDurationConstraint>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other knownClassifications)]
        pub unsafe fn knownClassifications(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "CoreML_MLModel", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Init initWithMLModel:error:_)]
        pub unsafe fn initWithMLModel_error(
            this: Allocated<Self>,
            ml_model: &MLModel,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Init initWithClassifierIdentifier:error:_)]
        pub unsafe fn initWithClassifierIdentifier_error(
            this: Allocated<Self>,
            classifier_identifier: &SNClassifierIdentifier,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
