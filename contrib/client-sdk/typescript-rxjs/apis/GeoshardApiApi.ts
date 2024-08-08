// tslint:disable
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

import type { Observable } from 'rxjs';
import type { AjaxResponse } from 'rxjs/ajax';
import { BaseAPI, throwIfNullOrUndefined, encodeURI } from '../runtime';
import type { OperationOpts, HttpHeaders, HttpQuery } from '../runtime';
import type {
    CreateIndexResponse,
    DropIndexResponse,
    InsertKey,
    InsertKeyBatch,
    InsertKeyBatchResponse,
    InsertKeyResponse,
    QueryRangeManyResponse,
    QueryRangeResponse,
    RemoveKey,
    RemoveKeyBatch,
    RemoveKeyBatchResponse,
    RemoveKeyResponse,
} from '../models';

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

export interface InsertKeyBatchRequest {
    index: string;
    insertKeyBatch: InsertKeyBatch;
}

export interface QueryRangeRequest {
    index: string;
    lat: number;
    lng: number;
    range: number;
    count?: number | null;
    sorted?: boolean | null;
}

export interface QueryRangeManyRequest {
    indices: Array<string>;
    lat: number;
    lng: number;
    range: number;
    count?: number | null;
    sorted?: boolean | null;
}

export interface RemoveKeyRequest {
    index: string;
    removeKey: RemoveKey;
}

export interface RemoveKeyBatchRequest {
    index: string;
    removeKeyBatch: RemoveKeyBatch;
}

/**
 * no description
 */
export class GeoshardApiApi extends BaseAPI {

    /**
     * Creates an in-memory index within this geoshard
     * Create geospatial index
     */
    createIndex({ index }: CreateIndexRequest): Observable<CreateIndexResponse>
    createIndex({ index }: CreateIndexRequest, opts?: OperationOpts): Observable<AjaxResponse<CreateIndexResponse>>
    createIndex({ index }: CreateIndexRequest, opts?: OperationOpts): Observable<CreateIndexResponse | AjaxResponse<CreateIndexResponse>> {
        throwIfNullOrUndefined(index, 'index', 'createIndex');

        return this.request<CreateIndexResponse>({
            url: '/api/v1/shard/{index}/'.replace('{index}', encodeURI(index)),
            method: 'POST',
        }, opts?.responseOpts);
    };

    /**
     * Drop index. All keys will be lost
     * Deletes geospatial index
     */
    dropIndex({ index }: DropIndexRequest): Observable<DropIndexResponse>
    dropIndex({ index }: DropIndexRequest, opts?: OperationOpts): Observable<AjaxResponse<DropIndexResponse>>
    dropIndex({ index }: DropIndexRequest, opts?: OperationOpts): Observable<DropIndexResponse | AjaxResponse<DropIndexResponse>> {
        throwIfNullOrUndefined(index, 'index', 'dropIndex');

        return this.request<DropIndexResponse>({
            url: '/api/v1/shard/{index}/'.replace('{index}', encodeURI(index)),
            method: 'DELETE',
        }, opts?.responseOpts);
    };

    /**
     * Inserts key into geospatial index
     * Insert key into index
     */
    insertKey({ index, insertKey }: InsertKeyRequest): Observable<InsertKeyResponse>
    insertKey({ index, insertKey }: InsertKeyRequest, opts?: OperationOpts): Observable<AjaxResponse<InsertKeyResponse>>
    insertKey({ index, insertKey }: InsertKeyRequest, opts?: OperationOpts): Observable<InsertKeyResponse | AjaxResponse<InsertKeyResponse>> {
        throwIfNullOrUndefined(index, 'index', 'insertKey');
        throwIfNullOrUndefined(insertKey, 'insertKey', 'insertKey');

        const headers: HttpHeaders = {
            'Content-Type': 'application/json',
        };

        return this.request<InsertKeyResponse>({
            url: '/api/v1/shard/{index}/'.replace('{index}', encodeURI(index)),
            method: 'PUT',
            headers,
            body: insertKey,
        }, opts?.responseOpts);
    };

    /**
     * Inserts multiple keys into geospatial index
     * Insert multiple keys into index
     */
    insertKeyBatch({ index, insertKeyBatch }: InsertKeyBatchRequest): Observable<InsertKeyBatchResponse>
    insertKeyBatch({ index, insertKeyBatch }: InsertKeyBatchRequest, opts?: OperationOpts): Observable<AjaxResponse<InsertKeyBatchResponse>>
    insertKeyBatch({ index, insertKeyBatch }: InsertKeyBatchRequest, opts?: OperationOpts): Observable<InsertKeyBatchResponse | AjaxResponse<InsertKeyBatchResponse>> {
        throwIfNullOrUndefined(index, 'index', 'insertKeyBatch');
        throwIfNullOrUndefined(insertKeyBatch, 'insertKeyBatch', 'insertKeyBatch');

        const headers: HttpHeaders = {
            'Content-Type': 'application/json',
        };

        return this.request<InsertKeyBatchResponse>({
            url: '/api/v1/shard/{index}/batch/'.replace('{index}', encodeURI(index)),
            method: 'PUT',
            headers,
            body: insertKeyBatch,
        }, opts?.responseOpts);
    };

