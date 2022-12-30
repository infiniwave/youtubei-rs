use serde::Deserialize;
use serde_json::Value;

use super::{
    channel::{
        BackstageImageRenderer, BackstagePostRenderer, BackstagePostThreadRenderer,
        C4TabbedHeaderRenderer, ChannelAboutFullMetadataRenderer, ChannelMetadataRenderer,
        ChannelRenderer, ChannelVideoPlayerRenderer, GridChannelRenderer, GridRadioRenderer,
        PostMultiImageRenderer, RichGridRenderer, RichItemRenderer,
        SharedPostRenderer,
    },
    misc::{
        ButtonRenderer, ContinuationItemRenderer, GridRenderer, HashtagHeaderRenderer,
        IncludingResultsForRenderer, ItemSectionRenderer, MessageRenderer, ReelShelfRenderer,
        SectionListRenderer, ShelfRenderer, ShowingResultsForRenderer,
        SingleColumnBrowseResultsRenderer, ToggleButtonRenderer, TwoColumnBrowseResultsRenderer,
        TwoColumnSearchResultsRenderer, TwoColumnWatchNextResults,
    },
    playlist::{
        CompactPlaylistRenderer, CompactRadioRenderer, GridPlaylistRenderer,
        PlaylistHeaderRenderer, PlaylistMetadataRenderer, PlaylistRenderer,
        PlaylistVideoListRenderer, RadioRenderer,
    },
    thumbnail::ThumbnailOverlayTimeStatusRenderer,
    video::{
        CommentRenderer, CommentThreadRenderer, CompactMovieRenderer, CompactVideoRenderer,
        GridVideoRenderer, MetadataRowRenderer, PlaylistVideoRenderer, VideoPrimaryInfoRenderer,
        VideoRenderer, VideoSecondaryInfoRenderer,
    },
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ItemSectionRendererContents {
    ContinuationItemRenderer(ContinuationItemRenderer),
    GridRenderer(GridRenderer),
    GridVideoRenderer(GridVideoRenderer),
    BackstagePostThreadRenderer(BackstagePostThreadRenderer),
    ItemSectionRenderer(ItemSectionRenderer),
    PlaylistRenderer(PlaylistRenderer),
    VideoRenderer(VideoRenderer),
    ChannelRenderer(ChannelRenderer),
    GridChannelRenderer(GridChannelRenderer),
    GridPlaylistRenderer(GridPlaylistRenderer),
    ShelfRenderer(ShelfRenderer),
    ChannelVideoPlayerRenderer(ChannelVideoPlayerRenderer),
    ReelShelfRenderer(ReelShelfRenderer), // Shorts are stored in here
    //SearchPyvRenderer(Value), // TODO FIND OUT WHAT IT IS
    CommentsEntryPointHeaderRenderer(Value),
    RadioRenderer(RadioRenderer), // Wrapper for CompactRadioRenderer
    ChannelAboutFullMetadataRenderer(ChannelAboutFullMetadataRenderer),
    RecognitionShelfRenderer(Value), // Channel members but needs more investigation
    MessageRenderer(MessageRenderer), // Contains messages that e.g the channel has no videos
    ShowingResultsForRenderer(ShowingResultsForRenderer), // When youtube returns other results
    IncludingResultsForRenderer(IncludingResultsForRenderer), // When youtube returns other results
    PlaylistVideoListRenderer(PlaylistVideoListRenderer), // Contains a playlist data that can be viewed
    GridRadioRenderer(GridRadioRenderer),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HeaderContents {
    C4TabbedHeaderRenderer(C4TabbedHeaderRenderer),
    HashtagHeaderRenderer(HashtagHeaderRenderer),
    PlaylistHeaderRenderer(PlaylistHeaderRenderer),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NextContents {
    CompactPromotedVideoRenderer(Value), // Contains only ads which we dont need lol
    CompactRadioRenderer(CompactRadioRenderer), // Holds mixes? Aka autogenerated playlists
    CompactVideoRenderer(CompactVideoRenderer),
    CompactMovieRenderer(CompactMovieRenderer),
    CommentsEntryPointHeaderRenderer(Value),
    CommentRenderer(CommentRenderer),
    CommentThreadRenderer(CommentThreadRenderer),
    CommentsHeaderRenderer(Value),
    VideoPrimaryInfoRenderer(VideoPrimaryInfoRenderer),
    VideoSecondaryInfoRenderer(VideoSecondaryInfoRenderer),
    ContinuationItemRenderer(ContinuationItemRenderer),
    ItemSectionRenderer(ItemSectionRenderer),
    PromotedSparklesWebRenderer(Value),
    MerchandiseShelfRenderer(Value), // Basically ads but we dont want those so we don't parse it further
    CompactPlaylistRenderer(CompactPlaylistRenderer), // Contains a playlist
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TabRendererContent {
    RichGridRenderer(RichGridRenderer), // Hashtags are stored in there
    SectionListRenderer(SectionListRenderer),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RichGridRendererContent {
    RichItemRenderer(RichItemRenderer),
    ContinuationItemRenderer(ContinuationItemRenderer),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TopLevelButtons {
    ButtonRenderer(ButtonRenderer),
    ToggleButtonRenderer(ToggleButtonRenderer),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MetadataRowContents {
    MetadataRowRenderer(MetadataRowRenderer),
    MetadataRowHeaderRenderer(Value),
    RichMetadataRowRenderer(Value),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TwoColumnTypes {
    TwoColumnBrowseResultsRenderer(TwoColumnBrowseResultsRenderer),
    TwoColumnWatchNextResults(TwoColumnWatchNextResults),
    TwoColumnSearchResultsRenderer(TwoColumnSearchResultsRenderer),
    SingleColumnBrowseResultsRenderer(SingleColumnBrowseResultsRenderer),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MetadataRenderers {
    PlaylistMetadataRenderer(PlaylistMetadataRenderer),
    ChannelMetadataRenderer(ChannelMetadataRenderer),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlaylistVideoListContent {
    ContinuationItemRenderer(ContinuationItemRenderer),
    PlaylistVideoRenderer(PlaylistVideoRenderer),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ThumbnailOverlays {
    ThumbnailOverlayTimeStatusRenderer(ThumbnailOverlayTimeStatusRenderer),
    ThumbnailOverlayToggleButtonRenderer(Value),
    ThumbnailOverlayNowPlayingRenderer(Value),
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackstageAttachments {
    BackstageImageRenderer(BackstageImageRenderer),
    VideoRenderer(VideoRenderer),
    PostMultiImageRenderer(PostMultiImageRenderer),
    PollRenderer(Value),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommunityPost {
    SharedPostRenderer(SharedPostRenderer),
    BackstagePostRenderer(BackstagePostRenderer),
}
