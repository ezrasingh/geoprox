/*
geoprox-server

Geoprox server implementation providing a HTTP API for geospatial queries and position tracking

API version: 0.4.0
Contact: singhezra@gmail.com
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package geoprox_client

import (
	"bytes"
	"context"
	"io"
	"net/http"
	"net/url"
	"strings"
)


// GeohashApiAPIService GeohashApiAPI service
type GeohashApiAPIService service

type ApiDecodeGeohashRequest struct {
	ctx context.Context
	ApiService *GeohashApiAPIService
	ghash string
}

func (r ApiDecodeGeohashRequest) Execute() (*DecodeGeohashResponse, *http.Response, error) {
	return r.ApiService.DecodeGeohashExecute(r)
}

/*
DecodeGeohash Decode geohash into coordinates.

Decode geohash by path param, returns coordinates with precision estimates.

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @param ghash Geohash encoded region
 @return ApiDecodeGeohashRequest
*/
func (a *GeohashApiAPIService) DecodeGeohash(ctx context.Context, ghash string) ApiDecodeGeohashRequest {
	return ApiDecodeGeohashRequest{
		ApiService: a,
		ctx: ctx,
		ghash: ghash,
	}
}

// Execute executes the request
//  @return DecodeGeohashResponse
func (a *GeohashApiAPIService) DecodeGeohashExecute(r ApiDecodeGeohashRequest) (*DecodeGeohashResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodGet
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *DecodeGeohashResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeohashApiAPIService.DecodeGeohash")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/geohash/{ghash}/"
	localVarPath = strings.Replace(localVarPath, "{"+"ghash"+"}", url.PathEscape(parameterValueToString(r.ghash, "ghash")), -1)

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}

	// to determine the Content-Type header
	localVarHTTPContentTypes := []string{}

	// set Content-Type header
	localVarHTTPContentType := selectHeaderContentType(localVarHTTPContentTypes)
	if localVarHTTPContentType != "" {
		localVarHeaderParams["Content-Type"] = localVarHTTPContentType
	}

	// to determine the Accept header
	localVarHTTPHeaderAccepts := []string{"application/json"}

	// set Accept header
	localVarHTTPHeaderAccept := selectHeaderAccept(localVarHTTPHeaderAccepts)
	if localVarHTTPHeaderAccept != "" {
		localVarHeaderParams["Accept"] = localVarHTTPHeaderAccept
	}
	req, err := a.client.prepareRequest(r.ctx, localVarPath, localVarHTTPMethod, localVarPostBody, localVarHeaderParams, localVarQueryParams, localVarFormParams, formFiles)
	if err != nil {
		return localVarReturnValue, nil, err
	}

	localVarHTTPResponse, err := a.client.callAPI(req)
	if err != nil || localVarHTTPResponse == nil {
		return localVarReturnValue, localVarHTTPResponse, err
	}

	localVarBody, err := io.ReadAll(localVarHTTPResponse.Body)
	localVarHTTPResponse.Body.Close()
	localVarHTTPResponse.Body = io.NopCloser(bytes.NewBuffer(localVarBody))
	if err != nil {
		return localVarReturnValue, localVarHTTPResponse, err
	}

	if localVarHTTPResponse.StatusCode >= 300 {
		newErr := &GenericOpenAPIError{
			body:  localVarBody,
			error: localVarHTTPResponse.Status,
		}
		return localVarReturnValue, localVarHTTPResponse, newErr
	}

	err = a.client.decode(&localVarReturnValue, localVarBody, localVarHTTPResponse.Header.Get("Content-Type"))
	if err != nil {
		newErr := &GenericOpenAPIError{
			body:  localVarBody,
			error: err.Error(),
		}
		return localVarReturnValue, localVarHTTPResponse, newErr
	}

	return localVarReturnValue, localVarHTTPResponse, nil
}

