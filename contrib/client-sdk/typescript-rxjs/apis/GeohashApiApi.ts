// tslint:disable
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

import type { Observable } from 'rxjs';
import type { AjaxResponse } from 'rxjs/ajax';
import { BaseAPI, throwIfNullOrUndefined, encodeURI } from '../runtime';
import type { OperationOpts, HttpQuery } from '../runtime';
import type {
    DecodeGeohashResponse,
    EncodeLatLngResponse,
    GeohashNeighborsResponse,
} from '../models';

export interface DecodeGeohashRequest {
    ghash: string;
}

export interface EncodeLatlngRequest {
    lat: number;
    lng: number;
    depth: number;
}

export interface GetNeighborsRequest {
    ghash: string;
}

/**
 * no description
 */
export class GeohashApiApi extends BaseAPI {

    /**
     * Decode geohash by path param, returns coordinates with precision estimates.
     * Decode geohash into coordinates.
     */
    decodeGeohash({ ghash }: DecodeGeohashRequest): Observable<DecodeGeohashResponse>
    decodeGeohash({ ghash }: DecodeGeohashRequest, opts?: OperationOpts): Observable<AjaxResponse<DecodeGeohashResponse>>
    decodeGeohash({ ghash }: DecodeGeohashRequest, opts?: OperationOpts): Observable<DecodeGeohashResponse | AjaxResponse<DecodeGeohashResponse>> {
        throwIfNullOrUndefined(ghash, 'ghash', 'decodeGeohash');

        return this.request<DecodeGeohashResponse>({
            url: '/api/v1/geohash/{ghash}'.replace('{ghash}', encodeURI(ghash)),
            method: 'GET',
        }, opts?.responseOpts);
    };

    /**
     * Encode coordinates by query params, returns geohash.
     * Encode coordinates into geohash
     */
    encodeLatlng({ lat, lng, depth }: EncodeLatlngRequest): Observable<EncodeLatLngResponse>
    encodeLatlng({ lat, lng, depth }: EncodeLatlngRequest, opts?: OperationOpts): Observable<AjaxResponse<EncodeLatLngResponse>>
    encodeLatlng({ lat, lng, depth }: EncodeLatlngRequest, opts?: OperationOpts): Observable<EncodeLatLngResponse | AjaxResponse<EncodeLatLngResponse>> {
        throwIfNullOrUndefined(lat, 'lat', 'encodeLatlng');
        throwIfNullOrUndefined(lng, 'lng', 'encodeLatlng');
        throwIfNullOrUndefined(depth, 'depth', 'encodeLatlng');

        const query: HttpQuery = { // required parameters are used directly since they are already checked by throwIfNullOrUndefined
            'lat': lat,
            'lng': lng,
            'depth': depth,
        };

        return this.request<EncodeLatLngResponse>({
            url: '/api/v1/geohash',
            method: 'GET',
            query,
        }, opts?.responseOpts);
    };

    /**
     * Returns geohash neighbors in all cardinal directions.
     * Neighboring regions
     */
    getNeighbors({ ghash }: GetNeighborsRequest): Observable<GeohashNeighborsResponse>
    getNeighbors({ ghash }: GetNeighborsRequest, opts?: OperationOpts): Observable<AjaxResponse<GeohashNeighborsResponse>>
    getNeighbors({ ghash }: GetNeighborsRequest, opts?: OperationOpts): Observable<GeohashNeighborsResponse | AjaxResponse<GeohashNeighborsResponse>> {
        throwIfNullOrUndefined(ghash, 'ghash', 'getNeighbors');

        return this.request<GeohashNeighborsResponse>({
            url: '/api/v1/geohash/{ghash}/neighbors'.replace('{ghash}', encodeURI(ghash)),
            method: 'GET',
        }, opts?.responseOpts);
    };

}
