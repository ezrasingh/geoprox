# How Geoprox Works

## The Problem

Initially, for the [May 2024 Rust Indy.rs meetup](https://gitlab.com/indyrs/may2024/-/blob/main/Geo-Proximity-Detection-With-Rust.pdf), the proof of concept problem began with ride-share pairing like Uber or Lyft.

Ride pairing depends on being able to track and pair a set of drivers within the vicinity of some location (i.e., the pickup location of an order).

Since then, the problem has generalized to how can we efficiently track and retrieve resources geographically near some location.

## Requirements

- Keep track of the _approximate_ location of objects.
- Search should return a set of objects within a radius of the search location.

## Solution

We index the objects and hash their key and geographical position. Using a [geohash](https://en.wikipedia.org/wiki/Geohash) to encode the approximate location, we can map geohash prefixes into the set of objects contained in the Geohash using a [Prefix Tree](https://en.wikipedia.org/wiki/Trie) stored in-memory.

We can efficiently partition the search space and perform a [nearest neighbor search](https://en.wikipedia.org/wiki/Nearest_neighbour_algorithm) on a merged set of objects in the search region and return the results.
