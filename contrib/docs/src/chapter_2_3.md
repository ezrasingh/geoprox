# Insertion and Query

This section outlines the fundamental design strategy behind Geoprox, detailing the algorithms used for inserting and querying geospatial data.

The core approach involves encoding geographic coordinates into geohashes, utilizing a [Patricia Trie](https://en.wikipedia.org/wiki/Radix_tree#Variants) for efficient storage and retrieval, and employing a [KD-Tree](https://en.wikipedia.org/wiki/K-d_tree) for fast [nearest neighbor searches](https://en.wikipedia.org/wiki/Nearest_neighbor_search). These techniques collectively enable Geoprox to efficiently manage and query large volumes of geospatial data.

## Data Structures

- **Approximate Rider Position Tracking:**

  - Utilize a [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)[\* see v0.3.0 release notes](https://github.com/ezrasingh/geoprox/releases/tag/v0.3.0) to track approximate rider positions.
  - Format: `object_key` => `geohash(lat, lng)`

- **Key/Value Storage:**
  - Use [`StringPatriciaMap`](https://docs.rs/patricia_tree/0.8.0/patricia_tree/map/type.StringPatriciaMap.html) for efficient key/value storage.
  - Format: `geohash_region` => `HashSet(object_key)`

## Insert Procedure

1. **Hash Object's Latitude/Longitude:**

   - Hash the object's latitude and longitude up to a certain depth (e.g., 6). This depth, called `insert_depth`, should be sufficient to ensure the resulting geohash region is large enough to store the density of objects in that area.

2. **Upsert Object Key:**
   - Insert or update the `object_key` in the `StringPatriciaMap` with its `geohash_region`.
   - If `object_key` had an existing `geohash_region`, remove its entry from that region in the `StringPatriciaMap`.

## Search Procedure

1. **Initiate Search:**

   - A search is initiated at a specific location with a given search range (in kilometers).

2. **Encode Latitude/Longitude:**

   - Encode the latitude and longitude into a geohash up to a specific depth (e.g., 6). This depth, called `search_depth`, should ensure the geohash region is small enough to fit within the search radius.

3. **Optimize Geohash Search Region:**

   - Optimize the geohash search region by expanding it until it covers the search area.
   - If the geohash search region does not contain the search radius, remove the last character (effectively expanding the region) until it fits within the search area.

4. **Partition `StringPatriciaMap`:**

   - Partition the `StringPatriciaMap` by the common prefix of the optimized geohash.

5. **Construct [`KdTree`](https://docs.rs/kiddo/4.2.0/kiddo/type.KdTree.html):**

   - Iterate over the partition and construct a `KdTree`.
   - Store objects and their approximate locations in the `KdTree`.

6. **Compute Nearest Neighbor:**
   - Perform nearest neighbor computation with the search requirements against the `KdTree` to get the final result.
