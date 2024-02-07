//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(u32)]
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    pub enum NSOpenGLGlobalOption {
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLGOFormatCacheSize = 501,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLGOClearFormatCache = 502,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLGORetainRenderers = 503,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLGOUseBuildCache = 506,
        #[deprecated]
        NSOpenGLGOResetLibrary = 504,
    }
);

#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAllRenderers: c_uint = 1;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFATripleBuffer: c_uint = 3;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFADoubleBuffer: c_uint = 5;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAuxBuffers: c_uint = 7;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAColorSize: c_uint = 8;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAlphaSize: c_uint = 11;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFADepthSize: c_uint = 12;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAStencilSize: c_uint = 13;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAccumSize: c_uint = 14;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAMinimumPolicy: c_uint = 51;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAMaximumPolicy: c_uint = 52;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFASampleBuffers: c_uint = 55;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFASamples: c_uint = 56;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAuxDepthStencil: c_uint = 57;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAColorFloat: c_uint = 58;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAMultisample: c_uint = 59;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFASupersample: c_uint = 60;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFASampleAlpha: c_uint = 61;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFARendererID: c_uint = 70;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFANoRecovery: c_uint = 72;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAccelerated: c_uint = 73;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAClosestPolicy: c_uint = 74;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFABackingStore: c_uint = 76;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAScreenMask: c_uint = 84;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAllowOfflineRenderers: c_uint = 96;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAAcceleratedCompute: c_uint = 97;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAOpenGLProfile: c_uint = 99;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLPFAVirtualScreenCount: c_uint = 128;
#[deprecated]
pub const NSOpenGLPFAStereo: c_uint = 6;
#[deprecated]
pub const NSOpenGLPFAOffScreen: c_uint = 53;
#[deprecated]
pub const NSOpenGLPFAFullScreen: c_uint = 54;
#[deprecated]
pub const NSOpenGLPFASingleRenderer: c_uint = 71;
#[deprecated]
pub const NSOpenGLPFARobust: c_uint = 75;
#[deprecated]
pub const NSOpenGLPFAMPSafe: c_uint = 78;
#[deprecated]
pub const NSOpenGLPFAWindow: c_uint = 80;
#[deprecated]
pub const NSOpenGLPFAMultiScreen: c_uint = 81;
#[deprecated]
pub const NSOpenGLPFACompliant: c_uint = 83;
#[deprecated]
pub const NSOpenGLPFAPixelBuffer: c_uint = 90;
#[deprecated]
pub const NSOpenGLPFARemotePixelBuffer: c_uint = 91;

pub type NSOpenGLPixelFormatAttribute = u32;

#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLProfileVersionLegacy: c_uint = 0x1000;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLProfileVersion3_2Core: c_uint = 0x3200;
#[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
pub const NSOpenGLProfileVersion4_1Core: c_uint = 0x4100;

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
    pub enum NSOpenGLContextParameter {
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterSwapInterval = 222,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterSurfaceOrder = 235,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterSurfaceOpacity = 236,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterSurfaceBackingSize = 304,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterReclaimResources = 308,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterCurrentRendererID = 309,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterGPUVertexProcessing = 310,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterGPUFragmentProcessing = 311,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterHasDrawable = 314,
        #[deprecated = "OpenGL API deprecated; please use Metal and MetalKit.  (Define GL_SILENCE_DEPRECATION to silence these warnings.)"]
        NSOpenGLContextParameterMPSwapsInFlight = 315,
        #[deprecated]
        NSOpenGLContextParameterSwapRectangle = 200,
        #[deprecated]
        NSOpenGLContextParameterSwapRectangleEnable = 201,
        #[deprecated]
        NSOpenGLContextParameterRasterizationEnable = 221,
        #[deprecated]
        NSOpenGLContextParameterStateValidation = 301,
        #[deprecated]
        NSOpenGLContextParameterSurfaceSurfaceVolatile = 306,
    }
);

extern_static!(NSOpenGLCPSwapInterval: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterSwapInterval.0));

extern_static!(NSOpenGLCPSurfaceOrder: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterSurfaceOrder.0));

extern_static!(NSOpenGLCPSurfaceOpacity: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterSurfaceOpacity.0));

extern_static!(NSOpenGLCPSurfaceBackingSize: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterSurfaceBackingSize.0));

extern_static!(NSOpenGLCPReclaimResources: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterReclaimResources.0));

extern_static!(NSOpenGLCPCurrentRendererID: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterCurrentRendererID.0));

extern_static!(NSOpenGLCPGPUVertexProcessing: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterGPUVertexProcessing.0));

extern_static!(NSOpenGLCPGPUFragmentProcessing: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterGPUFragmentProcessing.0));

extern_static!(NSOpenGLCPHasDrawable: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterHasDrawable.0));

extern_static!(NSOpenGLCPMPSwapsInFlight: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterMPSwapsInFlight.0));

extern_static!(NSOpenGLCPSwapRectangle: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterSwapRectangle.0));

extern_static!(NSOpenGLCPSwapRectangleEnable: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterSwapRectangleEnable.0));

extern_static!(NSOpenGLCPRasterizationEnable: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterRasterizationEnable.0));

extern_static!(NSOpenGLCPStateValidation: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterStateValidation.0));

extern_static!(NSOpenGLCPSurfaceSurfaceVolatile: NSOpenGLContextParameter = NSOpenGLContextParameter(NSOpenGLContextParameterSurfaceSurfaceVolatile.0));
