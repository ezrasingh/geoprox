// tslint:disable
/**
 * geoprox-server
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

/**
 * Returns index creation status
 * @export
 * @interface CreateIndexResponse
 */
export interface CreateIndexResponse {
    /**
     * If true index was created
     * @type {boolean}
     * @memberof CreateIndexResponse
     */
    created: boolean;
    /**
     * If true index alredy exist
     * @type {boolean}
     * @memberof CreateIndexResponse
     */
    existed: boolean;
}
