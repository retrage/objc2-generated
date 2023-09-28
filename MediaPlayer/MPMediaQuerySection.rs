//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaQuerySection")]
    pub struct MPMediaQuerySection;

    #[cfg(feature = "MediaPlayer_MPMediaQuerySection")]
    unsafe impl ClassType for MPMediaQuerySection {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaQuerySection")]
unsafe impl NSCoding for MPMediaQuerySection {}

#[cfg(feature = "MediaPlayer_MPMediaQuerySection")]
unsafe impl NSCopying for MPMediaQuerySection {}

#[cfg(feature = "MediaPlayer_MPMediaQuerySection")]
unsafe impl NSObjectProtocol for MPMediaQuerySection {}

#[cfg(feature = "MediaPlayer_MPMediaQuerySection")]
unsafe impl NSSecureCoding for MPMediaQuerySection {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaQuerySection")]
    unsafe impl MPMediaQuerySection {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[method(range)]
        pub unsafe fn range(&self) -> NSRange;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPMediaQuerySection")]
    unsafe impl MPMediaQuerySection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
