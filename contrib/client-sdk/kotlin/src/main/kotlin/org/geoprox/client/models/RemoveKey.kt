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
 * Arguments for removing a key
 *
 * @param key Object key
 */


data class RemoveKey (

    /* Object key */
    @Json(name = "key")
    val key: kotlin.String

) {


}
