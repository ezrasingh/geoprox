/**
 * geoprox-server
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.2.0
 * Contact: singhezra@gmail.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { RequestFile } from './models';
import { Neighbor } from './neighbor';

/**
* Returns resource keys found with their distance
*/
export class QueryRangeResponse {
    /**
    * Resource keys found within range
    */
    'found': Array<Neighbor>;

    static discriminator: string | undefined = undefined;

    static attributeTypeMap: Array<{name: string, baseName: string, type: string}> = [
        {
            "name": "found",
            "baseName": "found",
            "type": "Array<Neighbor>"
        }    ];

    static getAttributeTypeMap() {
        return QueryRangeResponse.attributeTypeMap;
    }
}

