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
import type { Neighbor } from './Neighbor';
import {
    NeighborFromJSON,
    NeighborFromJSONTyped,
    NeighborToJSON,
} from './Neighbor';

/**
 * Returns object keys found with their distance
 * @export
 * @interface QueryRangeResponse
 */
export interface QueryRangeResponse {
    /**
     * Object keys found within range
     * @type {Array<Neighbor>}
     * @memberof QueryRangeResponse
     */
    found: Array<Neighbor>;
}

/**
 * Check if a given object implements the QueryRangeResponse interface.
 */
export function instanceOfQueryRangeResponse(value: object): value is QueryRangeResponse {
    if (!('found' in value) || value['found'] === undefined) return false;
    return true;
}

export function QueryRangeResponseFromJSON(json: any): QueryRangeResponse {
    return QueryRangeResponseFromJSONTyped(json, false);
}

export function QueryRangeResponseFromJSONTyped(json: any, ignoreDiscriminator: boolean): QueryRangeResponse {
    if (json == null) {
        return json;
    }
    return {
        
        'found': ((json['found'] as Array<any>).map(NeighborFromJSON)),
    };
}

export function QueryRangeResponseToJSON(value?: QueryRangeResponse | null): any {
    if (value == null) {
        return value;
    }
    return {
        
        'found': ((value['found'] as Array<any>).map(NeighborToJSON)),
    };
}

