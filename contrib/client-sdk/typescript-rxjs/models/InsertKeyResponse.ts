// tslint:disable
/**
 * geoprox-server
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.2
 * Contact: singhezra@gmail.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

/**
 * Returns key and geohash
 * @export
 * @interface InsertKeyResponse
 */
export interface InsertKeyResponse {
    /**
     * Geohash encoded latitude/longitude
     * @type {string}
     * @memberof InsertKeyResponse
     */
    geohash: string;
    /**
     * Object key
     * @type {string}
     * @memberof InsertKeyResponse
     */
    key: string;
}
