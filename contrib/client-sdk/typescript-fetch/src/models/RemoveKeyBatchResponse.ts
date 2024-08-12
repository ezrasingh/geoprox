/* tslint:disable */
/* eslint-disable */
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

import { mapValues } from '../runtime';
/**
 * Returns batch key deletion status
 * @export
 * @interface RemoveKeyBatchResponse
 */
export interface RemoveKeyBatchResponse {
    /**
     * If true all keys were removed
     * @type {boolean}
     * @memberof RemoveKeyBatchResponse
     */
    deleted: boolean;
}

/**
 * Check if a given object implements the RemoveKeyBatchResponse interface.
 */
export function instanceOfRemoveKeyBatchResponse(value: object): value is RemoveKeyBatchResponse {
    if (!('deleted' in value) || value['deleted'] === undefined) return false;
    return true;
}

export function RemoveKeyBatchResponseFromJSON(json: any): RemoveKeyBatchResponse {
    return RemoveKeyBatchResponseFromJSONTyped(json, false);
}

export function RemoveKeyBatchResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): RemoveKeyBatchResponse {
    if (json == null) {
        return json;
    }
    return {
        
        'deleted': json['deleted'],
    };
}

export function RemoveKeyBatchResponseToJSON(value?: RemoveKeyBatchResponse | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'deleted': value['deleted'],
    };
}
