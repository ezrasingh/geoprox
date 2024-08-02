use geoprox_core::models::Neighbor;
use serde::{Deserialize, Serialize};
use utoipa::{openapi, IntoParams, ToResponse, ToSchema};

/// Returns geohash decoded as latitude/longitude with precision errors
#[derive(Serialize, ToSchema, ToResponse)]
pub struct DecodeGeohashResponse {
    /// Latitude
    pub lat: f64,
    /// Latitude error
    pub lat_error: f64,
    /// Longitude
    pub lng: f64,
    /// Longitude error
    pub lng_error: f64,
}

/// Arguments for encoding latitude/longitude as geohash
#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct EncodeLatLng {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
    /// Determines geohash length
    #[schema(minimum = 1, maximum = 10)]
    pub depth: usize,
}

/// Returns geohash encoded latitude/longitude
#[derive(Serialize, ToSchema, ToResponse)]
pub struct EncodeLatLngResponse {
    pub geohash: String,
}

/// Neighboring geohash regions
#[derive(Serialize, ToSchema, ToResponse)]
pub struct GeohashNeighborsResponse {
    /// South West
    pub sw: String,
    /// South
    pub s: String,
    /// South East
    pub se: String,
    /// West
    pub w: String,
    /// East
    pub e: String,
    /// North West
    pub nw: String,
    /// North
    pub n: String,
    /// North East
    pub ne: String,
}

impl From<geoprox_core::geohash::Neighbors> for GeohashNeighborsResponse {
    fn from(value: geoprox_core::geohash::Neighbors) -> Self {
        Self {
            sw: value.sw,
            s: value.s,
            se: value.se,
            w: value.w,
            e: value.e,
            nw: value.nw,
            n: value.n,
            ne: value.ne,
        }
    }
}

/// Returns index creation status
#[derive(Serialize, ToSchema, ToResponse)]
pub struct CreateIndexResponse {
    /// If true index was created
    pub created: bool,
    /// If true index alredy exist
    pub existed: bool,
}

/// Arguments for inserting a key
#[derive(Deserialize, ToSchema)]
pub struct InsertKey {
    /// Resource key
    pub key: String,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
}

/// Returns key and geohash
#[derive(Serialize, ToSchema, ToResponse)]
pub struct InsertKeyResponse {
    /// Resource key
    pub key: String,
    /// Geohash encoded latitude/longitude
    pub geohash: String,
}

/// Arguments for removing a key
#[derive(Deserialize, ToSchema)]
pub struct RemoveKey {
    /// Resource key
    pub key: String,
}

/// Returns key and deletion status
#[derive(Serialize, ToSchema, ToResponse)]
pub struct RemoveKeyResponse {
    /// Resource key
    pub key: String,
    /// If true key was removed
    pub deleted: bool,
}

/// Returns index deletion status
#[derive(Serialize, ToSchema, ToResponse)]
pub struct DropIndexResponse {
    /// If true index was deleted
    pub deleted: bool,
}

/// Arguments for range query
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct QueryRange {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
    /// Search radius in kilometers
    #[schema(minimum = 0, maximum = 0xFFFF)]
    pub range: u16,
}

#[derive(Serialize)]
pub struct KeysFound(pub Vec<Neighbor>);

impl<'__s> ToSchema<'__s> for KeysFound {
    fn schema() -> (&'__s str, openapi::RefOr<openapi::schema::Schema>) {
        (
            "KeysFound",
            openapi::ArrayBuilder::new()
                .unique_items(true)
                .items(
                    openapi::ObjectBuilder::new()
                        .property(
                            "key",
                            openapi::ObjectBuilder::new().schema_type(openapi::SchemaType::String),
                        )
                        .property(
                            "distance",
                            openapi::ObjectBuilder::new()
                                .schema_type(openapi::SchemaType::Number)
                                .format(Some(openapi::SchemaFormat::KnownFormat(
                                    openapi::KnownFormat::Float,
                                )))
                                .description(Some("Distance in kilometers")),
                        ),
                )
                .description(Some("Resource keys found within range"))
                .build()
                .into(),
        )
    }
}

/// Returns resource keys found with their distance
#[derive(Serialize, ToSchema, ToResponse)]
pub struct QueryRangeResponse {
    /// Resource keys found within range
    pub found: KeysFound,
}
