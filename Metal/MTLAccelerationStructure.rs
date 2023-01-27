//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLAccelerationStructureUsage {
        MTLAccelerationStructureUsageNone = 0,
        MTLAccelerationStructureUsageRefit = 1 << 0,
        MTLAccelerationStructureUsagePreferFastBuild = 1 << 1,
        MTLAccelerationStructureUsageExtendedLimits = 1 << 2,
    }
);

ns_options!(
    #[underlying(u32)]
    pub enum MTLAccelerationStructureInstanceOptions {
        MTLAccelerationStructureInstanceOptionNone = 0,
        MTLAccelerationStructureInstanceOptionDisableTriangleCulling = 1 << 0,
        MTLAccelerationStructureInstanceOptionTriangleFrontFacingWindingCounterClockwise = 1 << 1,
        MTLAccelerationStructureInstanceOptionOpaque = 1 << 2,
        MTLAccelerationStructureInstanceOptionNonOpaque = 1 << 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
    pub struct MTLAccelerationStructureDescriptor;

    #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
    unsafe impl ClassType for MTLAccelerationStructureDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
unsafe impl NSObjectProtocol for MTLAccelerationStructureDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
    unsafe impl MTLAccelerationStructureDescriptor {
        #[method(usage)]
        pub unsafe fn usage(&self) -> MTLAccelerationStructureUsage;

        #[method(setUsage:)]
        pub unsafe fn setUsage(&self, usage: MTLAccelerationStructureUsage);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAccelerationStructureGeometryDescriptor")]
    pub struct MTLAccelerationStructureGeometryDescriptor;

    #[cfg(feature = "Metal_MTLAccelerationStructureGeometryDescriptor")]
    unsafe impl ClassType for MTLAccelerationStructureGeometryDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLAccelerationStructureGeometryDescriptor")]
unsafe impl NSObjectProtocol for MTLAccelerationStructureGeometryDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureGeometryDescriptor")]
    unsafe impl MTLAccelerationStructureGeometryDescriptor {
        #[method(intersectionFunctionTableOffset)]
        pub unsafe fn intersectionFunctionTableOffset(&self) -> NSUInteger;

        #[method(setIntersectionFunctionTableOffset:)]
        pub fn setIntersectionFunctionTableOffset(
            &self,
            intersection_function_table_offset: NSUInteger,
        );

        #[method(opaque)]
        pub unsafe fn opaque(&self) -> bool;

        #[method(setOpaque:)]
        pub unsafe fn setOpaque(&self, opaque: bool);

        #[method(allowDuplicateIntersectionFunctionInvocation)]
        pub unsafe fn allowDuplicateIntersectionFunctionInvocation(&self) -> bool;

        #[method(setAllowDuplicateIntersectionFunctionInvocation:)]
        pub unsafe fn setAllowDuplicateIntersectionFunctionInvocation(
            &self,
            allow_duplicate_intersection_function_invocation: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other primitiveDataBuffer)]
        pub unsafe fn primitiveDataBuffer(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>, Shared>>;

        #[method(setPrimitiveDataBuffer:)]
        pub fn setPrimitiveDataBuffer(
            &self,
            primitive_data_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        #[method(primitiveDataBufferOffset)]
        pub unsafe fn primitiveDataBufferOffset(&self) -> NSUInteger;

        #[method(setPrimitiveDataBufferOffset:)]
        pub unsafe fn setPrimitiveDataBufferOffset(&self, primitive_data_buffer_offset: NSUInteger);

        #[method(primitiveDataStride)]
        pub unsafe fn primitiveDataStride(&self) -> NSUInteger;

        #[method(setPrimitiveDataStride:)]
        pub fn setPrimitiveDataStride(&self, primitive_data_stride: NSUInteger);

        #[method(primitiveDataElementSize)]
        pub unsafe fn primitiveDataElementSize(&self) -> NSUInteger;

        #[method(setPrimitiveDataElementSize:)]
        pub fn setPrimitiveDataElementSize(&self, primitive_data_element_size: NSUInteger);
    }
);

ns_enum!(
    #[underlying(u32)]
    pub enum MTLMotionBorderMode {
        MTLMotionBorderModeClamp = 0,
        MTLMotionBorderModeVanish = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLPrimitiveAccelerationStructureDescriptor")]
    pub struct MTLPrimitiveAccelerationStructureDescriptor;

    #[cfg(feature = "Metal_MTLPrimitiveAccelerationStructureDescriptor")]
    unsafe impl ClassType for MTLPrimitiveAccelerationStructureDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureDescriptor;
    }
);

#[cfg(feature = "Metal_MTLPrimitiveAccelerationStructureDescriptor")]
unsafe impl NSObjectProtocol for MTLPrimitiveAccelerationStructureDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLPrimitiveAccelerationStructureDescriptor")]
    unsafe impl MTLPrimitiveAccelerationStructureDescriptor {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLAccelerationStructureGeometryDescriptor"
        ))]
        #[method_id(@__retain_semantics Other geometryDescriptors)]
        pub unsafe fn geometryDescriptors(
            &self,
        ) -> Option<Id<NSArray<MTLAccelerationStructureGeometryDescriptor>, Shared>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLAccelerationStructureGeometryDescriptor"
        ))]
        #[method(setGeometryDescriptors:)]
        pub fn setGeometryDescriptors(
            &self,
            geometry_descriptors: Option<&NSArray<MTLAccelerationStructureGeometryDescriptor>>,
        );

        #[method(motionStartBorderMode)]
        pub unsafe fn motionStartBorderMode(&self) -> MTLMotionBorderMode;

        #[method(setMotionStartBorderMode:)]
        pub unsafe fn setMotionStartBorderMode(
            &self,
            motion_start_border_mode: MTLMotionBorderMode,
        );

        #[method(motionEndBorderMode)]
        pub unsafe fn motionEndBorderMode(&self) -> MTLMotionBorderMode;

        #[method(setMotionEndBorderMode:)]
        pub unsafe fn setMotionEndBorderMode(&self, motion_end_border_mode: MTLMotionBorderMode);

        #[method(motionStartTime)]
        pub unsafe fn motionStartTime(&self) -> c_float;

        #[method(setMotionStartTime:)]
        pub unsafe fn setMotionStartTime(&self, motion_start_time: c_float);

        #[method(motionEndTime)]
        pub unsafe fn motionEndTime(&self) -> c_float;

        #[method(setMotionEndTime:)]
        pub unsafe fn setMotionEndTime(&self, motion_end_time: c_float);

        #[method(motionKeyframeCount)]
        pub unsafe fn motionKeyframeCount(&self) -> NSUInteger;

        #[method(setMotionKeyframeCount:)]
        pub unsafe fn setMotionKeyframeCount(&self, motion_keyframe_count: NSUInteger);

        #[method_id(@__retain_semantics Other descriptor)]
        pub fn descriptor() -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAccelerationStructureTriangleGeometryDescriptor")]
    pub struct MTLAccelerationStructureTriangleGeometryDescriptor;

    #[cfg(feature = "Metal_MTLAccelerationStructureTriangleGeometryDescriptor")]
    unsafe impl ClassType for MTLAccelerationStructureTriangleGeometryDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureGeometryDescriptor;
    }
);

