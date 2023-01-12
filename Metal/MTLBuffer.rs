//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_protocol!(
    pub struct MTLBuffer;

    unsafe impl ProtocolType for MTLBuffer {
        #[method(length)]
        pub fn length(&self) -> NSUInteger;

        #[method(contents)]
        pub fn contents(&self) -> NonNull<c_void>;

        #[method(didModifyRange:)]
        pub fn didModifyRange(&self, range: NSRange);

        #[cfg(feature = "Metal_MTLTextureDescriptor")]
        #[method_id(@__retain_semantics New newTextureWithDescriptor:offset:bytesPerRow:)]
        pub fn newTextureWithDescriptor_offset_bytesPerRow(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: NSUInteger,
            bytesPerRow: NSUInteger,
        ) -> Option<Id<MTLTexture, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(addDebugMarker:range:)]
        pub fn addDebugMarker_range(&self, marker: &NSString, range: NSRange);

        #[method(removeAllDebugMarkers)]
        pub fn removeAllDebugMarkers(&self);

        #[method_id(@__retain_semantics Other remoteStorageBuffer)]
        pub fn remoteStorageBuffer(&self) -> Option<Id<MTLBuffer, Shared>>;

        #[method_id(@__retain_semantics New newRemoteBufferViewForDevice:)]
        pub fn newRemoteBufferViewForDevice(
            &self,
            device: &MTLDevice,
        ) -> Option<Id<MTLBuffer, Shared>>;

        #[method(gpuAddress)]
        pub fn gpuAddress(&self) -> u64;
    }
);
