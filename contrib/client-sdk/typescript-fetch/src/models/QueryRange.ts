/* tslint:disable */
/* eslint-disable */
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

import { mapValues } from '../runtime';
/**
 * Arguments for range query
 * @export
 * @interface QueryRange
 */
export interface QueryRange {
    /**
     * latitude
     * @type {number}
     * @memberof QueryRange
     */
    lat: number;
    /**
     * longitude
     * @type {number}
     * @memberof QueryRange
     */
    lng: number;
    /**
     * search radius in kilometers
     * @type {number}
     * @memberof QueryRange
     */
    range: number;
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
        
        'lat': json['lat'],
        'lng': json['lng'],
        'range': json['range'],
    };
}

export function QueryRangeToJSON(value?: QueryRange | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'lat': value['lat'],
        'lng': value['lng'],
        'range': value['range'],
    };
}

