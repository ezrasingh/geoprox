// tslint:disable
/**
 * geoprox-server
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.1
 * Contact: singhezra@gmail.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

/**
 * Returns geohash encoded latitude/longitude
 * @export
 * @interface EncodeLatLngResponse
 */
export interface EncodeLatLngResponse {
    /**
     * @type {string}
     * @memberof EncodeLatLngResponse
     */
    geohash: string;
}
