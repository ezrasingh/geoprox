/**
 * geoprox-server
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.1
 * Contact: singhezra@gmail.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { RequestFile } from './models';

/**
* Neighboring geohash regions
*/
export class GeohashNeighborsResponse {
    /**
    * East
    */
    'e': string;
    /**
    * North
    */
    'n': string;
    /**
    * North East
    */
    'ne': string;
    /**
    * North West
    */
    'nw': string;
    /**
    * South
    */
    's': string;
    /**
    * South East
    */
    'se': string;
    /**
    * South West
    */
    'sw': string;
    /**
    * West
    */
    'w': string;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "e",
            "baseName": "e",
            "type": "string"
        },
        {
            "name": "n",
            "baseName": "n",
            "type": "string"
        },
        {
            "name": "ne",
            "baseName": "ne",
            "type": "string"
        },
        {
            "name": "nw",
            "baseName": "nw",
            "type": "string"
        },
        {
            "name": "s",
            "baseName": "s",
            "type": "string"
        },
        {
            "name": "se",
            "baseName": "se",
            "type": "string"
        },
        {
            "name": "sw",
            "baseName": "sw",
            "type": "string"
        },
        {
            "name": "w",
            "baseName": "w",
            "type": "string"
        }    ];

    static getAttributeTypeMap() {
        return GeohashNeighborsResponse.attributeTypeMap;
    }
}

