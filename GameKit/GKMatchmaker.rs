//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKInviteRecipientResponse {
        GKInviteRecipientResponseAccepted = 0,
        GKInviteRecipientResponseDeclined = 1,
        GKInviteRecipientResponseFailed = 2,
        GKInviteRecipientResponseIncompatible = 3,
        GKInviteRecipientResponseUnableToConnect = 4,
        GKInviteRecipientResponseNoAnswer = 5,
        GKInviteeResponseAccepted = GKInviteRecipientResponseAccepted.0,
        GKInviteeResponseDeclined = GKInviteRecipientResponseDeclined.0,
        GKInviteeResponseFailed = GKInviteRecipientResponseFailed.0,
        GKInviteeResponseIncompatible = GKInviteRecipientResponseIncompatible.0,
        GKInviteeResponseUnableToConnect = GKInviteRecipientResponseUnableToConnect.0,
        GKInviteeResponseNoAnswer = GKInviteRecipientResponseNoAnswer.0,
    }
);

pub type GKInviteeResponse = GKInviteRecipientResponse;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum GKMatchType {
        GKMatchTypePeerToPeer = 0,
        GKMatchTypeHosted = 1,
        GKMatchTypeTurnBased = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKMatchRequest")]
    pub struct GKMatchRequest;

    #[cfg(feature = "GameKit_GKMatchRequest")]
    unsafe impl ClassType for GKMatchRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKMatchRequest")]
unsafe impl NSObjectProtocol for GKMatchRequest {}

extern_methods!(
    #[cfg(feature = "GameKit_GKMatchRequest")]
    unsafe impl GKMatchRequest {
        #[method(minPlayers)]
        pub unsafe fn minPlayers(&self) -> NSUInteger;

        #[method(setMinPlayers:)]
        pub unsafe fn setMinPlayers(&self, min_players: NSUInteger);

        #[method(maxPlayers)]
        pub unsafe fn maxPlayers(&self) -> NSUInteger;

        #[method(setMaxPlayers:)]
        pub unsafe fn setMaxPlayers(&self, max_players: NSUInteger);

        #[method(playerGroup)]
        pub unsafe fn playerGroup(&self) -> NSUInteger;

        #[method(setPlayerGroup:)]
        pub unsafe fn setPlayerGroup(&self, player_group: NSUInteger);

        #[method(playerAttributes)]
        pub unsafe fn playerAttributes(&self) -> u32;

        #[method(setPlayerAttributes:)]
        pub unsafe fn setPlayerAttributes(&self, player_attributes: u32);

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKPlayer"))]
        #[method_id(@__retain_semantics Other recipients)]
        pub unsafe fn recipients(&self) -> Option<Id<NSArray<GKPlayer>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKPlayer"))]
        #[method(setRecipients:)]
        pub unsafe fn setRecipients(&self, recipients: Option<&NSArray<GKPlayer>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other inviteMessage)]
        pub unsafe fn inviteMessage(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setInviteMessage:)]
        pub unsafe fn setInviteMessage(&self, invite_message: Option<&NSString>);

        #[method(defaultNumberOfPlayers)]
        pub unsafe fn defaultNumberOfPlayers(&self) -> NSUInteger;

        #[method(setDefaultNumberOfPlayers:)]
        pub unsafe fn setDefaultNumberOfPlayers(&self, default_number_of_players: NSUInteger);

        #[deprecated]
        #[method(restrictToAutomatch)]
        pub unsafe fn restrictToAutomatch(&self) -> bool;

        #[deprecated]
        #[method(setRestrictToAutomatch:)]
        pub unsafe fn setRestrictToAutomatch(&self, restrict_to_automatch: bool);

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(recipientResponseHandler)]
        pub unsafe fn recipientResponseHandler(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<GKPlayer>, GKInviteRecipientResponse)>;

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(setRecipientResponseHandler:)]
        pub unsafe fn setRecipientResponseHandler(
            &self,
            recipient_response_handler: Option<
                &Block<dyn Fn(NonNull<GKPlayer>, GKInviteRecipientResponse)>,
            >,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(inviteeResponseHandler)]
        pub unsafe fn inviteeResponseHandler(
            &self,
        ) -> *mut Block<dyn Fn(NonNull<NSString>, GKInviteeResponse)>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setInviteeResponseHandler:)]
        pub unsafe fn setInviteeResponseHandler(
            &self,
            invitee_response_handler: Option<&Block<dyn Fn(NonNull<NSString>, GKInviteeResponse)>>,
        );

        #[method(maxPlayersAllowedForMatchOfType:)]
        pub unsafe fn maxPlayersAllowedForMatchOfType(match_type: GKMatchType) -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other playersToInvite)]
        pub unsafe fn playersToInvite(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method(setPlayersToInvite:)]
        pub unsafe fn setPlayersToInvite(&self, players_to_invite: Option<&NSArray<NSString>>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other queueName)]
        pub unsafe fn queueName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setQueueName:)]
        pub unsafe fn setQueueName(&self, queue_name: Option<&NSString>);

        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Option<Id<GKMatchProperties>>;

        #[method(setProperties:)]
        pub unsafe fn setProperties(&self, properties: Option<&GKMatchProperties>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "GameKit_GKPlayer"))]
        #[method_id(@__retain_semantics Other recipientProperties)]
        pub unsafe fn recipientProperties(
            &self,
        ) -> Option<Id<NSDictionary<GKPlayer, GKMatchProperties>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "GameKit_GKPlayer"))]
        #[method(setRecipientProperties:)]
        pub unsafe fn setRecipientProperties(
            &self,
            recipient_properties: Option<&NSDictionary<GKPlayer, GKMatchProperties>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKMatchRequest")]
    unsafe impl GKMatchRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKInvite")]
    pub struct GKInvite;

    #[cfg(feature = "GameKit_GKInvite")]
    unsafe impl ClassType for GKInvite {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKInvite")]
unsafe impl NSObjectProtocol for GKInvite {}

extern_methods!(
    #[cfg(feature = "GameKit_GKInvite")]
    unsafe impl GKInvite {
        #[cfg(feature = "GameKit_GKPlayer")]
        #[method_id(@__retain_semantics Other sender)]
        pub unsafe fn sender(&self) -> Id<GKPlayer>;

        #[method(isHosted)]
        pub unsafe fn isHosted(&self) -> bool;

        #[method(playerGroup)]
        pub unsafe fn playerGroup(&self) -> NSUInteger;

        #[method(playerAttributes)]
        pub unsafe fn playerAttributes(&self) -> u32;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other inviter)]
        pub unsafe fn inviter(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKInvite")]
    unsafe impl GKInvite {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait GKInviteEventListener {
        #[cfg(all(feature = "GameKit_GKInvite", feature = "GameKit_GKPlayer"))]
        #[optional]
        #[method(player:didAcceptInvite:)]
        unsafe fn player_didAcceptInvite(&self, player: &GKPlayer, invite: &GKInvite);

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKPlayer"))]
        #[optional]
        #[method(player:didRequestMatchWithRecipients:)]
        unsafe fn player_didRequestMatchWithRecipients(
            &self,
            player: &GKPlayer,
            recipient_players: &NSArray<GKPlayer>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "GameKit_GKPlayer"
        ))]
        #[deprecated]
        #[optional]
        #[method(player:didRequestMatchWithPlayers:)]
        unsafe fn player_didRequestMatchWithPlayers(
            &self,
            player: &GKPlayer,
            player_i_ds_to_invite: &NSArray<NSString>,
        );
    }

    unsafe impl ProtocolType for dyn GKInviteEventListener {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKMatchedPlayers")]
    pub struct GKMatchedPlayers;

    #[cfg(feature = "GameKit_GKMatchedPlayers")]
    unsafe impl ClassType for GKMatchedPlayers {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKMatchedPlayers")]
unsafe impl NSObjectProtocol for GKMatchedPlayers {}

extern_methods!(
    #[cfg(feature = "GameKit_GKMatchedPlayers")]
    unsafe impl GKMatchedPlayers {
        #[method_id(@__retain_semantics Other properties)]
        pub unsafe fn properties(&self) -> Option<Id<GKMatchProperties>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKPlayer"))]
        #[method_id(@__retain_semantics Other players)]
        pub unsafe fn players(&self) -> Id<NSArray<GKPlayer>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "GameKit_GKPlayer"))]
        #[method_id(@__retain_semantics Other playerProperties)]
        pub unsafe fn playerProperties(
            &self,
        ) -> Option<Id<NSDictionary<GKPlayer, GKMatchProperties>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKMatchedPlayers")]
    unsafe impl GKMatchedPlayers {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKMatchmaker")]
    pub struct GKMatchmaker;

    #[cfg(feature = "GameKit_GKMatchmaker")]
    unsafe impl ClassType for GKMatchmaker {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKMatchmaker")]
unsafe impl NSObjectProtocol for GKMatchmaker {}

extern_methods!(
    #[cfg(feature = "GameKit_GKMatchmaker")]
    unsafe impl GKMatchmaker {
        #[method_id(@__retain_semantics Other sharedMatchmaker)]
        pub unsafe fn sharedMatchmaker() -> Id<GKMatchmaker>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "GameKit_GKInvite",
            feature = "GameKit_GKMatch"
        ))]
        #[method(matchForInvite:completionHandler:)]
        pub unsafe fn matchForInvite_completionHandler(
            &self,
            invite: &GKInvite,
            completion_handler: Option<&Block<dyn Fn(*mut GKMatch, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "GameKit_GKMatch",
            feature = "GameKit_GKMatchRequest"
        ))]
        #[method(findMatchForRequest:withCompletionHandler:)]
        pub unsafe fn findMatchForRequest_withCompletionHandler(
            &self,
            request: &GKMatchRequest,
            completion_handler: Option<&Block<dyn Fn(*mut GKMatch, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKMatchRequest",
            feature = "GameKit_GKPlayer"
        ))]
        #[method(findPlayersForHostedRequest:withCompletionHandler:)]
        pub unsafe fn findPlayersForHostedRequest_withCompletionHandler(
            &self,
            request: &GKMatchRequest,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<GKPlayer>, *mut NSError)>>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "GameKit_GKMatchRequest",
            feature = "GameKit_GKMatchedPlayers"
        ))]
        #[method(findMatchedPlayers:withCompletionHandler:)]
        pub unsafe fn findMatchedPlayers_withCompletionHandler(
            &self,
            request: &GKMatchRequest,
            completion_handler: &Block<dyn Fn(*mut GKMatchedPlayers, *mut NSError)>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "GameKit_GKMatch",
            feature = "GameKit_GKMatchRequest"
        ))]
        #[method(addPlayersToMatch:matchRequest:completionHandler:)]
        pub unsafe fn addPlayersToMatch_matchRequest_completionHandler(
            &self,
            r#match: &GKMatch,
            match_request: &GKMatchRequest,
            completion_handler: Option<&Block<dyn Fn(*mut NSError)>>,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(cancelPendingInviteToPlayer:)]
        pub unsafe fn cancelPendingInviteToPlayer(&self, player: &GKPlayer);

        #[cfg(feature = "GameKit_GKMatch")]
        #[method(finishMatchmakingForMatch:)]
        pub unsafe fn finishMatchmakingForMatch(&self, r#match: &GKMatch);

        #[cfg(feature = "Foundation_NSError")]
        #[method(queryPlayerGroupActivity:withCompletionHandler:)]
        pub unsafe fn queryPlayerGroupActivity_withCompletionHandler(
            &self,
            player_group: NSUInteger,
            completion_handler: Option<&Block<dyn Fn(NSInteger, *mut NSError)>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(queryActivityWithCompletionHandler:)]
        pub unsafe fn queryActivityWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<dyn Fn(NSInteger, *mut NSError)>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(queryQueueActivity:withCompletionHandler:)]
        pub unsafe fn queryQueueActivity_withCompletionHandler(
            &self,
            queue_name: &NSString,
            completion_handler: Option<&Block<dyn Fn(NSInteger, *mut NSError)>>,
        );

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(startBrowsingForNearbyPlayersWithHandler:)]
        pub unsafe fn startBrowsingForNearbyPlayersWithHandler(
            &self,
            reachable_handler: Option<&Block<dyn Fn(NonNull<GKPlayer>, Bool)>>,
        );

        #[method(stopBrowsingForNearbyPlayers)]
        pub unsafe fn stopBrowsingForNearbyPlayers(&self);

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(startGroupActivityWithPlayerHandler:)]
        pub unsafe fn startGroupActivityWithPlayerHandler(
            &self,
            handler: &Block<dyn Fn(NonNull<GKPlayer>)>,
        );

        #[method(stopGroupActivity)]
        pub unsafe fn stopGroupActivity(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKMatchmaker")]
    unsafe impl GKMatchmaker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// GKDeprecated
    #[cfg(feature = "GameKit_GKMatchmaker")]
    unsafe impl GKMatchmaker {
        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKInvite"))]
        #[deprecated = "Use registerListener on GKLocalPlayer to register an object that implements the GKInviteEventListener instead."]
        #[method(inviteHandler)]
        pub unsafe fn inviteHandler(&self) -> *mut Block<dyn Fn(NonNull<GKInvite>, *mut NSArray)>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKInvite"))]
        #[deprecated = "Use registerListener on GKLocalPlayer to register an object that implements the GKInviteEventListener instead."]
        #[method(setInviteHandler:)]
        pub unsafe fn setInviteHandler(
            &self,
            invite_handler: Option<&Block<dyn Fn(NonNull<GKInvite>, *mut NSArray)>>,
        );
    }
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKMatchmaker")]
    unsafe impl GKMatchmaker {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(startBrowsingForNearbyPlayersWithReachableHandler:)]
        pub unsafe fn startBrowsingForNearbyPlayersWithReachableHandler(
            &self,
            reachable_handler: Option<&Block<dyn Fn(NonNull<NSString>, Bool)>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(cancelInviteToPlayer:)]
        pub unsafe fn cancelInviteToPlayer(&self, player_id: &NSString);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "GameKit_GKMatchRequest"
        ))]
        #[deprecated]
        #[method(findPlayersForHostedMatchRequest:withCompletionHandler:)]
        pub unsafe fn findPlayersForHostedMatchRequest_withCompletionHandler(
            &self,
            request: &GKMatchRequest,
            completion_handler: Option<&Block<dyn Fn(*mut NSArray<NSString>, *mut NSError)>>,
        );
    }
);
