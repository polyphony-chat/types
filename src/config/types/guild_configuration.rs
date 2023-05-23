use serde::{Deserialize, Serialize};

use crate::config::types::subconfigs::guild::{
    autojoin::AutoJoinConfiguration, discovery::DiscoverConfiguration,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GuildFeatures {
    ActivitiesAlpha,
    ActivitiesEmployee,
    ActivitiesInternalDev,
    AnimatedBanner,
    AnimatedIcon,
    ApplicationCommandPermissionsV2,
    AutoModeration,
    AutoModTriggerKeywordFilter,
    AutoModTriggerMLSpamFilter,
    AutoModTriggerSpamLinkFilter,
    AutoModTriggerUserProfile,
    Banner,
    BFG,
    BoostingTiersExperimentMediumGuild,
    BoostingTiersExperimentSmallGuild,
    BotDeveloperEarlyAccess,
    BurstReactions,
    CommunityCanary,
    CommunityExpLargeGated,
    CommunityExpLargeUngated,
    CommunityExpMedium,
    ChannelEmojisGenerated,
    ChannelHighlights,
    ChannelHighlightsDisabled,
    ClydeEnabled,
    ClydeExperimentEnabled,
    ClydeDisabled,
    Community,
    CreatorAcceptedNewTerms,
    CreatorMonetizable,
    CreatorMonetizableDisabled,
    CreatorMonetizablePendingNewOwnerOnboarding,
    CreatorMonetizableProvisional,
    CreatorMonetizableRestricted,
    CreatorMonetizableWhiteglove,
    CreatorMonetizableApplicationAllowlist,
    CreateStorePage,
    DeveloperSupportServer,
    DiscoverableDisabled,
    Discoverable,
    EnabledDiscoverableBefore,
    ExposedToActivitiesWTPExperiment,
    GuestsEnabled,
    GuildAutomodDefaultList,
    GuildCommunicationDisabledGuilds,
    GuildHomeDeprecationOverride,
    GuildHomeOverride,
    GuildHomeTest,
    GuildMemberVerificationExperiment,
    GuildOnboarding,
    GuildOnboardingAdminOnly,
    GuildOnboardingEverEnabled,
    GuildOnboardingHasPrompts,
    GuildRoleSubscription,
    GuildRoleSubscriptionPurchaseFeedbackLoop,
    GuildRoleSubscriptionTrials,
    GuildServerGuide,
    GuildWebPageVanityURL,
    HadEarlyActivitiesAccess,
    HasDirectoryEntry,
    HideFromExperimentUI,
    Hub,
    IncreasedThreadLimit,
    InternalEmployeeOnly,
    InviteSplash,
    InvitesDisabled,
    LinkedToHub,
    MarketplacesConnectionRoles,
    MemberProfiles,
    MemberVerificationGateEnabled,
    MemberVerificationManualApproval,
    MobileWebRoleSubscriptionPurchasePage,
    MonetizationEnabled,
    MoreEmoji,
    MoreStickers,
    News,
    NewThreadPermissions,
    Partnered,
    PremiumTier3Override,
    PreviewEnabled,
    RaidAlertsDisabled,
    RelayEnabled,
    RestrictSpamRiskGuild,
    RoleIcons,
    RoleSubscriptionsAvailableForPurchase,
    RoleSubscriptionsEnabled,
    RoleSubscriptionsEnabledForPurchase,
    Shard,
    SharedCanvasFriendsAndFamilyTest,
    Soundboard,
    SummariesEnabled,
    SummariesEnabledGA,
    SummariesDisabledByUser,
    SummariesEnabledByUser,
    TextInStageEnabled,
    TextInVoiceEnabled,
    ThreadsEnabledTesting,
    ThreadsEnabled,
    ThreadDefaultAutoArchiveDuration,
    ThreadsOnlyChannel,
    TicketedEventsEnabled,
    TicketingEnabled,
    VanityUrls,
    Verified,
    VIPRegions,
    VoiceChannelEffects,
    WelcomeScreenEnabled,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuildConfiguration {
    pub discovery: DiscoverConfiguration,
    pub autojoin: AutoJoinConfiguration,
    pub defaultfeatures: Vec<GuildFeatures>,
}
