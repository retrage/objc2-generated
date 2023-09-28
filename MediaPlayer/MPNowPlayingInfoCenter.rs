//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MPNowPlayingInfoMediaType {
        MPNowPlayingInfoMediaTypeNone = 0,
        MPNowPlayingInfoMediaTypeAudio = 1,
        MPNowPlayingInfoMediaTypeVideo = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MPNowPlayingPlaybackState {
        MPNowPlayingPlaybackStateUnknown = 0,
        MPNowPlayingPlaybackStatePlaying = 1,
        MPNowPlayingPlaybackStatePaused = 2,
        MPNowPlayingPlaybackStateStopped = 3,
        MPNowPlayingPlaybackStateInterrupted = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
    pub struct MPNowPlayingInfoCenter;

    #[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
    unsafe impl ClassType for MPNowPlayingInfoCenter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
unsafe impl NSObjectProtocol for MPNowPlayingInfoCenter {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPNowPlayingInfoCenter")]
    unsafe impl MPNowPlayingInfoCenter {
        #[method_id(@__retain_semantics Other defaultCenter)]
        pub unsafe fn defaultCenter() -> Id<MPNowPlayingInfoCenter>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other nowPlayingInfo)]
        pub unsafe fn nowPlayingInfo(&self) -> Option<Id<NSDictionary<NSString, AnyObject>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setNowPlayingInfo:)]
        pub unsafe fn setNowPlayingInfo(
            &self,
            now_playing_info: Option<&NSDictionary<NSString, AnyObject>>,
        );

        #[method(playbackState)]
        pub unsafe fn playbackState(&self) -> MPNowPlayingPlaybackState;

        #[method(setPlaybackState:)]
        pub unsafe fn setPlaybackState(&self, playback_state: MPNowPlayingPlaybackState);
    }
);

extern_static!(MPNowPlayingInfoPropertyElapsedPlaybackTime: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyPlaybackRate: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyDefaultPlaybackRate: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyPlaybackQueueIndex: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyPlaybackQueueCount: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyChapterNumber: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyChapterCount: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyIsLiveStream: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyAvailableLanguageOptions: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyCurrentLanguageOptions: &'static NSString);

extern_static!(MPNowPlayingInfoCollectionIdentifier: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyExternalContentIdentifier: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyExternalUserProfileIdentifier: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyServiceIdentifier: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyPlaybackProgress: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyMediaType: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyAssetURL: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyCurrentPlaybackDate: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyAdTimeRanges: &'static NSString);

extern_static!(MPNowPlayingInfoPropertyCreditsStartTime: &'static NSString);
