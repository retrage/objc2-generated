//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZAudioOutputStreamSink")]
    pub struct VZHostAudioOutputStreamSink;

    #[cfg(feature = "VZAudioOutputStreamSink")]
    unsafe impl ClassType for VZHostAudioOutputStreamSink {
        #[inherits(NSObject)]
        type Super = VZAudioOutputStreamSink;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZAudioOutputStreamSink")]
unsafe impl NSObjectProtocol for VZHostAudioOutputStreamSink {}

extern_methods!(
    #[cfg(feature = "VZAudioOutputStreamSink")]
    unsafe impl VZHostAudioOutputStreamSink {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `VZAudioOutputStreamSink`
    #[cfg(feature = "VZAudioOutputStreamSink")]
    unsafe impl VZHostAudioOutputStreamSink {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
