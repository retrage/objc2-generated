//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MPMediaPlaylistAttribute {
        MPMediaPlaylistAttributeNone = 0,
        MPMediaPlaylistAttributeOnTheGo = 1 << 0,
        MPMediaPlaylistAttributeSmart = 1 << 1,
        MPMediaPlaylistAttributeGenius = 1 << 2,
    }
);

extern_static!(MPMediaPlaylistPropertyPersistentID: &'static NSString);

extern_static!(MPMediaPlaylistPropertyCloudGlobalID: &'static NSString);

extern_static!(MPMediaPlaylistPropertyName: &'static NSString);

extern_static!(MPMediaPlaylistPropertyPlaylistAttributes: &'static NSString);

extern_static!(MPMediaPlaylistPropertySeedItems: &'static NSString);

extern_static!(MPMediaPlaylistPropertyDescriptionText: &'static NSString);

extern_static!(MPMediaPlaylistPropertyAuthorDisplayName: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
    pub struct MPMediaPlaylist;

    #[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
    unsafe impl ClassType for MPMediaPlaylist {
        #[inherits(MPMediaEntity, NSObject)]
        type Super = MPMediaItemCollection;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
unsafe impl NSCoding for MPMediaPlaylist {}

#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
unsafe impl NSObjectProtocol for MPMediaPlaylist {}

#[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
unsafe impl NSSecureCoding for MPMediaPlaylist {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
    unsafe impl MPMediaPlaylist {
        #[method(persistentID)]
        pub unsafe fn persistentID(&self) -> MPMediaEntityPersistentID;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other cloudGlobalID)]
        pub unsafe fn cloudGlobalID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSString>>;

        #[method(playlistAttributes)]
        pub unsafe fn playlistAttributes(&self) -> MPMediaPlaylistAttribute;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MediaPlayer_MPMediaItem"))]
        #[method_id(@__retain_semantics Other seedItems)]
        pub unsafe fn seedItems(&self) -> Option<Id<NSArray<MPMediaItem>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other descriptionText)]
        pub unsafe fn descriptionText(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other authorDisplayName)]
        pub unsafe fn authorDisplayName(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(addItemWithProductID:completionHandler:)]
        pub unsafe fn addItemWithProductID_completionHandler(
            &self,
            product_id: &NSString,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "MediaPlayer_MPMediaItem"
        ))]
        #[method(addMediaItems:completionHandler:)]
        pub unsafe fn addMediaItems_completionHandler(
            &self,
            media_items: &NSArray<MPMediaItem>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `MPMediaItemCollection`
    #[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
    unsafe impl MPMediaPlaylist {
        #[cfg(all(feature = "Foundation_NSArray", feature = "MediaPlayer_MPMediaItem"))]
        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(
            this: Allocated<Self>,
            items: &NSArray<MPMediaItem>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MediaPlayer_MPMediaPlaylist")]
    unsafe impl MPMediaPlaylist {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaPlaylistCreationMetadata")]
    pub struct MPMediaPlaylistCreationMetadata;

    #[cfg(feature = "MediaPlayer_MPMediaPlaylistCreationMetadata")]
    unsafe impl ClassType for MPMediaPlaylistCreationMetadata {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaPlaylistCreationMetadata")]
unsafe impl NSObjectProtocol for MPMediaPlaylistCreationMetadata {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaPlaylistCreationMetadata")]
    unsafe impl MPMediaPlaylistCreationMetadata {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:)]
        pub unsafe fn initWithName(this: Allocated<Self>, name: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other authorDisplayName)]
        pub unsafe fn authorDisplayName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAuthorDisplayName:)]
        pub unsafe fn setAuthorDisplayName(&self, author_display_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other descriptionText)]
        pub unsafe fn descriptionText(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDescriptionText:)]
        pub unsafe fn setDescriptionText(&self, description_text: &NSString);
    }
);
