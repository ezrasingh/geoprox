// tslint:disable
/**
 * geoprox-server
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.5.0
 * Contact: singhezra@gmail.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

/**
 * Returns results and errors of batch key insert
 * @export
 * @interface InsertKeyBatchResponse
 */
export interface InsertKeyBatchResponse {
    /**
     * Contains information about which keys failed to be inserted and the associated error details.
     * @type {{ [key: string]: string; }}
     * @memberof InsertKeyBatchResponse
     */
    errors: { [key: string]: string; };
    /**
     * Object keys that have been inserted in the index and their geohash.
     * @type {{ [key: string]: string; }}
     * @memberof InsertKeyBatchResponse
     */
    results: { [key: string]: string; };
}
