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
 * Arguments for encoding latitude/longitude as geohash
 * @export
 * @interface EncodeLatLng
 */
export interface EncodeLatLng {
    /**
     * Determines geohash length
     * @type {number}
     * @memberof EncodeLatLng
     */
    depth: number;
    /**
     * Latitude
     * @type {number}
     * @memberof EncodeLatLng
     */
    lat: number;
    /**
     * Longitude
     * @type {number}
     * @memberof EncodeLatLng
     */
    lng: number;
}
