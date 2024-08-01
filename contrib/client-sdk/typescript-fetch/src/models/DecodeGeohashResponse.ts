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
 * Returns geohash decoded as latitude/longitude with precision errors
 * @export
 * @interface DecodeGeohashResponse
 */
export interface DecodeGeohashResponse {
    /**
     * Latitude
     * @type {number}
     * @memberof DecodeGeohashResponse
     */
    lat: number;
    /**
     * Latitude error
     * @type {number}
     * @memberof DecodeGeohashResponse
     */
    latError: number;
    /**
     * Longitude
     * @type {number}
     * @memberof DecodeGeohashResponse
     */
    lng: number;
    /**
     * Longitude error
     * @type {number}
     * @memberof DecodeGeohashResponse
     */
    lngError: number;
}

/**
 * Check if a given object implements the DecodeGeohashResponse interface.
 */
export function instanceOfDecodeGeohashResponse(value: object): value is DecodeGeohashResponse {
    if (!('lat' in value) || value['lat'] === undefined) return false;
    if (!('latError' in value) || value['latError'] === undefined) return false;
    if (!('lng' in value) || value['lng'] === undefined) return false;
    if (!('lngError' in value) || value['lngError'] === undefined) return false;
    return true;
}

export function DecodeGeohashResponseFromJSON(json: any): DecodeGeohashResponse {
    return DecodeGeohashResponseFromJSONTyped(json, false);
}

export function DecodeGeohashResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): DecodeGeohashResponse {
    if (json == null) {
        return json;
    }
    return {
        
        'lat': json['lat'],
        'latError': json['lat_error'],
        'lng': json['lng'],
        'lngError': json['lng_error'],
    };
}

export function DecodeGeohashResponseToJSON(value?: DecodeGeohashResponse | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'lat': value['lat'],
        'lat_error': value['latError'],
        'lng': value['lng'],
        'lng_error': value['lngError'],
    };
}
