/* tslint:disable */
/* eslint-disable */
/**
 * geoprox-server
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.3.1
 * Contact: singhezra@gmail.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { mapValues } from '../runtime';
/**
 * Arguments for range query
 * @export
 * @interface QueryRange
 */
export interface QueryRange {
    /**
     * Maximum number of neighbors that can be returned (default 100)
     * @type {number}
     * @memberof QueryRange
     */
    count?: number | null;
    /**
     * Latitude
     * @type {number}
     * @memberof QueryRange
     */
    lat: number;
    /**
     * Longitude
     * @type {number}
     * @memberof QueryRange
     */
    lng: number;
    /**
     * Search radius in kilometers
     * @type {number}
     * @memberof QueryRange
     */
    range: number;
    /**
     * If enabled neighbors will be sorted by distance, nearest to furthest (default false)
     * @type {boolean}
     * @memberof QueryRange
     */
    sorted?: boolean | null;
}

/**
 * Check if a given object implements the QueryRange interface.
 */
export function instanceOfQueryRange(value: object): value is QueryRange {
    if (!('lat' in value) || value['lat'] === undefined) return false;
    if (!('lng' in value) || value['lng'] === undefined) return false;
    if (!('range' in value) || value['range'] === undefined) return false;
    return true;
}

export function QueryRangeFromJSON(json: any): QueryRange {
    return QueryRangeFromJSONTyped(json, false);
}

export function QueryRangeFromJSONTyped(json: any, ignoreDiscriminator: boolean): QueryRange {
    if (json == null) {
        return json;
    }
    return {
        
        'count': json['count'] == null ? undefined : json['count'],
        'lat': json['lat'],
        'lng': json['lng'],
        'range': json['range'],
        'sorted': json['sorted'] == null ? undefined : json['sorted'],
    };
}

export function QueryRangeToJSON(value?: QueryRange | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'count': value['count'],
        'lat': value['lat'],
        'lng': value['lng'],
        'range': value['range'],
        'sorted': value['sorted'],
    };
}

