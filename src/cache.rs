//! Cache module for status mode
//! Stores API responses to avoid frequent API calls

use std::path::PathBuf;
use std::fs;

use crate::models::QuotaLimitResponse;

/// Cache entry with timestamp
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CacheEntry {
    /// Unix timestamp when the cache was written
    pub timestamp: i64,
    /// The cached quota data
    pub data: QuotaLimitResponse,
}

/// Get the cache file path: ~/.cache/glm-usage-monitor/status.json
pub fn cache_path() -> PathBuf {
    let cache_dir = std::env::var("XDG_CACHE_HOME")
        .ok()
        .filter(|p| !p.is_empty())
        .map(|p| PathBuf::from(p))
        .unwrap_or_else(|| {
            dirs::home_dir()
                .expect("Failed to get home directory")
                .join(".cache")
        })
        .join("glm-usage-monitor");

    cache_dir.join("status.json")
}

/// Read cache from disk
pub fn read_cache() -> Option<CacheEntry> {
    let path = cache_path();
    if !path.exists() {
        return None;
    }

    let content = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

/// Write cache to disk
pub fn write_cache(data: &QuotaLimitResponse) {
    let path = cache_path();

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let entry = CacheEntry {
        timestamp: chrono::Utc::now().timestamp(),
        data: data.clone(),
    };

    if let Ok(json) = serde_json::to_string(&entry) {
        let _ = fs::write(&path, json);
    }
}

/// Check if cache entry is fresh (within TTL)
pub fn is_fresh(entry: &CacheEntry, ttl_secs: u64) -> bool {
    let now = chrono::Utc::now().timestamp();
    let elapsed = now - entry.timestamp;
    elapsed >= 0 && (elapsed as u64) < ttl_secs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_fresh() {
        let now = chrono::Utc::now().timestamp();
        let fresh_entry = CacheEntry {
            timestamp: now - 100, // 100 seconds ago
            data: QuotaLimitResponse { limits: vec![] },
        };
        let stale_entry = CacheEntry {
            timestamp: now - 400, // 400 seconds ago
            data: QuotaLimitResponse { limits: vec![] },
        };

        assert!(is_fresh(&fresh_entry, 300)); // 5 min TTL
        assert!(!is_fresh(&stale_entry, 300));
    }
}
