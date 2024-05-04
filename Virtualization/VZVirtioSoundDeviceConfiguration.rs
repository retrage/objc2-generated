//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZAudioDeviceConfiguration")]
    pub struct VZVirtioSoundDeviceConfiguration;

    #[cfg(feature = "VZAudioDeviceConfiguration")]
    unsafe impl ClassType for VZVirtioSoundDeviceConfiguration {
        #[inherits(NSObject)]
        type Super = VZAudioDeviceConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZAudioDeviceConfiguration")]
unsafe impl NSCopying for VZVirtioSoundDeviceConfiguration {}

#[cfg(feature = "VZAudioDeviceConfiguration")]
unsafe impl NSObjectProtocol for VZVirtioSoundDeviceConfiguration {}

extern_methods!(
    #[cfg(feature = "VZAudioDeviceConfiguration")]
    unsafe impl VZVirtioSoundDeviceConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "VZVirtioSoundDeviceStreamConfiguration")]
        #[method_id(@__retain_semantics Other streams)]
        pub unsafe fn streams(&self) -> Id<NSArray<VZVirtioSoundDeviceStreamConfiguration>>;

        #[cfg(feature = "VZVirtioSoundDeviceStreamConfiguration")]
        #[method(setStreams:)]
        pub unsafe fn setStreams(&self, streams: &NSArray<VZVirtioSoundDeviceStreamConfiguration>);
    }
);

extern_methods!(
    /// Methods declared on superclass `VZAudioDeviceConfiguration`
    #[cfg(feature = "VZAudioDeviceConfiguration")]
    unsafe impl VZVirtioSoundDeviceConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
