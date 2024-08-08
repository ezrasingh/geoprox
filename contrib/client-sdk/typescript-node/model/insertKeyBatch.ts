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
import { InsertKey } from './insertKey';

/**
* Arguments for inserting multiple keys
*/
export class InsertKeyBatch {
    /**
    * Object key
    */
    'keys': Array<InsertKey>;
    'preserveOrder': boolean;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "keys",
            "baseName": "keys",
            "type": "Array<InsertKey>"
        },
        {
            "name": "preserveOrder",
            "baseName": "preserve_order",
            "type": "boolean"
        }    ];

    static getAttributeTypeMap() {
        return InsertKeyBatch.attributeTypeMap;
    }
}

