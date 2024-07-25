# GeoProx

A Geo-Proximity detection service for efficient real-time geo-aware contract pairing.

This is a service that allows you to determine which users are nearby a contract and can be applied to applications like Uber, Lyft, Grubhub.

[Discussed @ Rust Indy.rs meetup](https://gitlab.com/indyrs/may2024)

## Goals

- Keep track of **approximate location of riders**

- When order is placed should return **set of drivers nearby the pickup location**

## Todos

- Improve testing coverage

- Add horizontal scaling support

- Add client sdk