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
* Returns key and deletion status
*/
export class RemoveKeyResponse {
    /**
    * If true key was removed
    */
    'deleted': boolean;
    /**
    * resource key
    */
    'key': string;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "deleted",
            "baseName": "deleted",
            "type": "boolean"
        },
        {
            "name": "key",
            "baseName": "key",
            "type": "string"
        }    ];

    static getAttributeTypeMap() {
        return RemoveKeyResponse.attributeTypeMap;
    }
}

