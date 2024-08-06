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
 * Arguments for inserting a key
 * @export
 * @interface InsertKey
 */
export interface InsertKey {
    /**
     * Resource key
     * @type {string}
     * @memberof InsertKey
     */
    key: string;
    /**
     * Latitude
     * @type {number}
     * @memberof InsertKey
     */
    lat: number;
    /**
     * Longitude
     * @type {number}
     * @memberof InsertKey
     */
    lng: number;
}

/**
 * Check if a given object implements the InsertKey interface.
 */
export function instanceOfInsertKey(value: object): value is InsertKey {
    if (!('key' in value) || value['key'] === undefined) return false;
    if (!('lat' in value) || value['lat'] === undefined) return false;
    if (!('lng' in value) || value['lng'] === undefined) return false;
    return true;
}

export function InsertKeyFromJSON(json: any): InsertKey {
    return InsertKeyFromJSONTyped(json, false);
}

export function InsertKeyFromJSONTyped(json: any, ignoreDiscriminator: boolean): InsertKey {
    if (json == null) {
        return json;
    }
    return {
        
        'key': json['key'],
        'lat': json['lat'],
        'lng': json['lng'],
    };
}

export function InsertKeyToJSON(value?: InsertKey | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'key': value['key'],
        'lat': value['lat'],
        'lng': value['lng'],
    };
}

