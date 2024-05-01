# GeoProx

A Geo-Proximity detection service for efficient real-time geo-aware contract pairing.

This is a service that allows you to determine which users are nearby a contract and can be applied to applications like Uber, Lyft, Grubhub.

## Goals

- Should keep track of **approximate location of riders**

- When order is placed should return **set of drivers nearby the pickup location**

## Getting Started

```shell
# with cargo
cargo run -- --help

# or if you have just
just run
```

## API Endpoints

| Method   | Endpoint   | Payload                                            | Description          |
| -------- | ---------- | -------------------------------------------------- | -------------------- |
| `POST`   | `/rider/`  | `{ uid: number, position: [number, number] }`      | Place rider          |
| `DELETE` | `/rider/`  | `{ uid: number }`                                  | Remove rider         |
| `POST`   | `/search/` | `{ distance: number, position: [number, number] }` | Search nearby riders |