#[cfg(feature = "Metal_MTLAccelerationStructureTriangleGeometryDescriptor")]
unsafe impl NSObjectProtocol for MTLAccelerationStructureTriangleGeometryDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureTriangleGeometryDescriptor")]
    unsafe impl MTLAccelerationStructureTriangleGeometryDescriptor {
        #[method_id(@__retain_semantics Other vertexBuffer)]
        pub unsafe fn vertexBuffer(&self) -> Option<Id<ProtocolObject<dyn MTLBuffer>, Shared>>;

        #[method(setVertexBuffer:)]
        pub fn setVertexBuffer(&self, vertex_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        #[method(vertexBufferOffset)]
        pub unsafe fn vertexBufferOffset(&self) -> NSUInteger;

        #[method(setVertexBufferOffset:)]
        pub unsafe fn setVertexBufferOffset(&self, vertex_buffer_offset: NSUInteger);

        #[method(vertexFormat)]
        pub unsafe fn vertexFormat(&self) -> MTLAttributeFormat;

        #[method(setVertexFormat:)]
        pub unsafe fn setVertexFormat(&self, vertex_format: MTLAttributeFormat);

        #[method(vertexStride)]
        pub unsafe fn vertexStride(&self) -> NSUInteger;

        #[method(setVertexStride:)]
        pub fn setVertexStride(&self, vertex_stride: NSUInteger);

        #[method_id(@__retain_semantics Other indexBuffer)]
        pub unsafe fn indexBuffer(&self) -> Option<Id<ProtocolObject<dyn MTLBuffer>, Shared>>;

        #[method(setIndexBuffer:)]
        pub fn setIndexBuffer(&self, index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        #[method(indexBufferOffset)]
        pub unsafe fn indexBufferOffset(&self) -> NSUInteger;

        #[method(setIndexBufferOffset:)]
        pub unsafe fn setIndexBufferOffset(&self, index_buffer_offset: NSUInteger);

        #[method(indexType)]
        pub unsafe fn indexType(&self) -> MTLIndexType;

        #[method(setIndexType:)]
        pub unsafe fn setIndexType(&self, index_type: MTLIndexType);

        #[method(triangleCount)]
        pub unsafe fn triangleCount(&self) -> NSUInteger;

        #[method(setTriangleCount:)]
        pub fn setTriangleCount(&self, triangle_count: NSUInteger);

        #[method_id(@__retain_semantics Other transformationMatrixBuffer)]
        pub unsafe fn transformationMatrixBuffer(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>, Shared>>;

        #[method(setTransformationMatrixBuffer:)]
        pub unsafe fn setTransformationMatrixBuffer(
            &self,
            transformation_matrix_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        #[method(transformationMatrixBufferOffset)]
        pub unsafe fn transformationMatrixBufferOffset(&self) -> NSUInteger;

        #[method(setTransformationMatrixBufferOffset:)]
        pub unsafe fn setTransformationMatrixBufferOffset(
            &self,
            transformation_matrix_buffer_offset: NSUInteger,
        );

        #[method_id(@__retain_semantics Other descriptor)]
        pub fn descriptor() -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAccelerationStructureBoundingBoxGeometryDescriptor")]
    pub struct MTLAccelerationStructureBoundingBoxGeometryDescriptor;

    #[cfg(feature = "Metal_MTLAccelerationStructureBoundingBoxGeometryDescriptor")]
    unsafe impl ClassType for MTLAccelerationStructureBoundingBoxGeometryDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureGeometryDescriptor;
    }
);

#[cfg(feature = "Metal_MTLAccelerationStructureBoundingBoxGeometryDescriptor")]
unsafe impl NSObjectProtocol for MTLAccelerationStructureBoundingBoxGeometryDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureBoundingBoxGeometryDescriptor")]
    unsafe impl MTLAccelerationStructureBoundingBoxGeometryDescriptor {
        #[method_id(@__retain_semantics Other boundingBoxBuffer)]
        pub unsafe fn boundingBoxBuffer(&self)
            -> Option<Id<ProtocolObject<dyn MTLBuffer>, Shared>>;

        #[method(setBoundingBoxBuffer:)]
        pub fn setBoundingBoxBuffer(
            &self,
            bounding_box_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        #[method(boundingBoxBufferOffset)]
        pub unsafe fn boundingBoxBufferOffset(&self) -> NSUInteger;

        #[method(setBoundingBoxBufferOffset:)]
        pub unsafe fn setBoundingBoxBufferOffset(&self, bounding_box_buffer_offset: NSUInteger);

        #[method(boundingBoxStride)]
        pub unsafe fn boundingBoxStride(&self) -> NSUInteger;

        #[method(setBoundingBoxStride:)]
        pub unsafe fn setBoundingBoxStride(&self, bounding_box_stride: NSUInteger);

        #[method(boundingBoxCount)]
        pub unsafe fn boundingBoxCount(&self) -> NSUInteger;

        #[method(setBoundingBoxCount:)]
        pub fn setBoundingBoxCount(&self, bounding_box_count: NSUInteger);

        #[method_id(@__retain_semantics Other descriptor)]
        pub fn descriptor() -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLMotionKeyframeData")]
    pub struct MTLMotionKeyframeData;

    #[cfg(feature = "Metal_MTLMotionKeyframeData")]
    unsafe impl ClassType for MTLMotionKeyframeData {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLMotionKeyframeData")]
unsafe impl NSObjectProtocol for MTLMotionKeyframeData {}

extern_methods!(
    #[cfg(feature = "Metal_MTLMotionKeyframeData")]
    unsafe impl MTLMotionKeyframeData {
        #[method_id(@__retain_semantics Other buffer)]
        pub unsafe fn buffer(&self) -> Option<Id<ProtocolObject<dyn MTLBuffer>, Shared>>;

        #[method(setBuffer:)]
        pub unsafe fn setBuffer(&self, buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        #[method(offset)]
        pub unsafe fn offset(&self) -> NSUInteger;

        #[method(setOffset:)]
        pub unsafe fn setOffset(&self, offset: NSUInteger);

        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAccelerationStructureMotionTriangleGeometryDescriptor")]
    pub struct MTLAccelerationStructureMotionTriangleGeometryDescriptor;

    #[cfg(feature = "Metal_MTLAccelerationStructureMotionTriangleGeometryDescriptor")]
    unsafe impl ClassType for MTLAccelerationStructureMotionTriangleGeometryDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureGeometryDescriptor;
    }
);

#[cfg(feature = "Metal_MTLAccelerationStructureMotionTriangleGeometryDescriptor")]
unsafe impl NSObjectProtocol for MTLAccelerationStructureMotionTriangleGeometryDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureMotionTriangleGeometryDescriptor")]
    unsafe impl MTLAccelerationStructureMotionTriangleGeometryDescriptor {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLMotionKeyframeData"
        ))]
        #[method_id(@__retain_semantics Other vertexBuffers)]
        pub unsafe fn vertexBuffers(&self) -> Id<NSArray<MTLMotionKeyframeData>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLMotionKeyframeData"
        ))]
        #[method(setVertexBuffers:)]
        pub unsafe fn setVertexBuffers(&self, vertex_buffers: &NSArray<MTLMotionKeyframeData>);

        #[method(vertexFormat)]
        pub unsafe fn vertexFormat(&self) -> MTLAttributeFormat;

        #[method(setVertexFormat:)]
        pub unsafe fn setVertexFormat(&self, vertex_format: MTLAttributeFormat);

        #[method(vertexStride)]
        pub unsafe fn vertexStride(&self) -> NSUInteger;

        #[method(setVertexStride:)]
        pub unsafe fn setVertexStride(&self, vertex_stride: NSUInteger);

        #[method_id(@__retain_semantics Other indexBuffer)]
        pub unsafe fn indexBuffer(&self) -> Option<Id<ProtocolObject<dyn MTLBuffer>, Shared>>;

        #[method(setIndexBuffer:)]
        pub unsafe fn setIndexBuffer(&self, index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        #[method(indexBufferOffset)]
        pub unsafe fn indexBufferOffset(&self) -> NSUInteger;

        #[method(setIndexBufferOffset:)]
        pub unsafe fn setIndexBufferOffset(&self, index_buffer_offset: NSUInteger);

        #[method(indexType)]
        pub unsafe fn indexType(&self) -> MTLIndexType;

        #[method(setIndexType:)]
        pub unsafe fn setIndexType(&self, index_type: MTLIndexType);

        #[method(triangleCount)]
        pub unsafe fn triangleCount(&self) -> NSUInteger;

        #[method(setTriangleCount:)]
        pub unsafe fn setTriangleCount(&self, triangle_count: NSUInteger);

        #[method_id(@__retain_semantics Other transformationMatrixBuffer)]
        pub unsafe fn transformationMatrixBuffer(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>, Shared>>;

        #[method(setTransformationMatrixBuffer:)]
        pub unsafe fn setTransformationMatrixBuffer(
            &self,
            transformation_matrix_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        #[method(transformationMatrixBufferOffset)]
        pub unsafe fn transformationMatrixBufferOffset(&self) -> NSUInteger;

        #[method(setTransformationMatrixBufferOffset:)]
        pub unsafe fn setTransformationMatrixBufferOffset(
            &self,
            transformation_matrix_buffer_offset: NSUInteger,
        );

        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor() -> Id<Self, Shared>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor")]
    pub struct MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor;

    #[cfg(feature = "Metal_MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor")]
    unsafe impl ClassType for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureGeometryDescriptor;
    }
);

#[cfg(feature = "Metal_MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor")]
unsafe impl NSObjectProtocol for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor")]
    unsafe impl MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLMotionKeyframeData"
        ))]
        #[method_id(@__retain_semantics Other boundingBoxBuffers)]
        pub unsafe fn boundingBoxBuffers(&self) -> Id<NSArray<MTLMotionKeyframeData>, Shared>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLMotionKeyframeData"
        ))]
        #[method(setBoundingBoxBuffers:)]
        pub unsafe fn setBoundingBoxBuffers(
            &self,
            bounding_box_buffers: &NSArray<MTLMotionKeyframeData>,
        );

        #[method(boundingBoxStride)]
        pub unsafe fn boundingBoxStride(&self) -> NSUInteger;

        #[method(setBoundingBoxStride:)]
        pub unsafe fn setBoundingBoxStride(&self, bounding_box_stride: NSUInteger);

        #[method(boundingBoxCount)]
        pub unsafe fn boundingBoxCount(&self) -> NSUInteger;

        #[method(setBoundingBoxCount:)]
        pub unsafe fn setBoundingBoxCount(&self, bounding_box_count: NSUInteger);

        #[method_id(@__retain_semantics Other descriptor)]
        pub unsafe fn descriptor() -> Id<Self, Shared>;
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLAccelerationStructureInstanceDescriptor {
        pub transformationMatrix: MTLPackedFloat4x3,
        pub options: MTLAccelerationStructureInstanceOptions,
        pub mask: u32,
        pub intersectionFunctionTableOffset: u32,
        pub accelerationStructureIndex: u32,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLAccelerationStructureUserIDInstanceDescriptor {
        pub transformationMatrix: MTLPackedFloat4x3,
        pub options: MTLAccelerationStructureInstanceOptions,
        pub mask: u32,
        pub intersectionFunctionTableOffset: u32,
        pub accelerationStructureIndex: u32,
        pub userID: u32,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLAccelerationStructureInstanceDescriptorType {
        MTLAccelerationStructureInstanceDescriptorTypeDefault = 0,
        MTLAccelerationStructureInstanceDescriptorTypeUserID = 1,
        MTLAccelerationStructureInstanceDescriptorTypeMotion = 2,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLAccelerationStructureMotionInstanceDescriptor {
        pub options: MTLAccelerationStructureInstanceOptions,
        pub mask: u32,
        pub intersectionFunctionTableOffset: u32,
        pub accelerationStructureIndex: u32,
        pub userID: u32,
        pub motionTransformsStartIndex: u32,
        pub motionTransformsCount: u32,
        pub motionStartBorderMode: MTLMotionBorderMode,
        pub motionEndBorderMode: MTLMotionBorderMode,
        pub motionStartTime: c_float,
        pub motionEndTime: c_float,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLInstanceAccelerationStructureDescriptor")]
    pub struct MTLInstanceAccelerationStructureDescriptor;

    #[cfg(feature = "Metal_MTLInstanceAccelerationStructureDescriptor")]
    unsafe impl ClassType for MTLInstanceAccelerationStructureDescriptor {
        #[inherits(NSObject)]
        type Super = MTLAccelerationStructureDescriptor;
    }
);

#[cfg(feature = "Metal_MTLInstanceAccelerationStructureDescriptor")]
unsafe impl NSObjectProtocol for MTLInstanceAccelerationStructureDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLInstanceAccelerationStructureDescriptor")]
    unsafe impl MTLInstanceAccelerationStructureDescriptor {
        #[method_id(@__retain_semantics Other instanceDescriptorBuffer)]
        pub unsafe fn instanceDescriptorBuffer(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>, Shared>>;

        #[method(setInstanceDescriptorBuffer:)]
        pub fn setInstanceDescriptorBuffer(
            &self,
            instance_descriptor_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        #[method(instanceDescriptorBufferOffset)]
        pub unsafe fn instanceDescriptorBufferOffset(&self) -> NSUInteger;

        #[method(setInstanceDescriptorBufferOffset:)]
        pub unsafe fn setInstanceDescriptorBufferOffset(
            &self,
            instance_descriptor_buffer_offset: NSUInteger,
        );

        #[method(instanceDescriptorStride)]
        pub unsafe fn instanceDescriptorStride(&self) -> NSUInteger;

        #[method(setInstanceDescriptorStride:)]
        pub unsafe fn setInstanceDescriptorStride(&self, instance_descriptor_stride: NSUInteger);

        #[method(instanceCount)]
        pub unsafe fn instanceCount(&self) -> NSUInteger;

        #[method(setInstanceCount:)]
        pub fn setInstanceCount(&self, instance_count: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other instancedAccelerationStructures)]
        pub unsafe fn instancedAccelerationStructures(
            &self,
        ) -> Option<Id<NSArray<ProtocolObject<dyn MTLAccelerationStructure>>, Shared>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setInstancedAccelerationStructures:)]
        pub fn setInstancedAccelerationStructures(
            &self,
            instanced_acceleration_structures: Option<
                &NSArray<ProtocolObject<dyn MTLAccelerationStructure>>,
            >,
        );

        #[method(instanceDescriptorType)]
        pub unsafe fn instanceDescriptorType(
            &self,
        ) -> MTLAccelerationStructureInstanceDescriptorType;

        #[method(setInstanceDescriptorType:)]
        pub unsafe fn setInstanceDescriptorType(
            &self,
            instance_descriptor_type: MTLAccelerationStructureInstanceDescriptorType,
        );

        #[method_id(@__retain_semantics Other motionTransformBuffer)]
        pub unsafe fn motionTransformBuffer(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>, Shared>>;

        #[method(setMotionTransformBuffer:)]
        pub unsafe fn setMotionTransformBuffer(
            &self,
            motion_transform_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        #[method(motionTransformBufferOffset)]
        pub unsafe fn motionTransformBufferOffset(&self) -> NSUInteger;

        #[method(setMotionTransformBufferOffset:)]
        pub unsafe fn setMotionTransformBufferOffset(
            &self,
            motion_transform_buffer_offset: NSUInteger,
        );

        #[method(motionTransformCount)]
        pub unsafe fn motionTransformCount(&self) -> NSUInteger;

        #[method(setMotionTransformCount:)]
        pub unsafe fn setMotionTransformCount(&self, motion_transform_count: NSUInteger);

        #[method_id(@__retain_semantics Other descriptor)]
        pub fn descriptor() -> Id<Self, Shared>;
    }
);

extern_protocol!(
    pub unsafe trait MTLAccelerationStructure: MTLResource {
        #[method(size)]
        unsafe fn size(&self) -> NSUInteger;

        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;
    }

    unsafe impl ProtocolType for dyn MTLAccelerationStructure {}
);
