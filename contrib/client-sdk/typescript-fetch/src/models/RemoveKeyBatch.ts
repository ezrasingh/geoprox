/* tslint:disable */
/* eslint-disable */
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

import { mapValues } from '../runtime';
/**
 * Arguments for removing multiple keys
 * @export
 * @interface RemoveKeyBatch
 */
export interface RemoveKeyBatch {
    /**
     * Object key
     * @type {Array<string>}
     * @memberof RemoveKeyBatch
     */
    keys: Array<string>;
}

/**
 * Check if a given object implements the RemoveKeyBatch interface.
 */
export function instanceOfRemoveKeyBatch(value: object): value is RemoveKeyBatch {
    if (!('keys' in value) || value['keys'] === undefined) return false;
    return true;
}

export function RemoveKeyBatchFromJSON(json: any): RemoveKeyBatch {
    return RemoveKeyBatchFromJSONTyped(json, false);
}

export function RemoveKeyBatchFromJSONTyped(json: any, ignoreDiscriminator: boolean): RemoveKeyBatch {
    if (json == null) {
        return json;
    }
    return {
        
        'keys': json['keys'],
    };
}

export function RemoveKeyBatchToJSON(value?: RemoveKeyBatch | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'keys': value['keys'],
    };
}

