use std::collections::HashSet;

use super::{
    album_page_url, artist_page_url, artist_pairs, build_track, clean_home_subtitle, is_video_id,
    parse_abbreviated_count, parse_duration_text, playlist_page_url, CardData, HomeCard,
    LibraryKind, MusicArtistRef, ProviderId, Track, TrackAlbum, Value,
};

pub(super) fn text_value(value: &Value) -> String {
    if let Some(text) = value.as_str() {
        return text.to_string();
    }
    if let Some(text) = value.get("simpleText").and_then(Value::as_str) {
        return text.to_string();
    }
    value
        .get("runs")
        .and_then(Value::as_array)
        .map(|runs| {
            runs.iter()
                .filter_map(|run| run.get("text").and_then(Value::as_str))
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn flex_text(row: &Value, index: usize) -> Option<&Value> {
    row.get("flexColumns")?
        .get(index)?
        .get("musicResponsiveListItemFlexColumnRenderer")?
        .get("text")
}

pub(super) fn flex_text_value(row: &Value, index: usize) -> String {
    flex_text(row, index).map(text_value).unwrap_or_default()
}

pub(super) fn views_from_runs(runs: &[Value]) -> u64 {
    for run in runs {
        let text = run.get("text").and_then(Value::as_str).unwrap_or("").trim();
        if text.ends_with("plays") || text.ends_with("views") {
            return parse_abbreviated_count(text);
        }
    }
    0
}

pub(super) fn browse_page_type(browse: &Value) -> &str {
    browse
        .pointer("/browseEndpointContextSupportedConfigs/browseEndpointContextMusicConfig/pageType")
        .and_then(Value::as_str)
        .unwrap_or("")
}

pub(super) fn run_browse_info(run: &Value) -> Option<(&str, &str)> {
    let browse = run.pointer("/navigationEndpoint/browseEndpoint")?;
    Some((
        browse
            .get("browseId")
            .and_then(Value::as_str)
            .unwrap_or_default(),
        browse_page_type(browse),
    ))
}

pub(super) fn is_artist_browse(id: &str, page: &str) -> bool {
    matches!(
        page,
        "MUSIC_PAGE_TYPE_ARTIST" | "MUSIC_PAGE_TYPE_USER_CHANNEL"
    ) || id.starts_with("UC")
}

pub(super) fn is_album_browse(id: &str, page: &str) -> bool {
    page != "MUSIC_PAGE_TYPE_AUDIOBOOK"
        && (page == "MUSIC_PAGE_TYPE_ALBUM" || id.starts_with("MPREb_"))
}

pub(super) fn is_playlist_browse(id: &str, page: &str) -> bool {
    page == "MUSIC_PAGE_TYPE_PLAYLIST"
        || id.starts_with("VL")
        || id.starts_with("RD")
        || id.starts_with("OL")
}

pub(super) fn browse_kind(id: &str, page: &str) -> Option<LibraryKind> {
    if is_playlist_browse(id, page) {
        Some(LibraryKind::Playlist)
    } else if is_artist_browse(id, page) {
        Some(LibraryKind::Artist)
    } else if is_album_browse(id, page) {
        Some(LibraryKind::Album)
    } else {
        None
    }
}

pub(super) fn artists_from_runs(runs: &[Value]) -> Vec<MusicArtistRef> {
    let mut artists = Vec::new();
    for run in runs {
        let Some((browse_id, page_type)) = run_browse_info(run) else {
            continue;
        };
        let name = run.get("text").and_then(Value::as_str).unwrap_or("").trim();
        if name.is_empty() || !is_artist_browse(browse_id, page_type) {
            continue;
        }
        if !artists.iter().any(|artist: &MusicArtistRef| {
            artist.name == name && artist.browse_id.as_deref() == Some(browse_id)
        }) {
            artists.push(MusicArtistRef {
                name: name.to_string(),
                browse_id: (!browse_id.is_empty()).then(|| browse_id.to_string()),
            });
        }
    }
    artists
}

pub(super) fn raw_row_views(row: &Value) -> u64 {
    for index in 0..4 {
        let Some(text) = flex_text(row, index) else {
            continue;
        };
        if let Some(runs) = text.get("runs").and_then(Value::as_array) {
            let views = views_from_runs(runs);
            if views > 0 {
                return views;
            }
        }
        let text = text
            .get("simpleText")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim();
        if text.ends_with("plays") || text.ends_with("views") {
            return parse_abbreviated_count(text);
        }
    }
    0
}

pub(super) fn is_duration_text(text: &str) -> bool {
    text.contains(':')
        && text.split(':').count() <= 3
        && text
            .split(':')
            .all(|part| !part.is_empty() && part.chars().all(|ch| ch.is_ascii_digit()))
}

pub(super) fn raw_row_duration(row: &Value) -> u32 {
    let candidates = [
        row.pointer("/fixedColumns/0/musicResponsiveListItemFixedColumnRenderer/text"),
        flex_text(row, 1),
    ];
    for text in candidates.into_iter().flatten() {
        if let Some(runs) = text.get("runs").and_then(Value::as_array) {
            for run in runs {
                let text = run.get("text").and_then(Value::as_str).unwrap_or("").trim();
                if is_duration_text(text) {
                    return parse_duration_text(text);
                }
            }
        }
        let text = text
            .get("simpleText")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim();
        if is_duration_text(text) {
            return parse_duration_text(text);
        }
    }
    0
}

pub(super) fn raw_row_artists(row: &Value) -> Vec<MusicArtistRef> {
    let mut artists = Vec::new();
    for index in 0..4 {
        let Some(text) = flex_text(row, index) else {
            continue;
        };
        let Some(runs) = text.get("runs").and_then(Value::as_array) else {
            continue;
        };
        for artist in artists_from_runs(runs) {
            if !artists.iter().any(|existing: &MusicArtistRef| {
                existing.name == artist.name && existing.browse_id == artist.browse_id
            }) {
                artists.push(artist);
            }
        }
    }
    artists
}

pub(super) fn raw_row_album(row: &Value) -> Option<TrackAlbum> {
    for index in 0..4 {
        let Some(text) = flex_text(row, index) else {
            continue;
        };
        let Some(runs) = text.get("runs").and_then(Value::as_array) else {
            continue;
        };
        for run in runs {
            let Some((browse_id, page_type)) = run_browse_info(run) else {
                continue;
            };
            let name = run.get("text").and_then(Value::as_str).unwrap_or("").trim();
            if !name.is_empty() && is_album_browse(browse_id, page_type) {
                return Some(TrackAlbum {
                    name: name.to_string(),
                    id: browse_id.to_string(),
                });
            }
        }
    }
    None
}

pub(super) fn raw_row_video_id(row: &Value) -> Option<&str> {
    row.pointer("/playlistItemData/videoId")
        .or_else(|| {
            row.pointer(
                "/flexColumns/0/musicResponsiveListItemFlexColumnRenderer/text/runs/0/navigationEndpoint/watchEndpoint/videoId",
            )
        })
        .or_else(|| {
            row.pointer(
                "/overlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/watchEndpoint/videoId",
            )
        })
        .and_then(Value::as_str)
}

pub(super) fn thumbnail_list_url(thumbnails: Option<&Value>) -> String {
    thumbnails
        .and_then(Value::as_array)
        .and_then(|thumbnails| thumbnails.last())
        .and_then(|thumbnail| thumbnail.get("url"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

pub(super) fn raw_row_thumbnail(row: &Value) -> String {
    thumbnail_list_url(
        row.pointer("/thumbnail/musicThumbnailRenderer/thumbnail/thumbnails")
            .or_else(|| row.pointer("/thumbnail/thumbnails")),
    )
}

pub(super) fn card_thumbnail(card: &Value) -> String {
    thumbnail_list_url(
        card.pointer("/thumbnailRenderer/musicThumbnailRenderer/thumbnail/thumbnails")
            .or_else(|| card.pointer("/thumbnail/thumbnails")),
    )
}

pub(super) fn collect_renderers<'a>(value: &'a Value, key: &str, out: &mut Vec<&'a Value>) {
    match value {
        Value::Object(map) => {
            if let Some(renderer) = map.get(key) {
                out.push(renderer);
            }
            for child in map.values() {
                collect_renderers(child, key, out);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_renderers(item, key, out);
            }
        }
        _ => {}
    }
}

pub(super) fn collect_music_rows(value: &Value) -> Vec<&Value> {
    let mut rows = Vec::new();
    collect_renderers(value, "musicResponsiveListItemRenderer", &mut rows);
    rows
}

pub(super) fn collect_two_row_cards(value: &Value) -> Vec<&Value> {
    let mut cards = Vec::new();
    collect_renderers(value, "musicTwoRowItemRenderer", &mut cards);
    cards
}

pub(super) fn track_from_raw_row(
    row: &Value,
    album: Option<&TrackAlbum>,
    fallback_artists: &[MusicArtistRef],
    fallback_thumbnail: &str,
) -> Option<Track> {
    let video_id = raw_row_video_id(row)?;
    if !is_video_id(video_id) {
        return None;
    }
    let title = flex_text_value(row, 0);
    if title.is_empty() {
        return None;
    }
    let mut artists = raw_row_artists(row);
    if artists.is_empty() {
        artists = fallback_artists.to_vec();
    }
    let album = album.cloned().or_else(|| raw_row_album(row));
    let mut thumbnail = raw_row_thumbnail(row);
    if thumbnail.is_empty() {
        thumbnail = fallback_thumbnail.to_string();
    }
    Some(build_track(
        video_id,
        title,
        artist_pairs(&artists),
        album,
        raw_row_duration(row),
        thumbnail,
        raw_row_views(row),
    ))
}

pub(super) fn raw_rows_tracks(
    value: &Value,
    album: Option<&TrackAlbum>,
    fallback_artists: &[MusicArtistRef],
    fallback_thumbnail: &str,
) -> Vec<Track> {
    let mut seen = HashSet::new();
    collect_music_rows(value)
        .into_iter()
        .filter_map(|row| track_from_raw_row(row, album, fallback_artists, fallback_thumbnail))
        .filter(|track| match track.provider_id(ProviderId::YouTube) {
            Some(id) => seen.insert(id.to_string()),
            None => true,
        })
        .collect()
}

pub(super) fn raw_tracks(value: &Value) -> Vec<Track> {
    raw_rows_tracks(value, None, &[], "")
}

pub(super) fn track_from_two_row(
    card: &Value,
    fallback_artists: &[MusicArtistRef],
) -> Option<Track> {
    let video_id = card
        .pointer("/navigationEndpoint/watchEndpoint/videoId")
        .or_else(|| {
            card.pointer(
                "/thumbnailOverlay/musicItemThumbnailOverlayRenderer/content/musicPlayButtonRenderer/playNavigationEndpoint/watchEndpoint/videoId",
            )
        })
        .and_then(Value::as_str)?;
    if !is_video_id(video_id) {
        return None;
    }
    let title = card.get("title").map(text_value).unwrap_or_default();
    if title.is_empty() {
        return None;
    }
    let runs = card
        .get("subtitle")
        .and_then(|subtitle| subtitle.get("runs"))
        .and_then(Value::as_array);
    let mut artists = runs.map_or_else(Vec::new, |runs| artists_from_runs(runs));
    if artists.is_empty() {
        artists = fallback_artists.to_vec();
    }
    let views = runs.map_or(0, |runs| views_from_runs(runs));
    Some(build_track(
        video_id,
        title,
        artist_pairs(&artists),
        None,
        0,
        card_thumbnail(card),
        views,
    ))
}

pub(super) fn raw_two_row_tracks(value: &Value, fallback_artists: &[MusicArtistRef]) -> Vec<Track> {
    let mut seen = HashSet::new();
    collect_two_row_cards(value)
        .into_iter()
        .filter_map(|card| track_from_two_row(card, fallback_artists))
        .filter(|track| match track.provider_id(ProviderId::YouTube) {
            Some(id) => seen.insert(id.to_string()),
            None => true,
        })
        .collect()
}

pub(super) fn normalize_playlist_id(id: &str) -> &str {
    id.strip_prefix("VL").unwrap_or(id)
}

pub(super) fn playlist_browse_id(id: &str) -> String {
    format!("VL{}", normalize_playlist_id(id))
}

pub(super) fn two_row_card(card: &Value) -> Option<(LibraryKind, CardData)> {
    let browse = card.pointer("/navigationEndpoint/browseEndpoint")?;
    let raw_id = browse.get("browseId").and_then(Value::as_str)?;
    if raw_id.is_empty() {
        return None;
    }
    let kind = browse_kind(raw_id, browse_page_type(browse))?;
    let title = card.get("title").map(text_value).unwrap_or_default();
    if title.is_empty() {
        return None;
    }
    let subtitle = card.get("subtitle").map(text_value).unwrap_or_default();
    let subtitle = match kind {
        LibraryKind::Album => {
            let artist_names: Vec<String> = card
                .pointer("/subtitle/runs")
                .and_then(Value::as_array)
                .map(|runs| artists_from_runs(runs))
                .unwrap_or_default()
                .into_iter()
                .map(|artist| artist.name)
                .collect();
            if artist_names.is_empty() {
                clean_home_subtitle(&subtitle)
            } else {
                artist_names.join(", ")
            }
        }
        LibraryKind::Artist | LibraryKind::Playlist => subtitle,
    };
    let id = if kind == LibraryKind::Playlist {
        normalize_playlist_id(raw_id)
    } else {
        raw_id
    };
    let url = match kind {
        LibraryKind::Album => album_page_url(id),
        LibraryKind::Artist => artist_page_url(id),
        LibraryKind::Playlist => playlist_page_url(id),
    };
    Some((
        kind,
        CardData {
            id: id.to_string(),
            title,
            subtitle,
            thumbnail: card_thumbnail(card),
            url,
        },
    ))
}

pub(super) fn playlist_card_from_artist_shelf(item: &Value) -> Option<CardData> {
    let browse = item.pointer("/navigationEndpoint/browseEndpoint")?;
    let raw_id = browse.get("browseId").and_then(Value::as_str)?;
    if !is_playlist_browse(raw_id, browse_page_type(browse)) {
        return None;
    }
    let id = normalize_playlist_id(raw_id);
    let title = item.get("title").map(text_value).unwrap_or_default();
    if title.is_empty() {
        return None;
    }
    let subtitle = item.get("subtitle").map(text_value).unwrap_or_default();
    Some(CardData {
        url: playlist_page_url(id),
        id: id.to_string(),
        title,
        subtitle: clean_home_subtitle(&subtitle),
        thumbnail: card_thumbnail(item),
    })
}

pub(super) fn raw_artist_playlist_cards(value: &Value) -> Vec<CardData> {
    let mut cards = Vec::new();
    collect_artist_playlist_cards(value, &mut cards);
    cards
}

pub(super) fn collect_artist_playlist_cards(value: &Value, out: &mut Vec<CardData>) {
    match value {
        Value::Object(map) => {
            if let Some(carousel) = map.get("musicCarouselShelfRenderer") {
                let title = carousel
                    .pointer("/header/musicCarouselShelfBasicHeaderRenderer/title")
                    .map(text_value)
                    .unwrap_or_default()
                    .to_lowercase();
                if title.contains("playlist") || title.contains("featured on") {
                    if let Some(contents) = carousel.get("contents").and_then(Value::as_array) {
                        for item in contents {
                            let Some(card) = item
                                .get("musicTwoRowItemRenderer")
                                .and_then(playlist_card_from_artist_shelf)
                            else {
                                continue;
                            };
                            if !out.iter().any(|existing| existing.id == card.id) {
                                out.push(card);
                            }
                        }
                    }
                }
            }
            for child in map.values() {
                collect_artist_playlist_cards(child, out);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_artist_playlist_cards(item, out);
            }
        }
        _ => {}
    }
}

pub(super) fn raw_chart_artist_cards(value: &Value) -> Vec<CardData> {
    collect_music_rows(value)
        .into_iter()
        .filter_map(|row| {
            let (id, page_type) = run_browse_info(row)?;
            if !is_artist_browse(id, page_type) || id.is_empty() {
                return None;
            }
            let title = flex_text_value(row, 0);
            if title.is_empty() {
                return None;
            }
            let rank = row
                .pointer("/customIndexColumn/musicCustomIndexColumnRenderer/text")
                .map(text_value)
                .unwrap_or_default();
            let subtitle = if rank.is_empty() {
                flex_text_value(row, 1)
            } else {
                format!("#{rank}")
            };
            Some(CardData {
                id: id.to_string(),
                title,
                subtitle,
                thumbnail: raw_row_thumbnail(row),
                url: artist_page_url(id),
            })
        })
        .collect()
}

pub(super) fn browse_sections(value: &Value) -> Option<&[Value]> {
    value
        .pointer(
            "/contents/singleColumnBrowseResultsRenderer/tabs/0/tabRenderer/content/sectionListRenderer/contents",
        )
        .or_else(|| value.pointer("/continuationContents/sectionListContinuation/contents"))
        .and_then(Value::as_array)
        .map(Vec::as_slice)
}

pub(super) struct HomeShelf {
    pub(super) title: String,
    pub(super) tracks: Vec<Track>,
    pub(super) cards: Vec<HomeCard>,
}

pub(super) fn raw_home_shelves(value: &Value) -> Vec<HomeShelf> {
    let Some(sections) = browse_sections(value) else {
        return Vec::new();
    };
    sections
        .iter()
        .filter_map(|section| {
            let shelf = section
                .get("musicCarouselShelfRenderer")
                .or_else(|| section.get("musicShelfRenderer"))?;
            let title = shelf
                .pointer("/header/musicCarouselShelfBasicHeaderRenderer/title")
                .or_else(|| shelf.pointer("/header/musicShelfHeaderRenderer/title"))
                .or_else(|| shelf.get("title"))
                .map(text_value)
                .unwrap_or_default();
            let mut tracks = Vec::new();
            let mut cards = Vec::new();
            for item in shelf
                .get("contents")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                if let Some(row) = item.get("musicResponsiveListItemRenderer") {
                    if let Some(track) = track_from_raw_row(row, None, &[], "") {
                        tracks.push(track);
                    }
                } else if let Some(card) = item.get("musicTwoRowItemRenderer") {
                    if let Some(track) = track_from_two_row(card, &[]) {
                        tracks.push(track);
                    } else if let Some((kind, card)) = two_row_card(card) {
                        cards.push(HomeCard::new(kind, ProviderId::YouTube, card));
                    }
                }
            }
            Some(HomeShelf {
                title,
                tracks,
                cards,
            })
        })
        .collect()
}

pub(super) fn chart_video_playlist_id(value: &Value) -> Option<String> {
    let sections = browse_sections(value)?;
    let mut fallback = None;
    for section in sections {
        let Some(shelf) = section.get("musicCarouselShelfRenderer") else {
            continue;
        };
        let is_video_chart = shelf
            .pointer("/header/musicCarouselShelfBasicHeaderRenderer/title")
            .map(text_value)
            .unwrap_or_default()
            .to_lowercase()
            .contains("video chart");
        for card in shelf
            .get("contents")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            let Some(card) = card.get("musicTwoRowItemRenderer") else {
                continue;
            };
            let Some(browse) = card.pointer("/navigationEndpoint/browseEndpoint") else {
                continue;
            };
            let Some(raw_id) = browse.get("browseId").and_then(Value::as_str) else {
                continue;
            };
            if !is_playlist_browse(raw_id, browse_page_type(browse)) {
                continue;
            }
            let id = normalize_playlist_id(raw_id).to_string();
            if is_video_chart {
                return Some(id);
            }
            fallback.get_or_insert(id);
        }
    }
    fallback
}

pub(super) fn explore_album_cards(value: &Value) -> Vec<CardData> {
    collect_two_row_cards(value)
        .into_iter()
        .filter_map(two_row_card)
        .filter(|(kind, _)| *kind == LibraryKind::Album)
        .map(|(_, card)| card)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_raw_music_rows_with_metadata() {
        use serde_json::json;

        let value = json!({
            "musicResponsiveListItemRenderer": {
                "playlistItemData": { "videoId": "dQw4w9WgXcQ" },
                "flexColumns": [
                    {
                        "musicResponsiveListItemFlexColumnRenderer": {
                            "text": { "runs": [{ "text": "Never Gonna Give You Up" }] }
                        }
                    },
                    {
                        "musicResponsiveListItemFlexColumnRenderer": {
                            "text": { "runs": [
                                {
                                    "text": "Rick Astley",
                                    "navigationEndpoint": {
                                        "browseEndpoint": {
                                            "browseId": "UCwZEU0wAwIyZb4x5G_KJp2w",
                                            "browseEndpointContextSupportedConfigs": {
                                                "browseEndpointContextMusicConfig": {
                                                    "pageType": "MUSIC_PAGE_TYPE_ARTIST"
                                                }
                                            }
                                        }
                                    }
                                },
                                { "text": " • " },
                                {
                                    "text": "Whenever You Need Somebody",
                                    "navigationEndpoint": {
                                        "browseEndpoint": {
                                            "browseId": "MPREb_dcYZhAh5urI",
                                            "browseEndpointContextSupportedConfigs": {
                                                "browseEndpointContextMusicConfig": {
                                                    "pageType": "MUSIC_PAGE_TYPE_ALBUM"
                                                }
                                            }
                                        }
                                    }
                                },
                                { "text": " • " },
                                { "text": "3:34" }
                            ] }
                        }
                    },
                    {
                        "musicResponsiveListItemFlexColumnRenderer": {
                            "text": { "runs": [{ "text": "2B plays" }] }
                        }
                    }
                ],
                "thumbnail": {
                    "musicThumbnailRenderer": {
                        "thumbnail": { "thumbnails": [{ "url": "https://img/cover.jpg" }] }
                    }
                }
            }
        });
        let tracks = raw_rows_tracks(&value, None, &[], "");
        assert_eq!(tracks.len(), 1);
        let track = &tracks[0];
        assert_eq!(track.title, "Never Gonna Give You Up");
        assert_eq!(track.artists, vec!["Rick Astley"]);
        assert_eq!(track.duration(), 214);
        assert_eq!(track.play_count(), 2_000_000_000);
        assert_eq!(track.thumbnail(), "https://img/cover.jpg");
        assert_eq!(
            track
                .album()
                .map(|album| (album.name.as_str(), album.id.as_str())),
            Some(("Whenever You Need Somebody", "MPREb_dcYZhAh5urI"))
        );
        assert_eq!(
            track.artist_target(0),
            Some(("UCwZEU0wAwIyZb4x5G_KJp2w", ProviderId::YouTube))
        );
    }

    #[test]
    fn parses_raw_two_row_cards_with_views() {
        use serde_json::json;

        let value = json!({
            "musicTwoRowItemRenderer": {
                "title": { "runs": [{ "text": "Never Gonna Give You Up" }] },
                "subtitle": { "runs": [
                    {
                        "text": "Rick Astley",
                        "navigationEndpoint": {
                            "browseEndpoint": {
                                "browseId": "UCwZEU0wAwIyZb4x5G_KJp2w",
                                "browseEndpointContextSupportedConfigs": {
                                    "browseEndpointContextMusicConfig": {
                                        "pageType": "MUSIC_PAGE_TYPE_ARTIST"
                                    }
                                }
                            }
                        }
                    },
                    { "text": " • " },
                    { "text": "1.8B views" }
                ] },
                "navigationEndpoint": { "watchEndpoint": { "videoId": "dQw4w9WgXcQ" } },
                "thumbnailRenderer": {
                    "musicThumbnailRenderer": {
                        "thumbnail": { "thumbnails": [{ "url": "https://img/video.jpg" }] }
                    }
                }
            }
        });
        let tracks = raw_two_row_tracks(&value, &[]);
        assert_eq!(tracks.len(), 1);
        let track = &tracks[0];
        assert_eq!(track.title, "Never Gonna Give You Up");
        assert_eq!(track.artists, vec!["Rick Astley"]);
        assert_eq!(track.play_count(), 1_800_000_000);
        assert_eq!(track.thumbnail(), "https://img/video.jpg");
    }

    #[test]
    fn parses_raw_chart_artist_cards() {
        use serde_json::json;

        let value = json!({
            "musicResponsiveListItemRenderer": {
                "flexColumns": [
                    {
                        "musicResponsiveListItemFlexColumnRenderer": {
                            "text": { "runs": [{ "text": "Rick Astley" }] }
                        }
                    },
                    {
                        "musicResponsiveListItemFlexColumnRenderer": {
                            "text": { "runs": [{ "text": "2.32M subscribers" }] }
                        }
                    }
                ],
                "customIndexColumn": {
                    "musicCustomIndexColumnRenderer": {
                        "text": { "runs": [{ "text": "1" }] }
                    }
                },
                "navigationEndpoint": {
                    "browseEndpoint": {
                        "browseId": "UCwZEU0wAwIyZb4x5G_KJp2w",
                        "browseEndpointContextSupportedConfigs": {
                            "browseEndpointContextMusicConfig": {
                                "pageType": "MUSIC_PAGE_TYPE_ARTIST"
                            }
                        }
                    }
                }
            }
        });
        let cards = raw_chart_artist_cards(&value);
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].title, "Rick Astley");
        assert_eq!(cards[0].subtitle, "#1");
        assert_eq!(cards[0].id, "UCwZEU0wAwIyZb4x5G_KJp2w");
    }

    #[test]
    fn finds_the_video_chart_playlist() {
        use serde_json::json;

        let value = json!({
            "contents": {
                "singleColumnBrowseResultsRenderer": {
                    "tabs": [{
                        "tabRenderer": {
                            "content": {
                                "sectionListRenderer": {
                                    "contents": [
                                        {
                                            "musicCarouselShelfRenderer": {
                                                "header": {
                                                    "musicCarouselShelfBasicHeaderRenderer": {
                                                        "title": { "runs": [{ "text": "Top artists" }] }
                                                    }
                                                },
                                                "contents": []
                                            }
                                        },
                                        {
                                            "musicCarouselShelfRenderer": {
                                                "header": {
                                                    "musicCarouselShelfBasicHeaderRenderer": {
                                                        "title": { "runs": [{ "text": "Video charts" }] }
                                                    }
                                                },
                                                "contents": [{
                                                    "musicTwoRowItemRenderer": {
                                                        "title": { "runs": [{ "text": "Daily Top Music Videos" }] },
                                                        "navigationEndpoint": {
                                                            "browseEndpoint": {
                                                                "browseId": "VLPL4fGSI1pDJn6t3TXLGiiJdD-sZbrG3tG0",
                                                                "browseEndpointContextSupportedConfigs": {
                                                                    "browseEndpointContextMusicConfig": {
                                                                        "pageType": "MUSIC_PAGE_TYPE_PLAYLIST"
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }]
                                            }
                                        }
                                    ]
                                }
                            }
                        }
                    }]
                }
            }
        });
        assert_eq!(
            chart_video_playlist_id(&value).as_deref(),
            Some("PL4fGSI1pDJn6t3TXLGiiJdD-sZbrG3tG0")
        );
    }

    #[test]
    fn parses_artist_playlist_shelves() {
        use serde_json::json;

        let value = json!({
            "contents": [
                {
                    "musicCarouselShelfRenderer": {
                        "header": {
                            "musicCarouselShelfBasicHeaderRenderer": {
                                "title": { "runs": [{ "text": "Playlists by Rick Astley" }] }
                            }
                        },
                        "contents": [{
                            "musicTwoRowItemRenderer": {
                                "title": { "runs": [{ "text": "Rick Astley - Whenever" }] },
                                "subtitle": { "runs": [
                                    { "text": "Playlist" },
                                    { "text": " • " },
                                    { "text": "Rick Astley" }
                                ] },
                                "navigationEndpoint": {
                                    "browseEndpoint": {
                                        "browseId": "VLPL123",
                                        "browseEndpointContextSupportedConfigs": {
                                            "browseEndpointContextMusicConfig": {
                                                "pageType": "MUSIC_PAGE_TYPE_PLAYLIST"
                                            }
                                        }
                                    }
                                },
                                "thumbnailRenderer": {
                                    "musicThumbnailRenderer": {
                                        "thumbnail": { "thumbnails": [{ "url": "thumb" }] }
                                    }
                                }
                            }
                        }]
                    }
                },
                {
                    "musicCarouselShelfRenderer": {
                        "header": {
                            "musicCarouselShelfBasicHeaderRenderer": {
                                "title": { "runs": [{ "text": "Albums" }] }
                            }
                        },
                        "contents": [{
                            "musicTwoRowItemRenderer": {
                                "title": { "runs": [{ "text": "Album" }] },
                                "navigationEndpoint": {
                                    "browseEndpoint": {
                                        "browseId": "MPREb_x",
                                        "browseEndpointContextSupportedConfigs": {
                                            "browseEndpointContextMusicConfig": {
                                                "pageType": "MUSIC_PAGE_TYPE_ALBUM"
                                            }
                                        }
                                    }
                                }
                            }
                        }]
                    }
                }
            ]
        });
        let cards = raw_artist_playlist_cards(&value);
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0].id, "PL123");
        assert_eq!(cards[0].title, "Rick Astley - Whenever");
        assert_eq!(cards[0].subtitle, "Rick Astley");
        assert_eq!(cards[0].thumbnail, "thumb");
        assert_eq!(
            cards[0].url,
            "https://music.youtube.com/playlist?list=PL123"
        );
    }
}
