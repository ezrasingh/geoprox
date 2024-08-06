/* tslint:disable */
/* eslint-disable */
/**
 * geoprox-server
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.3.1
 * Contact: singhezra@gmail.com
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import type {
  CreateIndexResponse,
  DropIndexResponse,
  InsertKey,
  InsertKeyResponse,
  QueryRangeResponse,
  RemoveKey,
} from '../models/index';
import {
    CreateIndexResponseFromJSON,
    CreateIndexResponseToJSON,
    DropIndexResponseFromJSON,
    DropIndexResponseToJSON,
    InsertKeyFromJSON,
    InsertKeyToJSON,
    InsertKeyResponseFromJSON,
    InsertKeyResponseToJSON,
    QueryRangeResponseFromJSON,
    QueryRangeResponseToJSON,
    RemoveKeyFromJSON,
    RemoveKeyToJSON,
} from '../models/index';

export interface CreateIndexRequest {
    index: string;
}

export interface DropIndexRequest {
    index: string;
}

export interface InsertKeyRequest {
    index: string;
    insertKey: InsertKey;
}

export interface QueryRangeRequest {
    lat: number;
    lng: number;
    range: number;
    index: string;
    count?: number | null;
    sorted?: boolean | null;
}

export interface RemoveKeyRequest {
    index: string;
    removeKey: RemoveKey;
}

/**
 * 
 */
export class GeoshardApiApi extends runtime.BaseAPI {

    /**
     * Creates an in-memory index within this geoshard
     * Create geospatial index
     */
    async createIndexRaw(requestParameters: CreateIndexRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<CreateIndexResponse>> {
        if (requestParameters['index'] == null) {
            throw new runtime.RequiredError(
                'index',
                'Required parameter "index" was null or undefined when calling createIndex().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/shard/{index}`.replace(`{${"index"}}`, encodeURIComponent(String(requestParameters['index']))),
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => CreateIndexResponseFromJSON(jsonValue));
    }

    /**
     * Creates an in-memory index within this geoshard
     * Create geospatial index
     */
    async createIndex(requestParameters: CreateIndexRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<CreateIndexResponse> {
        const response = await this.createIndexRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Deletes geospatial index, all keys will be lost
     * Drop index
     */
    async dropIndexRaw(requestParameters: DropIndexRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<DropIndexResponse>> {
        if (requestParameters['index'] == null) {
            throw new runtime.RequiredError(
                'index',
                'Required parameter "index" was null or undefined when calling dropIndex().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/shard/{index}`.replace(`{${"index"}}`, encodeURIComponent(String(requestParameters['index']))),
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => DropIndexResponseFromJSON(jsonValue));
    }

    /**
     * Deletes geospatial index, all keys will be lost
     * Drop index
     */
    async dropIndex(requestParameters: DropIndexRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<DropIndexResponse> {
        const response = await this.dropIndexRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Inserts key into geospatial index
     * Insert key into index
     */
    async insertKeyRaw(requestParameters: InsertKeyRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<InsertKeyResponse>> {
        if (requestParameters['index'] == null) {
            throw new runtime.RequiredError(
                'index',
                'Required parameter "index" was null or undefined when calling insertKey().'
            );
        }

        if (requestParameters['insertKey'] == null) {
            throw new runtime.RequiredError(
                'insertKey',
                'Required parameter "insertKey" was null or undefined when calling insertKey().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/shard/{index}`.replace(`{${"index"}}`, encodeURIComponent(String(requestParameters['index']))),
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: InsertKeyToJSON(requestParameters['insertKey']),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => InsertKeyResponseFromJSON(jsonValue));
    }

    /**
     * Inserts key into geospatial index
     * Insert key into index
     */
    async insertKey(requestParameters: InsertKeyRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<InsertKeyResponse> {
        const response = await this.insertKeyRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Search geospatial index for all keys within some distance
     * Search nearby
     */
    async queryRangeRaw(requestParameters: QueryRangeRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<QueryRangeResponse>> {
        if (requestParameters['lat'] == null) {
            throw new runtime.RequiredError(
                'lat',
                'Required parameter "lat" was null or undefined when calling queryRange().'
            );
        }

        if (requestParameters['lng'] == null) {
            throw new runtime.RequiredError(
                'lng',
                'Required parameter "lng" was null or undefined when calling queryRange().'
            );
        }

        if (requestParameters['range'] == null) {
            throw new runtime.RequiredError(
                'range',
                'Required parameter "range" was null or undefined when calling queryRange().'
            );
        }

        if (requestParameters['index'] == null) {
            throw new runtime.RequiredError(
                'index',
                'Required parameter "index" was null or undefined when calling queryRange().'
            );
        }

        const queryParameters: any = {};

        if (requestParameters['lat'] != null) {
            queryParameters['lat'] = requestParameters['lat'];
        }

        if (requestParameters['lng'] != null) {
            queryParameters['lng'] = requestParameters['lng'];
        }

        if (requestParameters['range'] != null) {
            queryParameters['range'] = requestParameters['range'];
        }

        if (requestParameters['count'] != null) {
            queryParameters['count'] = requestParameters['count'];
        }

        if (requestParameters['sorted'] != null) {
            queryParameters['sorted'] = requestParameters['sorted'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/shard/{index}`.replace(`{${"index"}}`, encodeURIComponent(String(requestParameters['index']))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => QueryRangeResponseFromJSON(jsonValue));
    }

    /**
     * Search geospatial index for all keys within some distance
     * Search nearby
     */
    async queryRange(requestParameters: QueryRangeRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<QueryRangeResponse> {
        const response = await this.queryRangeRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Removed key from geospatial index
     * Remove key from index
     */
    async removeKeyRaw(requestParameters: RemoveKeyRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<InsertKeyResponse>> {
        if (requestParameters['index'] == null) {
            throw new runtime.RequiredError(
                'index',
                'Required parameter "index" was null or undefined when calling removeKey().'
            );
        }

        if (requestParameters['removeKey'] == null) {
            throw new runtime.RequiredError(
                'removeKey',
                'Required parameter "removeKey" was null or undefined when calling removeKey().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/shard/{index}`.replace(`{${"index"}}`, encodeURIComponent(String(requestParameters['index']))),
            method: 'PATCH',
            headers: headerParameters,
            query: queryParameters,
            body: RemoveKeyToJSON(requestParameters['removeKey']),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => InsertKeyResponseFromJSON(jsonValue));
    }

    /**
     * Removed key from geospatial index
     * Remove key from index
     */
    async removeKey(requestParameters: RemoveKeyRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<InsertKeyResponse> {
        const response = await this.removeKeyRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
