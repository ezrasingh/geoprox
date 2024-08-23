# Configuration

Geoprox `run` command allows specifying a configuration file using the `-c` or `--config` option or set the `GEOPROX_CONFIG` environment variable. This file can contain various settings to customize the behavior of the Geoprox server and commands. The configuration can be provided in any common format such as `YAML`, `TOML`, `JSON`, or `INI`.

## Example Usage

Specify config using `-c` flag:

```sh
# from current working directory
geoprox run -c geoprox.toml

# from absolute path
geoprox run -c /my/custom/config.yaml
```

or using environment variables:

```sh
export GEOPROX_CONFIG='/my/custom/config.json'

geoprox run
```

## Example Config

Here's an example configuration file in `TOML` format:

```toml
# All settings are optional; these are the default values.
[server]
# The address the server will bind to
http_addr = '0.0.0.0'
# The port the server will listen on
http_port = 5000
# Request timeout
timeout = '10s'

[shard]
# Determines the default geohash length for inserts
insert_depth = 6
# Determines the default geohash length for searches
search_depth = 6
# Specifies the default number of results returned in range queries
default_count = 100
# Toggles the default sorting behavior for query results
default_sorted = false

[server.snapshots]
# Toggle snapshot usage
disabled = false
# Directory where snapshots will be stored
path = '/var/lib/geoprox/snapshots'
# How often snapshots will be taken
every = '30s'
```

### Environment Variables

These are the currently supported environment variables. They will take precedence over settings defined in the configuration file.

| Environment Variable | Description                            | Default Value |
| -------------------- | -------------------------------------- | ------------- |
| `GEOPROX_CONFIG`     | Specifies the configuration file path. |               |
| `GEOPROX_HTTP_ADDR`  | The address the server will bind to.   | `0.0.0.0`     |
| `GEOPROX_HTTP_PORT`  | The port the server will listen on.    | `5000`        |

## Fine Tuning

The `insert_depth` and `search_depth` settings control the precision of geohashing and impact the performance of distance calculations and search queries.

### Optimizing `insert_depth`

**Higher Insert Depth:**

- **Description:** Objects are grouped into smaller, more precise regions.
- **Impact:** Searching takes slightly longer, but distance calculations are more accurate.
- **Use Case:** Ideal for scenarios where precise distance measurements are crucial.

**Lower Insert Depth:**

- **Description:** Objects are grouped into larger regions.
- **Impact:** Search performance improves, but distance calculations become less accurate.
- **Use Case:** Best for cases where fast general distance estimates are acceptable.

### Optimizing `search_depth`

**Higher Search Depth:**

- **Description:** The initial search region is smaller and more precise.
- **Impact:** Search queries take slightly longer, but results are more accurate.
- **Use Case:** Suitable for narrow range queries where high accuracy is needed.

**Lower Search Depth:**

- **Description:** The initial search region is larger, leading to faster searches.
- **Impact:** Search speed improves, but accuracy may be reduced.
- **Use Case:** Ideal for wide range queries where speed is prioritized over precision.
