//! Asset caching system

use crate::AssetError;
use mutsea_core::{Asset, AssetId};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Cached asset with timestamp
#[derive(Debug, Clone)]
struct CachedAsset {
    asset: Asset,
    cached_at: Instant,
    access_count: u64,
    last_accessed: Instant,
}

/// In-memory asset cache
pub struct AssetCache {
    cache: Arc<RwLock<HashMap<AssetId, CachedAsset>>>,
    max_size: usize,
    ttl: Duration,
}

impl AssetCache {
    /// Create a new asset cache
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            ttl,
        }
    }
    
    /// Get an asset from cache
    pub async fn get(&self, asset_id: AssetId) -> Option<Asset> {
        let mut cache = self.cache.write().await;
        
        if let Some(cached) = cache.get_mut(&asset_id) {
            // Check if expired
            if cached.cached_at.elapsed() > self.ttl {
                cache.remove(&asset_id);
                return None;
            }
            
            // Update access statistics
            cached.access_count += 1;
            cached.last_accessed = Instant::now();
            
            Some(cached.asset.clone())
        } else {
            None
        }
    }
    
    /// Put an asset in cache
    pub async fn put(&self, asset: Asset) {
        let mut cache = self.cache.write().await;
        
        // Check if we need to evict
        if cache.len() >= self.max_size {
            self.evict_lru(&mut cache);
        }
        
        let cached_asset = CachedAsset {
            asset: asset.clone(),
            cached_at: Instant::now(),
            access_count: 1,
            last_accessed: Instant::now(),
        };
        
        cache.insert(asset.id, cached_asset);
    }
    
    /// Remove an asset from cache
    pub async fn remove(&self, asset_id: AssetId) {
        let mut cache = self.cache.write().await;
        cache.remove(&asset_id);
    }
    
    /// Clear the cache
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
    
    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        
        let total_size: usize = cache.values().map(|c| c.asset.data.len()).sum();
        let avg_access_count = if cache.is_empty() {
            0.0
        } else {
            cache.values().map(|c| c.access_count).sum::<u64>() as f64 / cache.len() as f64
        };
        
        CacheStats {
            entry_count: cache.len(),
            total_size_bytes: total_size,
            max_entries: self.max_size,
            ttl_seconds: self.ttl.as_secs(),
            average_access_count: avg_access_count,
        }
    }
    
    /// Evict least recently used item
    fn evict_lru(&self, cache: &mut HashMap<AssetId, CachedAsset>) {
        if let Some((&lru_id, _)) = cache
            .iter()
            .min_by_key(|(_, cached)| cached.last_accessed)
        {
            cache.remove(&lru_id);
        }
    }
    
    /// Clean up expired entries
    pub async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        let now = Instant::now();
        
        cache.retain(|_, cached| now.duration_since(cached.cached_at) < self.ttl);
    }
    
    /// Start periodic cleanup task
    pub fn start_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
        let cache = Arc::clone(&self.cache);
        let ttl = self.ttl;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // Cleanup every 5 minutes
            
            loop {
                interval.tick().await;
                
                let mut cache_guard = cache.write().await;
                let now = Instant::now();
                let initial_size = cache_guard.len();
                
                cache_guard.retain(|_, cached| now.duration_since(cached.cached_at) < ttl);
                
                let removed = initial_size - cache_guard.len();
                if removed > 0 {
                    tracing::debug!("Cleaned up {} expired cache entries", removed);
                }
            }
        })
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entry_count: usize,
    pub total_size_bytes: usize,
    pub max_entries: usize,
    pub ttl_seconds: u64,
    pub average_access_count: f64,
}