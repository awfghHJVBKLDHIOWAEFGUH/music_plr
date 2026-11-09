use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{JsonStore, StoreLocation};
use crate::types::Track;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadEntry {
    pub track: Track,
    pub registered_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DownloadRegistry {
    entries: HashMap<String, DownloadEntry>,
}

/// Pre-`entries` `downloads.json`: tracks and their timestamps lived in two
/// parallel maps. Kept only as a deserialization shape for [`load_migrated`].
#[derive(Debug, Deserialize)]
struct LegacyDownloadRegistry {
    tracks: HashMap<String, Track>,
    #[serde(default)]
    registered_at: HashMap<String, u64>,
}

impl LegacyDownloadRegistry {
    fn into_registry(self) -> DownloadRegistry {
        let mut entries = HashMap::with_capacity(self.tracks.len());
        for (old_key, track) in self.tracks {
            let key = track.cache_key();
            let registered_at = self.registered_at.get(&old_key).copied().unwrap_or(0);
            entries.entry(key).or_insert(DownloadEntry {
                track,
                registered_at,
            });
        }
        DownloadRegistry { entries }
    }
}

/// The current file or its legacy shape. `DownloadRegistry::entries` is
/// required, so an old `tracks`/`registered_at` file falls through to `Legacy`
/// instead of being accepted as an empty current registry.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StoredDownloads {
    Current(DownloadRegistry),
    Legacy(LegacyDownloadRegistry),
}

impl JsonStore for DownloadRegistry {
    const FILE: &'static str = "downloads.json";
    const LOCATION: StoreLocation = StoreLocation::Data;
}

impl DownloadRegistry {
    /// Load the registry, converting a legacy two-map file into `entries` and
    /// rewriting its pre-slug `Debug:id` keys (e.g. `YouTube:…`, `LastFm:…`)
    /// to the `slug:id` format used by [`Track::cache_key`]. Current files are
    /// already keyed that way and load as-is.
    pub fn load_migrated() -> Self {
        let stored = std::fs::read_to_string(Self::path())
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok());
        match stored {
            Some(StoredDownloads::Current(registry)) => registry,
            Some(StoredDownloads::Legacy(legacy)) => {
                let store = legacy.into_registry();
                store.save();
                store
            }
            None => Self::default(),
        }
    }

    pub fn register(&mut self, track: Track) {
        let key = track.cache_key();
        self.entries.insert(
            key,
            DownloadEntry {
                track,
                registered_at: crate::data::home_cache::now_secs(),
            },
        );
        self.save();
    }

    pub fn remove(&mut self, key: &str) -> Option<Track> {
        let result = self.entries.remove(key).map(|entry| entry.track);
        self.save();
        result
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Returns the on-disk path of the downloaded audio file for `key`, if the
    /// track is registered and was downloaded to a known location.
    pub fn path_for(&self, key: &str) -> Option<String> {
        self.entries
            .get(key)
            .and_then(|entry| entry.track.local_path())
    }

    /// Every registered track, in no particular order, for callers that only
    /// need the set (thumbnail seeding).
    pub fn iter(&self) -> impl Iterator<Item = &Track> {
        self.entries.values().map(|entry| &entry.track)
    }

    /// Every registered track, newest download first. The home rail's row
    /// indices are built from this order, so drawing and playing agree; the
    /// Downloads view keeps it too, so both list the same thing the same way.
    pub fn tracks(&self) -> Vec<&Track> {
        let mut entries: Vec<(&String, &DownloadEntry)> = self.entries.iter().collect();
        entries.sort_by(|(a_key, a), (b_key, b)| {
            b.registered_at
                .cmp(&a.registered_at)
                .then_with(|| a_key.cmp(b_key))
        });
        entries.into_iter().map(|(_, entry)| &entry.track).collect()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn legacy_file_keeps_its_tracks_and_timestamps() {
        let mut tracks = HashMap::new();
        tracks.insert("YouTube:abc".to_string(), Track::default());
        let mut registered_at = HashMap::new();
        registered_at.insert("YouTube:abc".to_string(), 7);
        let json = serde_json::to_string(&json!({
            "tracks": tracks,
            "registered_at": registered_at,
        }))
        .unwrap();

        let stored: StoredDownloads = serde_json::from_str(&json).unwrap();
        let StoredDownloads::Legacy(legacy) = stored else {
            panic!("an old file should parse as the legacy shape");
        };
        let registry = legacy.into_registry();
        let entry = registry.entries.get("youtube:").unwrap();
        assert_eq!(entry.registered_at, 7);
    }

    #[test]
    fn current_file_parses_as_entries() {
        let mut entries = HashMap::new();
        entries.insert(
            "youtube:".to_string(),
            DownloadEntry {
                track: Track::default(),
                registered_at: 7,
            },
        );
        let json = serde_json::to_string(&json!({ "entries": entries })).unwrap();

        let stored: StoredDownloads = serde_json::from_str(&json).unwrap();
        let StoredDownloads::Current(registry) = stored else {
            panic!("a current file should parse as the current shape");
        };
        assert_eq!(registry.entries.get("youtube:").unwrap().registered_at, 7);
    }
}
