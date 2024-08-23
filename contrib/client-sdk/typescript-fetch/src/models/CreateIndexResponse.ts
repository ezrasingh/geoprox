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
 * Returns index creation status
 * @export
 * @interface CreateIndexResponse
 */
export interface CreateIndexResponse {
    /**
     * If true index was created
     * @type {boolean}
     * @memberof CreateIndexResponse
     */
    created: boolean;
    /**
     * If true index alredy exist
     * @type {boolean}
     * @memberof CreateIndexResponse
     */
    existed: boolean;
}

/**
 * Check if a given object implements the CreateIndexResponse interface.
 */
export function instanceOfCreateIndexResponse(value: object): value is CreateIndexResponse {
    if (!('created' in value) || value['created'] === undefined) return false;
    if (!('existed' in value) || value['existed'] === undefined) return false;
    return true;
}

export function CreateIndexResponseFromJSON(json: any): CreateIndexResponse {
    return CreateIndexResponseFromJSONTyped(json, false);
}

export function CreateIndexResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): CreateIndexResponse {
    if (json == null) {
        return json;
    }
    return {
        
        'created': json['created'],
        'existed': json['existed'],
    };
}

export function CreateIndexResponseToJSON(value?: CreateIndexResponse | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'created': value['created'],
        'existed': value['existed'],
    };
}

