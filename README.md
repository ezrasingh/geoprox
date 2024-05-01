# GeoProx

A Geo-Proximity detection service for efficient real-time geo-aware contract pairing.

This is a service that allows you to determine which users are nearby a contract and can be applied to applications like Uber, Lyft, Grubhub.

## Goals

- Keep track of **approximate location of riders**

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

## Todos

- Improve testing coverage

  - Examine potential edge cases (i.e geohash region boundary)

- Determine horizontal scaling strategy

  - Distributed in-memory cache could work
  - Another idea is distribute state via a consensus algorithm like Raft
  - Long term persistence may be achieved with a NoSQL solution

- Add gRPC support and OpenAPI spec schema (w/ codegen)

- Rename semantics to generalize the API conventions

  - "order" to "contract"
  - "rider" to "user"

- Possibly handle internal queue state for riders (when active and not active)

- Possibly leverage SSE for `place_order` in case a rider is not found at first.