type ApiEncodeLatlngRequest struct {
	ctx context.Context
	ApiService *GeohashApiAPIService
	lat *float64
	lng *float64
	depth *int32
}

// Latitude
func (r ApiEncodeLatlngRequest) Lat(lat float64) ApiEncodeLatlngRequest {
	r.lat = &lat
	return r
}

// Longitude
func (r ApiEncodeLatlngRequest) Lng(lng float64) ApiEncodeLatlngRequest {
	r.lng = &lng
	return r
}

// Determines geohash length
func (r ApiEncodeLatlngRequest) Depth(depth int32) ApiEncodeLatlngRequest {
	r.depth = &depth
	return r
}

func (r ApiEncodeLatlngRequest) Execute() (*EncodeLatLngResponse, *http.Response, error) {
	return r.ApiService.EncodeLatlngExecute(r)
}

/*
EncodeLatlng Encode coordinates into geohash

Encode coordinates by query params, returns geohash.

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @return ApiEncodeLatlngRequest
*/
func (a *GeohashApiAPIService) EncodeLatlng(ctx context.Context) ApiEncodeLatlngRequest {
	return ApiEncodeLatlngRequest{
		ApiService: a,
		ctx: ctx,
	}
}

// Execute executes the request
//  @return EncodeLatLngResponse
func (a *GeohashApiAPIService) EncodeLatlngExecute(r ApiEncodeLatlngRequest) (*EncodeLatLngResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodGet
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *EncodeLatLngResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeohashApiAPIService.EncodeLatlng")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/geohash/"

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}
	if r.lat == nil {
		return localVarReturnValue, nil, reportError("lat is required and must be specified")
	}
	if r.lng == nil {
		return localVarReturnValue, nil, reportError("lng is required and must be specified")
	}
	if r.depth == nil {
		return localVarReturnValue, nil, reportError("depth is required and must be specified")
	}
	if *r.depth < 0 {
		return localVarReturnValue, nil, reportError("depth must be greater than 0")
	}

	parameterAddToHeaderOrQuery(localVarQueryParams, "lat", r.lat, "")
	parameterAddToHeaderOrQuery(localVarQueryParams, "lng", r.lng, "")
	parameterAddToHeaderOrQuery(localVarQueryParams, "depth", r.depth, "")
	// to determine the Content-Type header
	localVarHTTPContentTypes := []string{}

	// set Content-Type header
	localVarHTTPContentType := selectHeaderContentType(localVarHTTPContentTypes)
	if localVarHTTPContentType != "" {
		localVarHeaderParams["Content-Type"] = localVarHTTPContentType
	}

	// to determine the Accept header
	localVarHTTPHeaderAccepts := []string{"application/json"}

	// set Accept header
	localVarHTTPHeaderAccept := selectHeaderAccept(localVarHTTPHeaderAccepts)
	if localVarHTTPHeaderAccept != "" {
		localVarHeaderParams["Accept"] = localVarHTTPHeaderAccept
	}
	req, err := a.client.prepareRequest(r.ctx, localVarPath, localVarHTTPMethod, localVarPostBody, localVarHeaderParams, localVarQueryParams, localVarFormParams, formFiles)
	if err != nil {
		return localVarReturnValue, nil, err
	}

	localVarHTTPResponse, err := a.client.callAPI(req)
	if err != nil || localVarHTTPResponse == nil {
		return localVarReturnValue, localVarHTTPResponse, err
	}

	localVarBody, err := io.ReadAll(localVarHTTPResponse.Body)
	localVarHTTPResponse.Body.Close()
	localVarHTTPResponse.Body = io.NopCloser(bytes.NewBuffer(localVarBody))
	if err != nil {
		return localVarReturnValue, localVarHTTPResponse, err
	}

	if localVarHTTPResponse.StatusCode >= 300 {
		newErr := &GenericOpenAPIError{
			body:  localVarBody,
			error: localVarHTTPResponse.Status,
		}
		return localVarReturnValue, localVarHTTPResponse, newErr
	}

	err = a.client.decode(&localVarReturnValue, localVarBody, localVarHTTPResponse.Header.Get("Content-Type"))
	if err != nil {
		newErr := &GenericOpenAPIError{
			body:  localVarBody,
			error: err.Error(),
		}
		return localVarReturnValue, localVarHTTPResponse, newErr
	}

	return localVarReturnValue, localVarHTTPResponse, nil
}

