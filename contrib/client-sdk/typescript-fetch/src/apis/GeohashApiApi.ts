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


import * as runtime from '../runtime';
import type {
  DecodeGeohashResponse,
  EncodeLatLngResponse,
  GeohashNeighborsResponse,
} from '../models/index';
import {
    DecodeGeohashResponseFromJSON,
    DecodeGeohashResponseToJSON,
    EncodeLatLngResponseFromJSON,
    EncodeLatLngResponseToJSON,
    GeohashNeighborsResponseFromJSON,
    GeohashNeighborsResponseToJSON,
} from '../models/index';

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
 * 
 */
export class GeohashApiApi extends runtime.BaseAPI {

    /**
     * Decode geohash by path param, returns coordinates with precision estimates.
     * Decode geohash into coordinates.
     */
    async decodeGeohashRaw(requestParameters: DecodeGeohashRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<DecodeGeohashResponse>> {
        if (requestParameters['ghash'] == null) {
            throw new runtime.RequiredError(
                'ghash',
                'Required parameter "ghash" was null or undefined when calling decodeGeohash().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/geohash/{ghash}`.replace(`{${"ghash"}}`, encodeURIComponent(String(requestParameters['ghash']))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => DecodeGeohashResponseFromJSON(jsonValue));
    }

    /**
     * Decode geohash by path param, returns coordinates with precision estimates.
     * Decode geohash into coordinates.
     */
    async decodeGeohash(requestParameters: DecodeGeohashRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<DecodeGeohashResponse> {
        const response = await this.decodeGeohashRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Encode coordinates by query params, returns geohash.
     * Encode coordinates into geohash
     */
    async encodeLatlngRaw(requestParameters: EncodeLatlngRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<EncodeLatLngResponse>> {
        if (requestParameters['lat'] == null) {
            throw new runtime.RequiredError(
                'lat',
                'Required parameter "lat" was null or undefined when calling encodeLatlng().'
            );
        }

        if (requestParameters['lng'] == null) {
            throw new runtime.RequiredError(
                'lng',
                'Required parameter "lng" was null or undefined when calling encodeLatlng().'
            );
        }

        if (requestParameters['depth'] == null) {
            throw new runtime.RequiredError(
                'depth',
                'Required parameter "depth" was null or undefined when calling encodeLatlng().'
            );
        }

        const queryParameters: any = {};

        if (requestParameters['lat'] != null) {
            queryParameters['lat'] = requestParameters['lat'];
        }

        if (requestParameters['lng'] != null) {
            queryParameters['lng'] = requestParameters['lng'];
        }

        if (requestParameters['depth'] != null) {
            queryParameters['depth'] = requestParameters['depth'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/geohash`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => EncodeLatLngResponseFromJSON(jsonValue));
    }

    /**
     * Encode coordinates by query params, returns geohash.
     * Encode coordinates into geohash
     */
    async encodeLatlng(requestParameters: EncodeLatlngRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<EncodeLatLngResponse> {
        const response = await this.encodeLatlngRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Returns geohash neighbors in all cardinal directions.
     * Neighboring regions
     */
    async getNeighborsRaw(requestParameters: GetNeighborsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<GeohashNeighborsResponse>> {
        if (requestParameters['ghash'] == null) {
            throw new runtime.RequiredError(
                'ghash',
                'Required parameter "ghash" was null or undefined when calling getNeighbors().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/geohash/{ghash}/neighbors`.replace(`{${"ghash"}}`, encodeURIComponent(String(requestParameters['ghash']))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => GeohashNeighborsResponseFromJSON(jsonValue));
    }

    /**
     * Returns geohash neighbors in all cardinal directions.
     * Neighboring regions
     */
    async getNeighbors(requestParameters: GetNeighborsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<GeohashNeighborsResponse> {
        const response = await this.getNeighborsRaw(requestParameters, initOverrides);
        return await response.value();
    }

}