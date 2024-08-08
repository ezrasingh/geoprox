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
	"reflect"
)


// GeoshardApiAPIService GeoshardApiAPI service
type GeoshardApiAPIService service

type ApiCreateIndexRequest struct {
	ctx context.Context
	ApiService *GeoshardApiAPIService
	index string
}

func (r ApiCreateIndexRequest) Execute() (*CreateIndexResponse, *http.Response, error) {
	return r.ApiService.CreateIndexExecute(r)
}

/*
CreateIndex Create geospatial index

Creates an in-memory index within this geoshard

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @param index Geospatial index name
 @return ApiCreateIndexRequest
*/
func (a *GeoshardApiAPIService) CreateIndex(ctx context.Context, index string) ApiCreateIndexRequest {
	return ApiCreateIndexRequest{
		ApiService: a,
		ctx: ctx,
		index: index,
	}
}

// Execute executes the request
//  @return CreateIndexResponse
func (a *GeoshardApiAPIService) CreateIndexExecute(r ApiCreateIndexRequest) (*CreateIndexResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodPost
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *CreateIndexResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeoshardApiAPIService.CreateIndex")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/shard/{index}/"
	localVarPath = strings.Replace(localVarPath, "{"+"index"+"}", url.PathEscape(parameterValueToString(r.index, "index")), -1)

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

type ApiDropIndexRequest struct {
	ctx context.Context
	ApiService *GeoshardApiAPIService
	index string
}

func (r ApiDropIndexRequest) Execute() (*DropIndexResponse, *http.Response, error) {
	return r.ApiService.DropIndexExecute(r)
}

/*
DropIndex Deletes geospatial index

Drop index. All keys will be lost

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @param index Geospatial index name
 @return ApiDropIndexRequest
*/
func (a *GeoshardApiAPIService) DropIndex(ctx context.Context, index string) ApiDropIndexRequest {
	return ApiDropIndexRequest{
		ApiService: a,
		ctx: ctx,
		index: index,
	}
}

// Execute executes the request
//  @return DropIndexResponse
func (a *GeoshardApiAPIService) DropIndexExecute(r ApiDropIndexRequest) (*DropIndexResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodDelete
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *DropIndexResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeoshardApiAPIService.DropIndex")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/shard/{index}/"
	localVarPath = strings.Replace(localVarPath, "{"+"index"+"}", url.PathEscape(parameterValueToString(r.index, "index")), -1)

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

type ApiInsertKeyRequest struct {
	ctx context.Context
	ApiService *GeoshardApiAPIService
	index string
	insertKey *InsertKey
}

// 
func (r ApiInsertKeyRequest) InsertKey(insertKey InsertKey) ApiInsertKeyRequest {
	r.insertKey = &insertKey
	return r
}

func (r ApiInsertKeyRequest) Execute() (*InsertKeyResponse, *http.Response, error) {
	return r.ApiService.InsertKeyExecute(r)
}

/*
InsertKey Insert key into index

Inserts key into geospatial index

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @param index Geospatial index name
 @return ApiInsertKeyRequest
*/
func (a *GeoshardApiAPIService) InsertKey(ctx context.Context, index string) ApiInsertKeyRequest {
	return ApiInsertKeyRequest{
		ApiService: a,
		ctx: ctx,
		index: index,
	}
}

// Execute executes the request
//  @return InsertKeyResponse
func (a *GeoshardApiAPIService) InsertKeyExecute(r ApiInsertKeyRequest) (*InsertKeyResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodPut
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *InsertKeyResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeoshardApiAPIService.InsertKey")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/shard/{index}/"
	localVarPath = strings.Replace(localVarPath, "{"+"index"+"}", url.PathEscape(parameterValueToString(r.index, "index")), -1)

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}
	if r.insertKey == nil {
		return localVarReturnValue, nil, reportError("insertKey is required and must be specified")
	}

	// to determine the Content-Type header
	localVarHTTPContentTypes := []string{"application/json"}

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
	// body params
	localVarPostBody = r.insertKey
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

type ApiInsertKeyBatchRequest struct {
	ctx context.Context
	ApiService *GeoshardApiAPIService
	index string
	insertKeyBatch *InsertKeyBatch
}

// 
func (r ApiInsertKeyBatchRequest) InsertKeyBatch(insertKeyBatch InsertKeyBatch) ApiInsertKeyBatchRequest {
	r.insertKeyBatch = &insertKeyBatch
	return r
}

func (r ApiInsertKeyBatchRequest) Execute() (*InsertKeyBatchResponse, *http.Response, error) {
	return r.ApiService.InsertKeyBatchExecute(r)
}

/*
InsertKeyBatch Insert multiple keys into index

Inserts multiple keys into geospatial index

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @param index Geospatial index name
 @return ApiInsertKeyBatchRequest
*/
func (a *GeoshardApiAPIService) InsertKeyBatch(ctx context.Context, index string) ApiInsertKeyBatchRequest {
	return ApiInsertKeyBatchRequest{
		ApiService: a,
		ctx: ctx,
		index: index,
	}
}

// Execute executes the request
//  @return InsertKeyBatchResponse
func (a *GeoshardApiAPIService) InsertKeyBatchExecute(r ApiInsertKeyBatchRequest) (*InsertKeyBatchResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodPut
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *InsertKeyBatchResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeoshardApiAPIService.InsertKeyBatch")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/shard/{index}/batch/"
	localVarPath = strings.Replace(localVarPath, "{"+"index"+"}", url.PathEscape(parameterValueToString(r.index, "index")), -1)

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}
	if r.insertKeyBatch == nil {
		return localVarReturnValue, nil, reportError("insertKeyBatch is required and must be specified")
	}

	// to determine the Content-Type header
	localVarHTTPContentTypes := []string{"application/json"}

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
	// body params
	localVarPostBody = r.insertKeyBatch
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

type ApiQueryRangeRequest struct {
	ctx context.Context
	ApiService *GeoshardApiAPIService
	index string
	lat *float64
	lng *float64
	range_ *int32
	count *int32
	sorted *bool
}

// Latitude
func (r ApiQueryRangeRequest) Lat(lat float64) ApiQueryRangeRequest {
	r.lat = &lat
	return r
}

// Longitude
func (r ApiQueryRangeRequest) Lng(lng float64) ApiQueryRangeRequest {
	r.lng = &lng
	return r
}

// Search radius in kilometers
func (r ApiQueryRangeRequest) Range_(range_ int32) ApiQueryRangeRequest {
	r.range_ = &range_
	return r
}

// Maximum number of neighbors that can be returned (default 100)
func (r ApiQueryRangeRequest) Count(count int32) ApiQueryRangeRequest {
	r.count = &count
	return r
}

// If enabled neighbors will be sorted by distance, nearest to furthest (default false)
func (r ApiQueryRangeRequest) Sorted(sorted bool) ApiQueryRangeRequest {
	r.sorted = &sorted
	return r
}

func (r ApiQueryRangeRequest) Execute() (*QueryRangeResponse, *http.Response, error) {
	return r.ApiService.QueryRangeExecute(r)
}

/*
QueryRange Search index for objects nearby

Search geospatial index for all keys within some distance

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @param index Geospatial index name
 @return ApiQueryRangeRequest
*/
func (a *GeoshardApiAPIService) QueryRange(ctx context.Context, index string) ApiQueryRangeRequest {
	return ApiQueryRangeRequest{
		ApiService: a,
		ctx: ctx,
		index: index,
	}
}

// Execute executes the request
//  @return QueryRangeResponse
func (a *GeoshardApiAPIService) QueryRangeExecute(r ApiQueryRangeRequest) (*QueryRangeResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodGet
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *QueryRangeResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeoshardApiAPIService.QueryRange")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/shard/{index}/"
	localVarPath = strings.Replace(localVarPath, "{"+"index"+"}", url.PathEscape(parameterValueToString(r.index, "index")), -1)

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}
	if r.lat == nil {
		return localVarReturnValue, nil, reportError("lat is required and must be specified")
	}
	if r.lng == nil {
		return localVarReturnValue, nil, reportError("lng is required and must be specified")
	}
	if r.range_ == nil {
		return localVarReturnValue, nil, reportError("range_ is required and must be specified")
	}
	if *r.range_ < 0 {
		return localVarReturnValue, nil, reportError("range_ must be greater than 0")
	}

	parameterAddToHeaderOrQuery(localVarQueryParams, "lat", r.lat, "")
	parameterAddToHeaderOrQuery(localVarQueryParams, "lng", r.lng, "")
	parameterAddToHeaderOrQuery(localVarQueryParams, "range", r.range_, "")
	if r.count != nil {
		parameterAddToHeaderOrQuery(localVarQueryParams, "count", r.count, "")
	}
	if r.sorted != nil {
		parameterAddToHeaderOrQuery(localVarQueryParams, "sorted", r.sorted, "")
	}
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

type ApiQueryRangeManyRequest struct {
	ctx context.Context
	ApiService *GeoshardApiAPIService
	indices *[]string
	lat *float64
	lng *float64
	range_ *int32
	count *int32
	sorted *bool
}

// List of indices to search
func (r ApiQueryRangeManyRequest) Indices(indices []string) ApiQueryRangeManyRequest {
	r.indices = &indices
	return r
}

// Latitude
func (r ApiQueryRangeManyRequest) Lat(lat float64) ApiQueryRangeManyRequest {
	r.lat = &lat
	return r
}

// Longitude
func (r ApiQueryRangeManyRequest) Lng(lng float64) ApiQueryRangeManyRequest {
	r.lng = &lng
	return r
}

// Search radius in kilometers
func (r ApiQueryRangeManyRequest) Range_(range_ int32) ApiQueryRangeManyRequest {
	r.range_ = &range_
	return r
}

// Maximum number of neighbors that can be returned (default 100)
func (r ApiQueryRangeManyRequest) Count(count int32) ApiQueryRangeManyRequest {
	r.count = &count
	return r
}

// If enabled neighbors will be sorted by distance, nearest to furthest (default false)
func (r ApiQueryRangeManyRequest) Sorted(sorted bool) ApiQueryRangeManyRequest {
	r.sorted = &sorted
	return r
}

func (r ApiQueryRangeManyRequest) Execute() (*QueryRangeManyResponse, *http.Response, error) {
	return r.ApiService.QueryRangeManyExecute(r)
}

/*
QueryRangeMany Search multiple indices for objects nearby

Search geospatial many indices for all keys within some distance

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @return ApiQueryRangeManyRequest
*/
func (a *GeoshardApiAPIService) QueryRangeMany(ctx context.Context) ApiQueryRangeManyRequest {
	return ApiQueryRangeManyRequest{
		ApiService: a,
		ctx: ctx,
	}
}

// Execute executes the request
//  @return QueryRangeManyResponse
func (a *GeoshardApiAPIService) QueryRangeManyExecute(r ApiQueryRangeManyRequest) (*QueryRangeManyResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodGet
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *QueryRangeManyResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeoshardApiAPIService.QueryRangeMany")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/shard/"

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}
	if r.indices == nil {
		return localVarReturnValue, nil, reportError("indices is required and must be specified")
	}
	if r.lat == nil {
		return localVarReturnValue, nil, reportError("lat is required and must be specified")
	}
	if r.lng == nil {
		return localVarReturnValue, nil, reportError("lng is required and must be specified")
	}
	if r.range_ == nil {
		return localVarReturnValue, nil, reportError("range_ is required and must be specified")
	}
	if *r.range_ < 0 {
		return localVarReturnValue, nil, reportError("range_ must be greater than 0")
	}

	{
		t := *r.indices
		if reflect.TypeOf(t).Kind() == reflect.Slice {
			s := reflect.ValueOf(t)
			for i := 0; i < s.Len(); i++ {
				parameterAddToHeaderOrQuery(localVarQueryParams, "indices", s.Index(i).Interface(), "multi")
			}
		} else {
			parameterAddToHeaderOrQuery(localVarQueryParams, "indices", t, "multi")
		}
	}
	parameterAddToHeaderOrQuery(localVarQueryParams, "lat", r.lat, "")
	parameterAddToHeaderOrQuery(localVarQueryParams, "lng", r.lng, "")
	parameterAddToHeaderOrQuery(localVarQueryParams, "range", r.range_, "")
	if r.count != nil {
		parameterAddToHeaderOrQuery(localVarQueryParams, "count", r.count, "")
	}
	if r.sorted != nil {
		parameterAddToHeaderOrQuery(localVarQueryParams, "sorted", r.sorted, "")
	}
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

type ApiRemoveKeyRequest struct {
	ctx context.Context
	ApiService *GeoshardApiAPIService
	index string
	removeKey *RemoveKey
}

// 
func (r ApiRemoveKeyRequest) RemoveKey(removeKey RemoveKey) ApiRemoveKeyRequest {
	r.removeKey = &removeKey
	return r
}

func (r ApiRemoveKeyRequest) Execute() (*RemoveKeyResponse, *http.Response, error) {
	return r.ApiService.RemoveKeyExecute(r)
}

/*
RemoveKey Remove key from index

Removes key from geospatial index

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @param index Geospatial index name
 @return ApiRemoveKeyRequest
*/
func (a *GeoshardApiAPIService) RemoveKey(ctx context.Context, index string) ApiRemoveKeyRequest {
	return ApiRemoveKeyRequest{
		ApiService: a,
		ctx: ctx,
		index: index,
	}
}

// Execute executes the request
//  @return RemoveKeyResponse
func (a *GeoshardApiAPIService) RemoveKeyExecute(r ApiRemoveKeyRequest) (*RemoveKeyResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodPatch
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *RemoveKeyResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeoshardApiAPIService.RemoveKey")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/shard/{index}/"
	localVarPath = strings.Replace(localVarPath, "{"+"index"+"}", url.PathEscape(parameterValueToString(r.index, "index")), -1)

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}
	if r.removeKey == nil {
		return localVarReturnValue, nil, reportError("removeKey is required and must be specified")
	}

	// to determine the Content-Type header
	localVarHTTPContentTypes := []string{"application/json"}

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
	// body params
	localVarPostBody = r.removeKey
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

type ApiRemoveKeyBatchRequest struct {
	ctx context.Context
	ApiService *GeoshardApiAPIService
	index string
	removeKeyBatch *RemoveKeyBatch
}

// 
func (r ApiRemoveKeyBatchRequest) RemoveKeyBatch(removeKeyBatch RemoveKeyBatch) ApiRemoveKeyBatchRequest {
	r.removeKeyBatch = &removeKeyBatch
	return r
}

func (r ApiRemoveKeyBatchRequest) Execute() (*RemoveKeyBatchResponse, *http.Response, error) {
	return r.ApiService.RemoveKeyBatchExecute(r)
}

/*
RemoveKeyBatch Remove multiple keys from index

Removes multiple keys from geospatial index

 @param ctx context.Context - for authentication, logging, cancellation, deadlines, tracing, etc. Passed from http.Request or context.Background().
 @param index Geospatial index name
 @return ApiRemoveKeyBatchRequest
*/
func (a *GeoshardApiAPIService) RemoveKeyBatch(ctx context.Context, index string) ApiRemoveKeyBatchRequest {
	return ApiRemoveKeyBatchRequest{
		ApiService: a,
		ctx: ctx,
		index: index,
	}
}

// Execute executes the request
//  @return RemoveKeyBatchResponse
func (a *GeoshardApiAPIService) RemoveKeyBatchExecute(r ApiRemoveKeyBatchRequest) (*RemoveKeyBatchResponse, *http.Response, error) {
	var (
		localVarHTTPMethod   = http.MethodPatch
		localVarPostBody     interface{}
		formFiles            []formFile
		localVarReturnValue  *RemoveKeyBatchResponse
	)

	localBasePath, err := a.client.cfg.ServerURLWithContext(r.ctx, "GeoshardApiAPIService.RemoveKeyBatch")
	if err != nil {
		return localVarReturnValue, nil, &GenericOpenAPIError{error: err.Error()}
	}

	localVarPath := localBasePath + "/api/v1/shard/{index}/batch/"
	localVarPath = strings.Replace(localVarPath, "{"+"index"+"}", url.PathEscape(parameterValueToString(r.index, "index")), -1)

	localVarHeaderParams := make(map[string]string)
	localVarQueryParams := url.Values{}
	localVarFormParams := url.Values{}
	if r.removeKeyBatch == nil {
		return localVarReturnValue, nil, reportError("removeKeyBatch is required and must be specified")
	}

	// to determine the Content-Type header
	localVarHTTPContentTypes := []string{"application/json"}

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
	// body params
	localVarPostBody = r.removeKeyBatch
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
