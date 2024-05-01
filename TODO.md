# Todos

- Improve testing coverage

  - Examine potential edge cases (i.e geohash region boundary)

- Determine horizontal scaling strategy

  - Possibly distributed in-memory cache could work
  - Another idea is distribute state via a consensus algorithm like Raft
  - Long term persistence may be achieved with a NoSQL solution

- Add gRPC support and OpenAPI spec schema (w/ codegen)

- Rename semantics to generalize the API conventions

  - "order" to "contract"
  - "rider" to "user"

- Possibly handle internal queue state for riders (when active and not active)

- Possibly leverage SSE for `place_order` in case a rider is not found at first.
