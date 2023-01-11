//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_fn!(
    pub unsafe fn MTLCreateSystemDefaultDevice() -> *mut MTLDevice;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSArray")]
    pub unsafe fn MTLCopyAllDevices() -> NonNull<NSArray<MTLDevice>>;
);

typed_enum!(
    pub type MTLDeviceNotificationName = NSString;
);

extern_static!(MTLDeviceWasAddedNotification: &'static MTLDeviceNotificationName);

extern_static!(MTLDeviceRemovalRequestedNotification: &'static MTLDeviceNotificationName);

extern_static!(MTLDeviceWasRemovedNotification: &'static MTLDeviceNotificationName);

pub type MTLDeviceNotificationHandler =
    *mut Block<(NonNull<MTLDevice>, NonNull<MTLDeviceNotificationName>), ()>;

extern_fn!(
    pub unsafe fn MTLRemoveDeviceObserver(observer: &NSObject);
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLFeatureSet {
        MTLFeatureSet_iOS_GPUFamily1_v1 = 0,
        MTLFeatureSet_iOS_GPUFamily2_v1 = 1,
        MTLFeatureSet_iOS_GPUFamily1_v2 = 2,
        MTLFeatureSet_iOS_GPUFamily2_v2 = 3,
        MTLFeatureSet_iOS_GPUFamily3_v1 = 4,
        MTLFeatureSet_iOS_GPUFamily1_v3 = 5,
        MTLFeatureSet_iOS_GPUFamily2_v3 = 6,
        MTLFeatureSet_iOS_GPUFamily3_v2 = 7,
        MTLFeatureSet_iOS_GPUFamily1_v4 = 8,
        MTLFeatureSet_iOS_GPUFamily2_v4 = 9,
        MTLFeatureSet_iOS_GPUFamily3_v3 = 10,
        MTLFeatureSet_iOS_GPUFamily4_v1 = 11,
        MTLFeatureSet_iOS_GPUFamily1_v5 = 12,
        MTLFeatureSet_iOS_GPUFamily2_v5 = 13,
        MTLFeatureSet_iOS_GPUFamily3_v4 = 14,
        MTLFeatureSet_iOS_GPUFamily4_v2 = 15,
        MTLFeatureSet_iOS_GPUFamily5_v1 = 16,
        MTLFeatureSet_macOS_GPUFamily1_v1 = 10000,
        MTLFeatureSet_OSX_GPUFamily1_v1 = MTLFeatureSet_macOS_GPUFamily1_v1,
        MTLFeatureSet_macOS_GPUFamily1_v2 = 10001,
        MTLFeatureSet_OSX_GPUFamily1_v2 = MTLFeatureSet_macOS_GPUFamily1_v2,
        MTLFeatureSet_macOS_ReadWriteTextureTier2 = 10002,
        MTLFeatureSet_OSX_ReadWriteTextureTier2 = MTLFeatureSet_macOS_ReadWriteTextureTier2,
        MTLFeatureSet_macOS_GPUFamily1_v3 = 10003,
        MTLFeatureSet_macOS_GPUFamily1_v4 = 10004,
        MTLFeatureSet_macOS_GPUFamily2_v1 = 10005,
        MTLFeatureSet_tvOS_GPUFamily1_v1 = 30000,
        MTLFeatureSet_TVOS_GPUFamily1_v1 = MTLFeatureSet_tvOS_GPUFamily1_v1,
        MTLFeatureSet_tvOS_GPUFamily1_v2 = 30001,
        MTLFeatureSet_tvOS_GPUFamily1_v3 = 30002,
        MTLFeatureSet_tvOS_GPUFamily2_v1 = 30003,
        MTLFeatureSet_tvOS_GPUFamily1_v4 = 30004,
        MTLFeatureSet_tvOS_GPUFamily2_v2 = 30005,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLGPUFamily {
        MTLGPUFamilyApple1 = 1001,
        MTLGPUFamilyApple2 = 1002,
        MTLGPUFamilyApple3 = 1003,
        MTLGPUFamilyApple4 = 1004,
        MTLGPUFamilyApple5 = 1005,
        MTLGPUFamilyApple6 = 1006,
        MTLGPUFamilyApple7 = 1007,
        MTLGPUFamilyMac1 = 2001,
        MTLGPUFamilyMac2 = 2002,
        MTLGPUFamilyCommon1 = 3001,
        MTLGPUFamilyCommon2 = 3002,
        MTLGPUFamilyCommon3 = 3003,
        MTLGPUFamilyMacCatalyst1 = 4001,
        MTLGPUFamilyMacCatalyst2 = 4002,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLDeviceLocation {
        MTLDeviceLocationBuiltIn = 0,
        MTLDeviceLocationSlot = 1,
        MTLDeviceLocationExternal = 2,
        MTLDeviceLocationUnspecified = NSUIntegerMax as _,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLPipelineOption {
        MTLPipelineOptionNone = 0,
        MTLPipelineOptionArgumentInfo = 1 << 0,
        MTLPipelineOptionBufferTypeInfo = 1 << 1,
        MTLPipelineOptionFailOnBinaryArchiveMiss = 1 << 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLReadWriteTextureTier {
        MTLReadWriteTextureTierNone = 0,
        MTLReadWriteTextureTier1 = 1,
        MTLReadWriteTextureTier2 = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLArgumentBuffersTier {
        MTLArgumentBuffersTier1 = 0,
        MTLArgumentBuffersTier2 = 1,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLSparseTextureRegionAlignmentMode {
        MTLSparseTextureRegionAlignmentModeOutward = 0,
        MTLSparseTextureRegionAlignmentModeInward = 1,
    }
);

extern_struct!(
    pub struct MTLAccelerationStructureSizes {
        pub accelerationStructureSize: NSUInteger,
        pub buildScratchBufferSize: NSUInteger,
        pub refitScratchBufferSize: NSUInteger,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLCounterSamplingPoint {
        MTLCounterSamplingPointAtStageBoundary = 0,
        MTLCounterSamplingPointAtDrawBoundary = 1,
        MTLCounterSamplingPointAtDispatchBoundary = 2,
        MTLCounterSamplingPointAtTileDispatchBoundary = 3,
        MTLCounterSamplingPointAtBlitBoundary = 4,
    }
);

extern_struct!(
    pub struct MTLSizeAndAlign {
        pub size: NSUInteger,
        pub align: NSUInteger,
    }
);

pub type MTLNewLibraryCompletionHandler = *mut Block<(*mut MTLLibrary, *mut NSError), ()>;

pub type MTLNewRenderPipelineStateCompletionHandler =
    *mut Block<(*mut MTLRenderPipelineState, *mut NSError), ()>;

pub type MTLNewRenderPipelineStateWithReflectionCompletionHandler = *mut Block<
    (
        *mut MTLRenderPipelineState,
        *mut MTLRenderPipelineReflection,
        *mut NSError,
    ),
    (),
>;

pub type MTLNewComputePipelineStateCompletionHandler =
    *mut Block<(*mut MTLComputePipelineState, *mut NSError), ()>;

pub type MTLNewComputePipelineStateWithReflectionCompletionHandler = *mut Block<
    (
        *mut MTLComputePipelineState,
        *mut MTLComputePipelineReflection,
        *mut NSError,
    ),
    (),
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLArgumentDescriptor;

    unsafe impl ClassType for MTLArgumentDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLArgumentDescriptor")]
    unsafe impl MTLArgumentDescriptor {
        #[method_id(@__retain_semantics Other argumentDescriptor)]
        pub fn argumentDescriptor() -> Id<MTLArgumentDescriptor, Shared>;

        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MTLDataType;

        #[method(setDataType:)]
        pub fn setDataType(&self, dataType: MTLDataType);

        #[method(index)]
        pub unsafe fn index(&self) -> NSUInteger;

        #[method(setIndex:)]
        pub fn setIndex(&self, index: NSUInteger);

        #[method(arrayLength)]
        pub unsafe fn arrayLength(&self) -> NSUInteger;

        #[method(setArrayLength:)]
        pub unsafe fn setArrayLength(&self, arrayLength: NSUInteger);

        #[method(access)]
        pub unsafe fn access(&self) -> MTLArgumentAccess;

        #[method(setAccess:)]
        pub fn setAccess(&self, access: MTLArgumentAccess);

        #[method(textureType)]
        pub unsafe fn textureType(&self) -> MTLTextureType;

        #[method(setTextureType:)]
        pub fn setTextureType(&self, textureType: MTLTextureType);

        #[method(constantBlockAlignment)]
        pub unsafe fn constantBlockAlignment(&self) -> NSUInteger;

        #[method(setConstantBlockAlignment:)]
        pub unsafe fn setConstantBlockAlignment(&self, constantBlockAlignment: NSUInteger);
    }
);

pub type MTLTimestamp = u64;

extern_protocol!(
    pub struct MTLDevice;

    unsafe impl ProtocolType for MTLDevice {
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Id<NSString, Shared>;

        #[method(registryID)]
        pub fn registryID(&self) -> u64;

        #[method(maxThreadsPerThreadgroup)]
        pub fn maxThreadsPerThreadgroup(&self) -> MTLSize;

        #[method(isLowPower)]
        pub fn isLowPower(&self) -> bool;

        #[method(isHeadless)]
        pub fn isHeadless(&self) -> bool;

        #[method(isRemovable)]
        pub fn isRemovable(&self) -> bool;

        #[method(hasUnifiedMemory)]
        pub fn hasUnifiedMemory(&self) -> bool;

        #[method(recommendedMaxWorkingSetSize)]
        pub fn recommendedMaxWorkingSetSize(&self) -> u64;

        #[method(location)]
        pub fn location(&self) -> MTLDeviceLocation;

        #[method(locationNumber)]
        pub fn locationNumber(&self) -> NSUInteger;

        #[method(maxTransferRate)]
        pub fn maxTransferRate(&self) -> u64;

        #[method(isDepth24Stencil8PixelFormatSupported)]
        pub fn isDepth24Stencil8PixelFormatSupported(&self) -> bool;

        #[method(readWriteTextureSupport)]
        pub fn readWriteTextureSupport(&self) -> MTLReadWriteTextureTier;

        #[method(argumentBuffersSupport)]
        pub fn argumentBuffersSupport(&self) -> MTLArgumentBuffersTier;

        #[method(areRasterOrderGroupsSupported)]
        pub unsafe fn areRasterOrderGroupsSupported(&self) -> bool;

        #[method(supports32BitFloatFiltering)]
        pub fn supports32BitFloatFiltering(&self) -> bool;

        #[method(supports32BitMSAA)]
        pub fn supports32BitMSAA(&self) -> bool;

        #[method(supportsQueryTextureLOD)]
        pub fn supportsQueryTextureLOD(&self) -> bool;

        #[method(supportsBCTextureCompression)]
        pub fn supportsBCTextureCompression(&self) -> bool;

        #[method(supportsPullModelInterpolation)]
        pub fn supportsPullModelInterpolation(&self) -> bool;

        #[method(areBarycentricCoordsSupported)]
        pub unsafe fn areBarycentricCoordsSupported(&self) -> bool;

        #[method(supportsShaderBarycentricCoordinates)]
        pub fn supportsShaderBarycentricCoordinates(&self) -> bool;

        #[method(currentAllocatedSize)]
        pub fn currentAllocatedSize(&self) -> NSUInteger;

        #[method_id(@__retain_semantics New newCommandQueue)]
        pub fn newCommandQueue(&self) -> Option<Id<MTLCommandQueue, Shared>>;

        #[method_id(@__retain_semantics New newCommandQueueWithMaxCommandBufferCount:)]
        pub fn newCommandQueueWithMaxCommandBufferCount(
            &self,
            maxCommandBufferCount: NSUInteger,
        ) -> Option<Id<MTLCommandQueue, Shared>>;

        #[method(heapTextureSizeAndAlignWithDescriptor:)]
        pub fn heapTextureSizeAndAlignWithDescriptor(
            &self,
            desc: &MTLTextureDescriptor,
        ) -> MTLSizeAndAlign;

        #[method(heapBufferSizeAndAlignWithLength:options:)]
        pub fn heapBufferSizeAndAlignWithLength_options(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
        ) -> MTLSizeAndAlign;

        #[method_id(@__retain_semantics New newHeapWithDescriptor:)]
        pub fn newHeapWithDescriptor(
            &self,
            descriptor: &MTLHeapDescriptor,
        ) -> Option<Id<MTLHeap, Shared>>;

        #[method_id(@__retain_semantics New newBufferWithLength:options:)]
        pub fn newBufferWithLength_options(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
        ) -> Option<Id<MTLBuffer, Shared>>;

        #[method_id(@__retain_semantics New newBufferWithBytes:length:options:)]
        pub unsafe fn newBufferWithBytes_length_options(
            &self,
            pointer: NonNull<c_void>,
            length: NSUInteger,
            options: MTLResourceOptions,
        ) -> Option<Id<MTLBuffer, Shared>>;

        #[method_id(@__retain_semantics New newBufferWithBytesNoCopy:length:options:deallocator:)]
        pub unsafe fn newBufferWithBytesNoCopy_length_options_deallocator(
            &self,
            pointer: NonNull<c_void>,
            length: NSUInteger,
            options: MTLResourceOptions,
            deallocator: Option<&Block<(NonNull<c_void>, NSUInteger), ()>>,
        ) -> Option<Id<MTLBuffer, Shared>>;

        #[method_id(@__retain_semantics New newDepthStencilStateWithDescriptor:)]
        pub fn newDepthStencilStateWithDescriptor(
            &self,
            descriptor: &MTLDepthStencilDescriptor,
        ) -> Option<Id<MTLDepthStencilState, Shared>>;

        #[method_id(@__retain_semantics New newTextureWithDescriptor:)]
        pub fn newTextureWithDescriptor(
            &self,
            descriptor: &MTLTextureDescriptor,
        ) -> Option<Id<MTLTexture, Shared>>;

        #[method_id(@__retain_semantics New newSharedTextureWithDescriptor:)]
        pub unsafe fn newSharedTextureWithDescriptor(
            &self,
            descriptor: &MTLTextureDescriptor,
        ) -> Option<Id<MTLTexture, Shared>>;

        #[method_id(@__retain_semantics New newSharedTextureWithHandle:)]
        pub unsafe fn newSharedTextureWithHandle(
            &self,
            sharedHandle: &MTLSharedTextureHandle,
        ) -> Option<Id<MTLTexture, Shared>>;

        #[method_id(@__retain_semantics New newSamplerStateWithDescriptor:)]
        pub fn newSamplerStateWithDescriptor(
            &self,
            descriptor: &MTLSamplerDescriptor,
        ) -> Option<Id<MTLSamplerState, Shared>>;

        #[method_id(@__retain_semantics New newDefaultLibrary)]
        pub fn newDefaultLibrary(&self) -> Option<Id<MTLLibrary, Shared>>;

        #[method_id(@__retain_semantics New newDefaultLibraryWithBundle:error:_)]
        pub unsafe fn newDefaultLibraryWithBundle_error(
            &self,
            bundle: &NSBundle,
        ) -> Result<Id<MTLLibrary, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics New newLibraryWithFile:error:_)]
        pub fn newLibraryWithFile_error(
            &self,
            filepath: &NSString,
        ) -> Result<Id<MTLLibrary, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics New newLibraryWithURL:error:_)]
        pub unsafe fn newLibraryWithURL_error(
            &self,
            url: &NSURL,
        ) -> Result<Id<MTLLibrary, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics New newLibraryWithSource:options:error:_)]
        pub fn newLibraryWithSource_options_error(
            &self,
            source: &NSString,
            options: Option<&MTLCompileOptions>,
        ) -> Result<Id<MTLLibrary, Shared>, Id<NSError, Shared>>;

        #[method(newLibraryWithSource:options:completionHandler:)]
        pub unsafe fn newLibraryWithSource_options_completionHandler(
            &self,
            source: &NSString,
            options: Option<&MTLCompileOptions>,
            completionHandler: MTLNewLibraryCompletionHandler,
        );

        #[method_id(@__retain_semantics New newLibraryWithStitchedDescriptor:error:_)]
        pub unsafe fn newLibraryWithStitchedDescriptor_error(
            &self,
            descriptor: &MTLStitchedLibraryDescriptor,
        ) -> Result<Id<MTLLibrary, Shared>, Id<NSError, Shared>>;

        #[method(newLibraryWithStitchedDescriptor:completionHandler:)]
        pub unsafe fn newLibraryWithStitchedDescriptor_completionHandler(
            &self,
            descriptor: &MTLStitchedLibraryDescriptor,
            completionHandler: MTLNewLibraryCompletionHandler,
        );

        #[method_id(@__retain_semantics New newRenderPipelineStateWithDescriptor:error:_)]
        pub fn newRenderPipelineStateWithDescriptor_error(
            &self,
            descriptor: &MTLRenderPipelineDescriptor,
        ) -> Result<Id<MTLRenderPipelineState, Shared>, Id<NSError, Shared>>;

        #[method(newRenderPipelineStateWithDescriptor:completionHandler:)]
        pub unsafe fn newRenderPipelineStateWithDescriptor_completionHandler(
            &self,
            descriptor: &MTLRenderPipelineDescriptor,
            completionHandler: MTLNewRenderPipelineStateCompletionHandler,
        );

        #[method(newRenderPipelineStateWithDescriptor:options:completionHandler:)]
        pub unsafe fn newRenderPipelineStateWithDescriptor_options_completionHandler(
            &self,
            descriptor: &MTLRenderPipelineDescriptor,
            options: MTLPipelineOption,
            completionHandler: MTLNewRenderPipelineStateWithReflectionCompletionHandler,
        );

        #[method_id(@__retain_semantics New newComputePipelineStateWithFunction:error:_)]
        pub fn newComputePipelineStateWithFunction_error(
            &self,
            computeFunction: &MTLFunction,
        ) -> Result<Id<MTLComputePipelineState, Shared>, Id<NSError, Shared>>;

        #[method(newComputePipelineStateWithFunction:completionHandler:)]
        pub unsafe fn newComputePipelineStateWithFunction_completionHandler(
            &self,
            computeFunction: &MTLFunction,
            completionHandler: MTLNewComputePipelineStateCompletionHandler,
        );

        #[method(newComputePipelineStateWithFunction:options:completionHandler:)]
        pub unsafe fn newComputePipelineStateWithFunction_options_completionHandler(
            &self,
            computeFunction: &MTLFunction,
            options: MTLPipelineOption,
            completionHandler: MTLNewComputePipelineStateWithReflectionCompletionHandler,
        );

        #[method(newComputePipelineStateWithDescriptor:options:completionHandler:)]
        pub unsafe fn newComputePipelineStateWithDescriptor_options_completionHandler(
            &self,
            descriptor: &MTLComputePipelineDescriptor,
            options: MTLPipelineOption,
            completionHandler: MTLNewComputePipelineStateWithReflectionCompletionHandler,
        );

        #[method_id(@__retain_semantics New newFence)]
        pub fn newFence(&self) -> Option<Id<MTLFence, Shared>>;

        #[method(supportsFeatureSet:)]
        pub fn supportsFeatureSet(&self, featureSet: MTLFeatureSet) -> bool;

        #[method(supportsFamily:)]
        pub fn supportsFamily(&self, gpuFamily: MTLGPUFamily) -> bool;

        #[method(supportsTextureSampleCount:)]
        pub fn supportsTextureSampleCount(&self, sampleCount: NSUInteger) -> bool;

        #[method(minimumLinearTextureAlignmentForPixelFormat:)]
        pub fn minimumLinearTextureAlignmentForPixelFormat(
            &self,
            format: MTLPixelFormat,
        ) -> NSUInteger;

        #[method(minimumTextureBufferAlignmentForPixelFormat:)]
        pub fn minimumTextureBufferAlignmentForPixelFormat(
            &self,
            format: MTLPixelFormat,
        ) -> NSUInteger;

        #[method(newRenderPipelineStateWithTileDescriptor:options:completionHandler:)]
        pub unsafe fn newRenderPipelineStateWithTileDescriptor_options_completionHandler(
            &self,
            descriptor: &MTLTileRenderPipelineDescriptor,
            options: MTLPipelineOption,
            completionHandler: MTLNewRenderPipelineStateWithReflectionCompletionHandler,
        );

        #[method(maxThreadgroupMemoryLength)]
        pub fn maxThreadgroupMemoryLength(&self) -> NSUInteger;

        #[method(maxArgumentBufferSamplerCount)]
        pub fn maxArgumentBufferSamplerCount(&self) -> NSUInteger;

        #[method(areProgrammableSamplePositionsSupported)]
        pub unsafe fn areProgrammableSamplePositionsSupported(&self) -> bool;

        #[method(getDefaultSamplePositions:count:)]
        pub unsafe fn getDefaultSamplePositions_count(
            &self,
            positions: NonNull<MTLSamplePosition>,
            count: NSUInteger,
        );

        #[method_id(@__retain_semantics New newArgumentEncoderWithArguments:)]
        pub fn newArgumentEncoderWithArguments(
            &self,
            arguments: &NSArray<MTLArgumentDescriptor>,
        ) -> Option<Id<MTLArgumentEncoder, Shared>>;

        #[method(supportsRasterizationRateMapWithLayerCount:)]
        pub unsafe fn supportsRasterizationRateMapWithLayerCount(
            &self,
            layerCount: NSUInteger,
        ) -> bool;

        #[method_id(@__retain_semantics New newRasterizationRateMapWithDescriptor:)]
        pub unsafe fn newRasterizationRateMapWithDescriptor(
            &self,
            descriptor: &MTLRasterizationRateMapDescriptor,
        ) -> Option<Id<MTLRasterizationRateMap, Shared>>;

        #[method_id(@__retain_semantics New newIndirectCommandBufferWithDescriptor:maxCommandCount:options:)]
        pub unsafe fn newIndirectCommandBufferWithDescriptor_maxCommandCount_options(
            &self,
            descriptor: &MTLIndirectCommandBufferDescriptor,
            maxCount: NSUInteger,
            options: MTLResourceOptions,
        ) -> Option<Id<MTLIndirectCommandBuffer, Shared>>;

        #[method_id(@__retain_semantics New newEvent)]
        pub fn newEvent(&self) -> Option<Id<MTLEvent, Shared>>;

        #[method_id(@__retain_semantics New newSharedEvent)]
        pub fn newSharedEvent(&self) -> Option<Id<MTLSharedEvent, Shared>>;

        #[method_id(@__retain_semantics New newSharedEventWithHandle:)]
        pub unsafe fn newSharedEventWithHandle(
            &self,
            sharedEventHandle: &MTLSharedEventHandle,
        ) -> Option<Id<MTLSharedEvent, Shared>>;

        #[method(peerGroupID)]
        pub unsafe fn peerGroupID(&self) -> u64;

        #[method(peerIndex)]
        pub unsafe fn peerIndex(&self) -> u32;

        #[method(peerCount)]
        pub unsafe fn peerCount(&self) -> u32;

        #[method(sparseTileSizeWithTextureType:pixelFormat:sampleCount:)]
        pub unsafe fn sparseTileSizeWithTextureType_pixelFormat_sampleCount(
            &self,
            textureType: MTLTextureType,
            pixelFormat: MTLPixelFormat,
            sampleCount: NSUInteger,
        ) -> MTLSize;

        #[method(sparseTileSizeInBytes)]
        pub unsafe fn sparseTileSizeInBytes(&self) -> NSUInteger;

        #[optional]
        #[method(convertSparsePixelRegions:toTileRegions:withTileSize:alignmentMode:numRegions:)]
        pub unsafe fn convertSparsePixelRegions_toTileRegions_withTileSize_alignmentMode_numRegions(
            &self,
            pixelRegions: NonNull<MTLRegion>,
            tileRegions: NonNull<MTLRegion>,
            tileSize: MTLSize,
            mode: MTLSparseTextureRegionAlignmentMode,
            numRegions: NSUInteger,
        );

        #[optional]
        #[method(convertSparseTileRegions:toPixelRegions:withTileSize:numRegions:)]
        pub unsafe fn convertSparseTileRegions_toPixelRegions_withTileSize_numRegions(
            &self,
            tileRegions: NonNull<MTLRegion>,
            pixelRegions: NonNull<MTLRegion>,
            tileSize: MTLSize,
            numRegions: NSUInteger,
        );

        #[method(maxBufferLength)]
        pub fn maxBufferLength(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other counterSets)]
        pub unsafe fn counterSets(&self) -> Option<Id<NSArray<MTLCounterSet>, Shared>>;

        #[method_id(@__retain_semantics New newCounterSampleBufferWithDescriptor:error:_)]
        pub unsafe fn newCounterSampleBufferWithDescriptor_error(
            &self,
            descriptor: &MTLCounterSampleBufferDescriptor,
        ) -> Result<Id<MTLCounterSampleBuffer, Shared>, Id<NSError, Shared>>;

        #[method(sampleTimestamps:gpuTimestamp:)]
        pub unsafe fn sampleTimestamps_gpuTimestamp(
            &self,
            cpuTimestamp: NonNull<MTLTimestamp>,
            gpuTimestamp: NonNull<MTLTimestamp>,
        );

        #[method(supportsCounterSampling:)]
        pub fn supportsCounterSampling(&self, samplingPoint: MTLCounterSamplingPoint) -> bool;

        #[method(supportsVertexAmplificationCount:)]
        pub fn supportsVertexAmplificationCount(&self, count: NSUInteger) -> bool;

        #[method(supportsDynamicLibraries)]
        pub fn supportsDynamicLibraries(&self) -> bool;

        #[method(supportsRenderDynamicLibraries)]
        pub unsafe fn supportsRenderDynamicLibraries(&self) -> bool;

        #[method_id(@__retain_semantics New newDynamicLibrary:error:_)]
        pub fn newDynamicLibrary_error(
            &self,
            library: &MTLLibrary,
        ) -> Result<Id<MTLDynamicLibrary, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics New newDynamicLibraryWithURL:error:_)]
        pub fn newDynamicLibraryWithURL_error(
            &self,
            url: &NSURL,
        ) -> Result<Id<MTLDynamicLibrary, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics New newBinaryArchiveWithDescriptor:error:_)]
        pub fn newBinaryArchiveWithDescriptor_error(
            &self,
            descriptor: &MTLBinaryArchiveDescriptor,
        ) -> Result<Id<MTLBinaryArchive, Shared>, Id<NSError, Shared>>;

        #[method(supportsRaytracing)]
        pub fn supportsRaytracing(&self) -> bool;

        #[method(accelerationStructureSizesWithDescriptor:)]
        pub fn accelerationStructureSizesWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
        ) -> MTLAccelerationStructureSizes;

        #[method_id(@__retain_semantics New newAccelerationStructureWithSize:)]
        pub fn newAccelerationStructureWithSize(
            &self,
            size: NSUInteger,
        ) -> Option<Id<MTLAccelerationStructure, Shared>>;

        #[method_id(@__retain_semantics New newAccelerationStructureWithDescriptor:)]
        pub unsafe fn newAccelerationStructureWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
        ) -> Option<Id<MTLAccelerationStructure, Shared>>;

        #[method(supportsFunctionPointers)]
        pub fn supportsFunctionPointers(&self) -> bool;

        #[method(supportsFunctionPointersFromRender)]
        pub unsafe fn supportsFunctionPointersFromRender(&self) -> bool;

        #[method(supportsRaytracingFromRender)]
        pub unsafe fn supportsRaytracingFromRender(&self) -> bool;

        #[method(supportsPrimitiveMotionBlur)]
        pub unsafe fn supportsPrimitiveMotionBlur(&self) -> bool;
    }
);
