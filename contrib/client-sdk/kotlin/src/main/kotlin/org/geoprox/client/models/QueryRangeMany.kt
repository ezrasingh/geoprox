/**
 *
 * Please note:
 * This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * Do not edit this file manually.
 *
 */

@file:Suppress(
    "ArrayInDataClass",
    "EnumEntryName",
    "RemoveRedundantQualifierName",
    "UnusedImport"
)

package org.geoprox.client.models


import com.squareup.moshi.Json
import com.squareup.moshi.JsonClass

/**
 * Arguments for group range query
 *
 * @param indices List of indices to search
 * @param lat Latitude
 * @param lng Longitude
 * @param range Search radius in kilometers
 * @param count Maximum number of neighbors that can be returned (default 100)
 * @param sorted If enabled neighbors will be sorted by distance, nearest to furthest (default false)
 */


data class QueryRangeMany (

    /* List of indices to search */
    @Json(name = "indices")
    val indices: kotlin.collections.List<kotlin.String>,

    /* Latitude */
    @Json(name = "lat")
    val lat: kotlin.Double,

    /* Longitude */
    @Json(name = "lng")
    val lng: kotlin.Double,

    /* Search radius in kilometers */
    @Json(name = "range")
    val range: kotlin.Int,

    /* Maximum number of neighbors that can be returned (default 100) */
    @Json(name = "count")
    val count: kotlin.Int? = null,

    /* If enabled neighbors will be sorted by distance, nearest to furthest (default false) */
    @Json(name = "sorted")
    val sorted: kotlin.Boolean? = null

) {


}

