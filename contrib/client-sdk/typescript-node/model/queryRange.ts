/**
 * geoprox-server
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.0
 * Contact: singhezra@gmail.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { RequestFile } from './models';

/**
* Arguments for range query
*/
export class QueryRange {
    /**
    * Maximum number of neighbors that can be returned (default 100)
    */
    'count'?: number | null;
    /**
    * Latitude
    */
    'lat': number;
    /**
    * Longitude
    */
    'lng': number;
    /**
    * Search radius in kilometers
    */
    'range': number;
    /**
    * If enabled neighbors will be sorted by distance, nearest to furthest (default false)
    */
    'sorted'?: boolean | null;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "count",
            "baseName": "count",
            "type": "number"
        },
        {
            "name": "lat",
            "baseName": "lat",
            "type": "number"
        },
        {
            "name": "lng",
            "baseName": "lng",
            "type": "number"
        },
        {
            "name": "range",
            "baseName": "range",
            "type": "number"
        },
        {
            "name": "sorted",
            "baseName": "sorted",
            "type": "boolean"
        }    ];

    static getAttributeTypeMap() {
        return QueryRange.attributeTypeMap;
    }
}

