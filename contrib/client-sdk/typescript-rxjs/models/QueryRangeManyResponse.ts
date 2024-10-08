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

import type {
    Neighbor,
} from './';

/**
 * Returns indexes and object keys found with their distance
 * @export
 * @interface QueryRangeManyResponse
 */
export interface QueryRangeManyResponse {
    /**
     * Contains information about any errors occured during batch search.
     * @type {{ [key: string]: string; }}
     * @memberof QueryRangeManyResponse
     */
    errors: { [key: string]: string; };
    /**
     * Object keys found within range
     * @type {{ [key: string]: Array<Neighbor>; }}
     * @memberof QueryRangeManyResponse
     */
    results: { [key: string]: Array<Neighbor>; };
}
