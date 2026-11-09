//! Cached home-feed rows.
//!
//! The feed paints from here instantly on open (and offline), then refreshes
//! in the background. Rows are keyed by provider + kind; a provider's rows all
//! refresh together.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{JsonStore, StoreLocation};
use crate::providers::{HomeSection, HomeSectionKey};

/// How long a cached row may be served before a background refresh is due.
pub const HOME_CACHE_TTL_SECS: u64 = 6 * 60 * 60;

/// The cache is the same row type the Home view holds, keyed by
/// [`HomeSectionKey::cache_key`]; `HomeSection.key` is skipped on disk and
/// rebuilt by [`HomeCache::after_load`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HomeCache {
    pub sections: HashMap<String, HomeSection>,
}

impl JsonStore for HomeCache {
    const FILE: &'static str = "home_cache.json";
    const LOCATION: StoreLocation = StoreLocation::Cache;

    fn after_load(&mut self) {
        self.sections.retain(|key, section| {
            let Some(parsed) = HomeSectionKey::from_cache_key(key) else {
                return false;
            };
            section.key = parsed;
            true
        });
    }
}

impl HomeCache {
    /// Store an already-built row (the refresh path keeps its `fetched_at`).
    pub fn insert_row(&mut self, row: HomeSection) {
        self.sections.insert(row.key.cache_key(), row);
    }

    pub fn section(&self, key: HomeSectionKey) -> Option<&HomeSection> {
        self.sections.get(&key.cache_key())
    }

    /// The oldest fetch time among `keys`, if any is cached: the age of the
    /// whole feed is the age of its stalest row.
    pub fn oldest(&self, keys: impl IntoIterator<Item = HomeSectionKey>) -> Option<u64> {
        keys.into_iter()
            .filter_map(|key| self.section(key).map(|s| s.fetched_at))
            .min()
    }

    /// Drop every cached row of `provider`, for when the server stops existing
    /// under that identity (deleted, or edited into another kind).
    pub fn forget_provider(&mut self, provider: crate::providers::ProviderId) {
        let prefix = format!("{}:", provider.slug());
        let before = self.sections.len();
        self.sections.retain(|key, _| !key.starts_with(&prefix));
        if self.sections.len() != before {
            self.save();
        }
    }
}

pub fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// True when `fetched_at` is older than [`HOME_CACHE_TTL_SECS`].
pub fn is_stale(fetched_at: u64) -> bool {
    now_secs().saturating_sub(fetched_at) > HOME_CACHE_TTL_SECS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::{HomeSectionData, HomeSectionKind, ProviderId};

    fn data() -> HomeSectionData {
        HomeSectionData::Cards(Vec::new())
    }

    fn row(key: HomeSectionKey, title: &str, fetched_at: u64) -> HomeSection {
        HomeSection {
            key,
            title: title.into(),
            data: std::sync::Arc::new(data()),
            fetched_at,
        }
    }

    #[test]
    fn a_row_is_keyed_by_provider_and_kind() {
        let mut cache = HomeCache::default();
        cache.insert_row(row(
            HomeSectionKey::new(ProviderId::YouTube, HomeSectionKind::QuickPicks),
            "",
            1_000,
        ));
        assert!(cache
            .section(HomeSectionKey::new(
                ProviderId::YouTube,
                HomeSectionKind::QuickPicks
            ))
            .is_some());
        assert!(cache
            .section(HomeSectionKey::new(
                ProviderId::Bandcamp,
                HomeSectionKind::QuickPicks
            ))
            .is_none());
        assert!(cache
            .section(HomeSectionKey::new(
                ProviderId::YouTube,
                HomeSectionKind::Trending
            ))
            .is_none());
    }

    #[test]
    fn oldest_picks_the_stalest_cached_row() {
        let mut cache = HomeCache::default();
        let youtube = HomeSectionKey::new(ProviderId::YouTube, HomeSectionKind::QuickPicks);
        let bandcamp = HomeSectionKey::new(ProviderId::Bandcamp, HomeSectionKind::NewReleases);
        cache.insert_row(row(youtube, "", 100));
        cache.insert_row(row(bandcamp, "", 900));
        assert_eq!(cache.oldest([youtube, bandcamp]), Some(100));
        assert_eq!(
            cache.oldest([HomeSectionKey::new(
                ProviderId::LastFm,
                HomeSectionKind::TopArtists
            )]),
            None
        );
    }

    #[test]
    fn rows_expire_after_the_ttl() {
        let now = now_secs();
        assert!(!is_stale(now));
        assert!(is_stale(now.saturating_sub(HOME_CACHE_TTL_SECS + 1)));
    }

    #[test]
    fn after_load_rebuilds_the_runtime_key_from_the_map_key() {
        let mut cache = HomeCache::default();
        let key = HomeSectionKey::new(ProviderId::Bandcamp, HomeSectionKind::NewReleases);
        cache.insert_row(row(key, "New", 42));
        let json = serde_json::to_string(&cache).unwrap();
        let mut restored: HomeCache = serde_json::from_str(&json).unwrap();
        assert_ne!(restored.section(key).unwrap().key, key);
        restored.after_load();
        assert_eq!(restored.section(key).unwrap().key, key);
        assert_eq!(restored.section(key).unwrap().fetched_at, 42);
    }

    #[test]
    fn after_load_drops_rows_with_unparseable_keys() {
        let mut cache = HomeCache::default();
        cache.sections.insert(
            "not-a-provider:not-a-kind".into(),
            HomeSection {
                key: HomeSectionKey::default(),
                title: String::new(),
                data: std::sync::Arc::new(data()),
                fetched_at: 1,
            },
        );
        cache.after_load();
        assert!(cache.sections.is_empty());
    }
}
