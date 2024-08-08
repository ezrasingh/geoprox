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
 * Returns index deletion status
 *
 * @param deleted If true index was deleted
 */


data class DropIndexResponse (

    /* If true index was deleted */
    @Json(name = "deleted")
    val deleted: kotlin.Boolean

) {


}

