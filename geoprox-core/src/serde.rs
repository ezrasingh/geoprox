use std::time::{Duration, Instant, SystemTime};

use serde::{Deserialize, Serialize};

use crate::index::SpatialIndex;

impl Serialize for SpatialIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let sys_now = SystemTime::now();
        let now = Instant::now();
        self.objects()
            .into_iter()
            .map(|(key, ghash, ttl)| (key, ghash, ttl.map(|expire_at| sys_now - (now - expire_at))))
            .collect::<Vec<(String, String, Option<SystemTime>)>>()
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SpatialIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let keys: Vec<(String, String, Option<SystemTime>)> = Vec::deserialize(deserializer)?;
        let sys_now = SystemTime::now();

        let mut index = SpatialIndex::default();
        keys.into_iter().for_each(|(key, ghash, sys_ttl)| {
            let expiration = sys_ttl.map(|ttl| {
                sys_now
                    .duration_since(ttl)
                    .unwrap_or(Duration::from_secs(0))
            });
            index.insert(&key, &ghash, expiration);
        });

        Ok(index)
    }
}
