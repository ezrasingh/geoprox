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

import org.geoprox.client.models.InsertKeyBatchResponse

class InsertKeyBatchResponseTest : ShouldSpec() {
    init {
        // uncomment below to create an instance of InsertKeyBatchResponse
        //val modelInstance = InsertKeyBatchResponse()

        // to test the property `errors` - Contains information about which keys failed to be inserted and the associated error details.
        should("test errors") {
            // uncomment below to test the property
            //modelInstance.errors shouldBe ("TODO")
        }

        // to test the property `results` - Object keys that have been inserted in the index and their geohash.
        should("test results") {
            // uncomment below to test the property
            //modelInstance.results shouldBe ("TODO")
        }

    }
}