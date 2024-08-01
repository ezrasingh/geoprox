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
 * Neighboring geohash regions
 * @export
 * @interface GeohashNeighborsResponse
 */
export interface GeohashNeighborsResponse {
    /**
     * East
     * @type {string}
     * @memberof GeohashNeighborsResponse
     */
    e: string;
    /**
     * North
     * @type {string}
     * @memberof GeohashNeighborsResponse
     */
    n: string;
    /**
     * North East
     * @type {string}
     * @memberof GeohashNeighborsResponse
     */
    ne: string;
    /**
     * North West
     * @type {string}
     * @memberof GeohashNeighborsResponse
     */
    nw: string;
    /**
     * South
     * @type {string}
     * @memberof GeohashNeighborsResponse
     */
    s: string;
    /**
     * South East
     * @type {string}
     * @memberof GeohashNeighborsResponse
     */
    se: string;
    /**
     * South West
     * @type {string}
     * @memberof GeohashNeighborsResponse
     */
    sw: string;
    /**
     * West
     * @type {string}
     * @memberof GeohashNeighborsResponse
     */
    w: string;
}

/**
 * Check if a given object implements the GeohashNeighborsResponse interface.
 */
export function instanceOfGeohashNeighborsResponse(value: object): value is GeohashNeighborsResponse {
    if (!('e' in value) || value['e'] === undefined) return false;
    if (!('n' in value) || value['n'] === undefined) return false;
    if (!('ne' in value) || value['ne'] === undefined) return false;
    if (!('nw' in value) || value['nw'] === undefined) return false;
    if (!('s' in value) || value['s'] === undefined) return false;
    if (!('se' in value) || value['se'] === undefined) return false;
    if (!('sw' in value) || value['sw'] === undefined) return false;
    if (!('w' in value) || value['w'] === undefined) return false;
    return true;
}

export function GeohashNeighborsResponseFromJSON(json: any): GeohashNeighborsResponse {
    return GeohashNeighborsResponseFromJSONTyped(json, false);
}

export function GeohashNeighborsResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): GeohashNeighborsResponse {
    if (json == null) {
        return json;
    }
    return {
        
        'e': json['e'],
        'n': json['n'],
        'ne': json['ne'],
        'nw': json['nw'],
        's': json['s'],
        'se': json['se'],
        'sw': json['sw'],
        'w': json['w'],
    };
}

export function GeohashNeighborsResponseToJSON(value?: GeohashNeighborsResponse | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'e': value['e'],
        'n': value['n'],
        'ne': value['ne'],
        'nw': value['nw'],
        's': value['s'],
        'se': value['se'],
        'sw': value['sw'],
        'w': value['w'],
    };
}
