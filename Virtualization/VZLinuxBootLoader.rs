//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;
use objc2_foundation::*;

use crate::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "VZBootLoader")]
    pub struct VZLinuxBootLoader;

    #[cfg(feature = "VZBootLoader")]
    unsafe impl ClassType for VZLinuxBootLoader {
        #[inherits(NSObject)]
        type Super = VZBootLoader;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "VZBootLoader")]
unsafe impl NSCopying for VZLinuxBootLoader {}

#[cfg(feature = "VZBootLoader")]
unsafe impl NSObjectProtocol for VZLinuxBootLoader {}

extern_methods!(
    #[cfg(feature = "VZBootLoader")]
    unsafe impl VZLinuxBootLoader {
        #[method_id(@__retain_semantics Init initWithKernelURL:)]
        pub unsafe fn initWithKernelURL(this: Allocated<Self>, kernel_url: &NSURL) -> Id<Self>;

        #[method_id(@__retain_semantics Other kernelURL)]
        pub unsafe fn kernelURL(&self) -> Id<NSURL>;

        #[method(setKernelURL:)]
        pub unsafe fn setKernelURL(&self, kernel_url: &NSURL);

        #[method_id(@__retain_semantics Other commandLine)]
        pub unsafe fn commandLine(&self) -> Id<NSString>;

        #[method(setCommandLine:)]
        pub unsafe fn setCommandLine(&self, command_line: &NSString);

        #[method_id(@__retain_semantics Other initialRamdiskURL)]
        pub unsafe fn initialRamdiskURL(&self) -> Option<Id<NSURL>>;

        #[method(setInitialRamdiskURL:)]
        pub unsafe fn setInitialRamdiskURL(&self, initial_ramdisk_url: Option<&NSURL>);
    }
);

extern_methods!(
    /// Methods declared on superclass `VZBootLoader`
    #[cfg(feature = "VZBootLoader")]
    unsafe impl VZLinuxBootLoader {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);
