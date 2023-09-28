//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
    pub struct MTLComputePassSampleBufferAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
    unsafe impl ClassType for MTLComputePassSampleBufferAttachmentDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
unsafe impl NSCopying for MTLComputePassSampleBufferAttachmentDescriptor {}

#[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLComputePassSampleBufferAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
    unsafe impl MTLComputePassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Other sampleBuffer)]
        pub unsafe fn sampleBuffer(&self)
            -> Option<Id<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

        #[method(setSampleBuffer:)]
        pub unsafe fn setSampleBuffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
        );

        #[method(startOfEncoderSampleIndex)]
        pub unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfEncoderSampleIndex:)]
        pub unsafe fn setStartOfEncoderSampleIndex(
            &self,
            start_of_encoder_sample_index: NSUInteger,
        );

        #[method(endOfEncoderSampleIndex)]
        pub unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfEncoderSampleIndex:)]
        pub unsafe fn setEndOfEncoderSampleIndex(&self, end_of_encoder_sample_index: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
    unsafe impl MTLComputePassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
    pub struct MTLComputePassSampleBufferAttachmentDescriptorArray;

    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
    unsafe impl ClassType for MTLComputePassSampleBufferAttachmentDescriptorArray {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
unsafe impl NSObjectProtocol for MTLComputePassSampleBufferAttachmentDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
    unsafe impl MTLComputePassSampleBufferAttachmentDescriptorArray {
        #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Id<MTLComputePassSampleBufferAttachmentDescriptor>;

        #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLComputePassSampleBufferAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
    unsafe impl MTLComputePassSampleBufferAttachmentDescriptorArray {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLComputePassDescriptor")]
    pub struct MTLComputePassDescriptor;

    #[cfg(feature = "Metal_MTLComputePassDescriptor")]
    unsafe impl ClassType for MTLComputePassDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLComputePassDescriptor")]
unsafe impl NSCopying for MTLComputePassDescriptor {}

#[cfg(feature = "Metal_MTLComputePassDescriptor")]
unsafe impl NSObjectProtocol for MTLComputePassDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLComputePassDescriptor")]
    unsafe impl MTLComputePassDescriptor {
        #[method_id(@__retain_semantics Other computePassDescriptor)]
        pub unsafe fn computePassDescriptor() -> Id<MTLComputePassDescriptor>;

        #[method(dispatchType)]
        pub unsafe fn dispatchType(&self) -> MTLDispatchType;

        #[method(setDispatchType:)]
        pub unsafe fn setDispatchType(&self, dispatch_type: MTLDispatchType);

        #[cfg(feature = "Metal_MTLComputePassSampleBufferAttachmentDescriptorArray")]
        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub unsafe fn sampleBufferAttachments(
            &self,
        ) -> Id<MTLComputePassSampleBufferAttachmentDescriptorArray>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLComputePassDescriptor")]
    unsafe impl MTLComputePassDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
