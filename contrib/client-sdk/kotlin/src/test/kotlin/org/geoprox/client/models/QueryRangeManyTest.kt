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

import io.kotlintest.shouldBe
import io.kotlintest.specs.ShouldSpec

import org.geoprox.client.models.QueryRangeMany

class QueryRangeManyTest : ShouldSpec() {
    init {
        // uncomment below to create an instance of QueryRangeMany
        //val modelInstance = QueryRangeMany()

        // to test the property `indices` - List of indices to search
        should("test indices") {
            // uncomment below to test the property
            //modelInstance.indices shouldBe ("TODO")
        }

        // to test the property `lat` - Latitude
        should("test lat") {
            // uncomment below to test the property
            //modelInstance.lat shouldBe ("TODO")
        }

        // to test the property `lng` - Longitude
        should("test lng") {
            // uncomment below to test the property
            //modelInstance.lng shouldBe ("TODO")
        }

        // to test the property `range` - Search radius in kilometers
        should("test range") {
            // uncomment below to test the property
            //modelInstance.range shouldBe ("TODO")
        }

        // to test the property `count` - Maximum number of neighbors that can be returned (default 100)
        should("test count") {
            // uncomment below to test the property
            //modelInstance.count shouldBe ("TODO")
        }

        // to test the property `sorted` - If enabled neighbors will be sorted by distance, nearest to furthest (default false)
        should("test sorted") {
            // uncomment below to test the property
            //modelInstance.sorted shouldBe ("TODO")
        }

    }
}