type ApiGetNeighborsRequest struct {
	ctx context.Context
	ApiService *GeohashApiAPIService
	ghash string
}

func (r ApiGetNeighborsRequest) Execute() (*GeohashNeighborsResponse, *http.Response, error) {
	return r.ApiService.GetNeighborsExecute(r)
}

/*
GetNeighbors Neighboring regions

Returns geohash neighbors in all cardinal directions.

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @param ghash Geohash encoded region
 @return ApiGetNeighborsRequest
*/
func (a *GeohashApiAPIService) GetNeighbors(ctx context.Context, ghash string) ApiGetNeighborsRequest {
	return ApiGetNeighborsRequest{
		ApiService: a,
		ctx: ctx,
		ghash: ghash,
	}
}

// Execute executes the request
//  @return GeohashNeighborsResponse
func (a *GeohashApiAPIService) GetNeighborsExecute(r ApiGetNeighborsRequest) (*GeohashNeighborsResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodGet
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *GeohashNeighborsResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeohashApiAPIService.GetNeighbors")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/geohash/{ghash}/neighbors/"
	localVarPath = strings.Replace(localVarPath, "{"+"ghash"+"}", url.PathEscape(parameterValueToString(r.ghash, "ghash")), -1)

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}

	// to determine the Content-Type header
	localVarHTTPContentTypes := []string{}

	// set Content-Type header
	localVarHTTPContentType := selectHeaderContentType(localVarHTTPContentTypes)
	if localVarHTTPContentType != "" {
		localVarHeaderParams["Content-Type"] = localVarHTTPContentType
	}

	// to determine the Accept header
	localVarHTTPHeaderAccepts := []string{"application/json"}

	// set Accept header
	localVarHTTPHeaderAccept := selectHeaderAccept(localVarHTTPHeaderAccepts)
	if localVarHTTPHeaderAccept != "" {
		localVarHeaderParams["Accept"] = localVarHTTPHeaderAccept
	}
	req, err := a.client.prepareRequest(r.ctx, localVarPath, localVarHTTPMethod, localVarPostBody, localVarHeaderParams, localVarQueryParams, localVarFormParams, formFiles)
	if err != nil {
		return localVarReturnValue, nil, err
	}

	localVarHTTPResponse, err := a.client.callAPI(req)
	if err != nil || localVarHTTPResponse == nil {
		return localVarReturnValue, localVarHTTPResponse, err
	}

	localVarBody, err := io.ReadAll(localVarHTTPResponse.Body)
	localVarHTTPResponse.Body.Close()
	localVarHTTPResponse.Body = io.NopCloser(bytes.NewBuffer(localVarBody))
	if err != nil {
		return localVarReturnValue, localVarHTTPResponse, err
	}

	if localVarHTTPResponse.StatusCode >= 300 {
		newErr := &GenericOpenAPIError{
			body:  localVarBody,
			error: localVarHTTPResponse.Status,
		}
		return localVarReturnValue, localVarHTTPResponse, newErr
	}

	err = a.client.decode(&localVarReturnValue, localVarBody, localVarHTTPResponse.Header.Get("Content-Type"))
	if err != nil {
		newErr := &GenericOpenAPIError{
			body:  localVarBody,
			error: err.Error(),
		}
		return localVarReturnValue, localVarHTTPResponse, newErr
	}

	return localVarReturnValue, localVarHTTPResponse, nil
}
