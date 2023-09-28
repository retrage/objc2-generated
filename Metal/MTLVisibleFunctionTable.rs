//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLVisibleFunctionTableDescriptor")]
    pub struct MTLVisibleFunctionTableDescriptor;

    #[cfg(feature = "Metal_MTLVisibleFunctionTableDescriptor")]
    unsafe impl ClassType for MTLVisibleFunctionTableDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLVisibleFunctionTableDescriptor")]
unsafe impl NSCopying for MTLVisibleFunctionTableDescriptor {}

#[cfg(feature = "Metal_MTLVisibleFunctionTableDescriptor")]
unsafe impl NSObjectProtocol for MTLVisibleFunctionTableDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLVisibleFunctionTableDescriptor")]
    unsafe impl MTLVisibleFunctionTableDescriptor {
        #[method_id(@__retain_semantics Other visibleFunctionTableDescriptor)]
        pub unsafe fn visibleFunctionTableDescriptor() -> Id<MTLVisibleFunctionTableDescriptor>;

        #[method(functionCount)]
        pub unsafe fn functionCount(&self) -> NSUInteger;

        #[method(setFunctionCount:)]
        pub unsafe fn setFunctionCount(&self, function_count: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLVisibleFunctionTableDescriptor")]
    unsafe impl MTLVisibleFunctionTableDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait MTLVisibleFunctionTable: MTLResource {
        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[method(setFunction:atIndex:)]
        unsafe fn setFunction_atIndex(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunctionHandle>>,
            index: NSUInteger,
        );

        #[method(setFunctions:withRange:)]
        unsafe fn setFunctions_withRange(
            &self,
            functions: NonNull<*const ProtocolObject<dyn MTLFunctionHandle>>,
            range: NSRange,
        );
    }

    unsafe impl ProtocolType for dyn MTLVisibleFunctionTable {}
);
