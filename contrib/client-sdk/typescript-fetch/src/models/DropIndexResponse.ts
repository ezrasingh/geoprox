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
 * Returns index deletion status
 * @export
 * @interface DropIndexResponse
 */
export interface DropIndexResponse {
    /**
     * If true index was deleted
     * @type {boolean}
     * @memberof DropIndexResponse
     */
    deleted: boolean;
}

/**
 * Check if a given object implements the DropIndexResponse interface.
 */
export function instanceOfDropIndexResponse(value: object): value is DropIndexResponse {
    if (!('deleted' in value) || value['deleted'] === undefined) return false;
    return true;
}

export function DropIndexResponseFromJSON(json: any): DropIndexResponse {
    return DropIndexResponseFromJSONTyped(json, false);
}

export function DropIndexResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): DropIndexResponse {
    if (json == null) {
        return json;
    }
    return {
        
        'deleted': json['deleted'],
    };
}

export function DropIndexResponseToJSON(value?: DropIndexResponse | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'deleted': value['deleted'],
    };
}

