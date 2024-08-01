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

import { RequestFile } from './models';

/**
* Returns geohash decoded as latitude/longitude with precision errors
*/
export class DecodeGeohashResponse {
    /**
    * Latitude
    */
    'lat': number;
    /**
    * Latitude error
    */
    'latError': number;
    /**
    * Longitude
    */
    'lng': number;
    /**
    * Longitude error
    */
    'lngError': number;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "lat",
            "baseName": "lat",
            "type": "number"
        },
        {
            "name": "latError",
            "baseName": "lat_error",
            "type": "number"
        },
        {
            "name": "lng",
            "baseName": "lng",
            "type": "number"
        },
        {
            "name": "lngError",
            "baseName": "lng_error",
            "type": "number"
        }    ];

    static getAttributeTypeMap() {
        return DecodeGeohashResponse.attributeTypeMap;
    }
}
