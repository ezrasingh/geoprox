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

import { RequestFile } from './models';

/**
* Returns index creation status
*/
export class CreateIndexResponse {
    /**
    * If true index was created
    */
    'created': boolean;
    /**
    * If true index alredy exist
    */
    'existed': boolean;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "created",
            "baseName": "created",
            "type": "boolean"
        },
        {
            "name": "existed",
            "baseName": "existed",
            "type": "boolean"
        }    ];

    static getAttributeTypeMap() {
        return CreateIndexResponse.attributeTypeMap;
    }
}