    /**
     * Search geospatial index for all keys within some distance
     * Search index for objects nearby
     */
    queryRange({ index, lat, lng, range, count, sorted }: QueryRangeRequest): Observable<QueryRangeResponse>
    queryRange({ index, lat, lng, range, count, sorted }: QueryRangeRequest, opts?: OperationOpts): Observable<AjaxResponse<QueryRangeResponse>>
    queryRange({ index, lat, lng, range, count, sorted }: QueryRangeRequest, opts?: OperationOpts): Observable<QueryRangeResponse | AjaxResponse<QueryRangeResponse>> {
        throwIfNullOrUndefined(index, 'index', 'queryRange');
        throwIfNullOrUndefined(lat, 'lat', 'queryRange');
        throwIfNullOrUndefined(lng, 'lng', 'queryRange');
        throwIfNullOrUndefined(range, 'range', 'queryRange');

        const query: HttpQuery = { // required parameters are used directly since they are already checked by throwIfNullOrUndefined
            'lat': lat,
            'lng': lng,
            'range': range,
        };

        if (count != null) { query['count'] = count; }
        if (sorted != null) { query['sorted'] = sorted; }

        return this.request<QueryRangeResponse>({
            url: '/api/v1/shard/{index}/'.replace('{index}', encodeURI(index)),
            method: 'GET',
            query,
        }, opts?.responseOpts);
    };

    /**
     * Search geospatial many indices for all keys within some distance
     * Search multiple indices for objects nearby
     */
    queryRangeMany({ indices, lat, lng, range, count, sorted }: QueryRangeManyRequest): Observable<QueryRangeManyResponse>
    queryRangeMany({ indices, lat, lng, range, count, sorted }: QueryRangeManyRequest, opts?: OperationOpts): Observable<AjaxResponse<QueryRangeManyResponse>>
    queryRangeMany({ indices, lat, lng, range, count, sorted }: QueryRangeManyRequest, opts?: OperationOpts): Observable<QueryRangeManyResponse | AjaxResponse<QueryRangeManyResponse>> {
        throwIfNullOrUndefined(indices, 'indices', 'queryRangeMany');
        throwIfNullOrUndefined(lat, 'lat', 'queryRangeMany');
        throwIfNullOrUndefined(lng, 'lng', 'queryRangeMany');
        throwIfNullOrUndefined(range, 'range', 'queryRangeMany');

        const query: HttpQuery = { // required parameters are used directly since they are already checked by throwIfNullOrUndefined
            'indices': indices,
            'lat': lat,
            'lng': lng,
            'range': range,
        };

        if (count != null) { query['count'] = count; }
        if (sorted != null) { query['sorted'] = sorted; }

        return this.request<QueryRangeManyResponse>({
            url: '/api/v1/shard/',
            method: 'GET',
            query,
        }, opts?.responseOpts);
    };

    /**
     * Removes key from geospatial index
     * Remove key from index
     */
    removeKey({ index, removeKey }: RemoveKeyRequest): Observable<RemoveKeyResponse>
    removeKey({ index, removeKey }: RemoveKeyRequest, opts?: OperationOpts): Observable<AjaxResponse<RemoveKeyResponse>>
    removeKey({ index, removeKey }: RemoveKeyRequest, opts?: OperationOpts): Observable<RemoveKeyResponse | AjaxResponse<RemoveKeyResponse>> {
        throwIfNullOrUndefined(index, 'index', 'removeKey');
        throwIfNullOrUndefined(removeKey, 'removeKey', 'removeKey');

        const headers: HttpHeaders = {
            'Content-Type': 'application/json',
        };

        return this.request<RemoveKeyResponse>({
            url: '/api/v1/shard/{index}/'.replace('{index}', encodeURI(index)),
            method: 'PATCH',
            headers,
            body: removeKey,
        }, opts?.responseOpts);
    };

    /**
     * Removes multiple keys from geospatial index
     * Remove multiple keys from index
     */
    removeKeyBatch({ index, removeKeyBatch }: RemoveKeyBatchRequest): Observable<RemoveKeyBatchResponse>
    removeKeyBatch({ index, removeKeyBatch }: RemoveKeyBatchRequest, opts?: OperationOpts): Observable<AjaxResponse<RemoveKeyBatchResponse>>
    removeKeyBatch({ index, removeKeyBatch }: RemoveKeyBatchRequest, opts?: OperationOpts): Observable<RemoveKeyBatchResponse | AjaxResponse<RemoveKeyBatchResponse>> {
        throwIfNullOrUndefined(index, 'index', 'removeKeyBatch');
        throwIfNullOrUndefined(removeKeyBatch, 'removeKeyBatch', 'removeKeyBatch');

        const headers: HttpHeaders = {
            'Content-Type': 'application/json',
        };

        return this.request<RemoveKeyBatchResponse>({
            url: '/api/v1/shard/{index}/batch/'.replace('{index}', encodeURI(index)),
            method: 'PATCH',
            headers,
            body: removeKeyBatch,
        }, opts?.responseOpts);
    };

}
