//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMusicPlayerControllerQueue")]
    pub struct MPMusicPlayerControllerQueue;

    #[cfg(feature = "MediaPlayer_MPMusicPlayerControllerQueue")]
    unsafe impl ClassType for MPMusicPlayerControllerQueue {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMusicPlayerControllerQueue")]
unsafe impl NSObjectProtocol for MPMusicPlayerControllerQueue {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMusicPlayerControllerQueue")]
    unsafe impl MPMusicPlayerControllerQueue {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MediaPlayer_MPMediaItem"))]
        #[method_id(@__retain_semantics Other items)]
        pub unsafe fn items(&self) -> Id<NSArray<MPMediaItem>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMusicPlayerControllerMutableQueue")]
    pub struct MPMusicPlayerControllerMutableQueue;

    #[cfg(feature = "MediaPlayer_MPMusicPlayerControllerMutableQueue")]
    unsafe impl ClassType for MPMusicPlayerControllerMutableQueue {
        #[inherits(NSObject)]
        type Super = MPMusicPlayerControllerQueue;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMusicPlayerControllerMutableQueue")]
unsafe impl NSObjectProtocol for MPMusicPlayerControllerMutableQueue {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMusicPlayerControllerMutableQueue")]
    unsafe impl MPMusicPlayerControllerMutableQueue {
        #[cfg(all(
            feature = "MediaPlayer_MPMediaItem",
            feature = "MediaPlayer_MPMusicPlayerQueueDescriptor"
        ))]
        #[method(insertQueueDescriptor:afterItem:)]
        pub unsafe fn insertQueueDescriptor_afterItem(
            &self,
            queue_descriptor: &MPMusicPlayerQueueDescriptor,
            after_item: Option<&MPMediaItem>,
        );

        #[cfg(feature = "MediaPlayer_MPMediaItem")]
        #[method(removeItem:)]
        pub unsafe fn removeItem(&self, item: &MPMediaItem);
    }
);

extern_methods!(
    /// Methods declared on superclass `MPMusicPlayerControllerQueue`
    #[cfg(feature = "MediaPlayer_MPMusicPlayerControllerMutableQueue")]
    unsafe impl MPMusicPlayerControllerMutableQueue {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
    pub struct MPMusicPlayerApplicationController;

    #[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
    unsafe impl ClassType for MPMusicPlayerApplicationController {
        #[inherits(NSObject)]
        type Super = MPMusicPlayerController;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
unsafe impl MPMediaPlayback for MPMusicPlayerApplicationController {}

#[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
unsafe impl NSObjectProtocol for MPMusicPlayerApplicationController {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
    unsafe impl MPMusicPlayerApplicationController {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "MediaPlayer_MPMusicPlayerControllerMutableQueue",
            feature = "MediaPlayer_MPMusicPlayerControllerQueue"
        ))]
        #[method(performQueueTransaction:completionHandler:)]
        pub unsafe fn performQueueTransaction_completionHandler(
            &self,
            queue_transaction: &Block<(NonNull<MPMusicPlayerControllerMutableQueue>,), ()>,
            completion_handler: &Block<(NonNull<MPMusicPlayerControllerQueue>, *mut NSError), ()>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPMusicPlayerController`
    #[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
    unsafe impl MPMusicPlayerApplicationController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_static!(MPMusicPlayerControllerQueueDidChangeNotification: &'static NSString);
