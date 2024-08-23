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
 * Returns key and deletion status
 * @export
 * @interface RemoveKeyResponse
 */
export interface RemoveKeyResponse {
    /**
     * If true key was removed
     * @type {boolean}
     * @memberof RemoveKeyResponse
     */
    deleted: boolean;
    /**
     * Object key
     * @type {string}
     * @memberof RemoveKeyResponse
     */
    key: string;
}
