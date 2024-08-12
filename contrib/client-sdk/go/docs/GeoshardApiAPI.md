# \GeoshardApiAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateIndex**](GeoshardApiAPI.md#CreateIndex) | **Post** /api/v1/shard/{index} | Create geospatial index
[**DropIndex**](GeoshardApiAPI.md#DropIndex) | **Delete** /api/v1/shard/{index} | Deletes geospatial index
[**InsertKey**](GeoshardApiAPI.md#InsertKey) | **Put** /api/v1/shard/{index} | Insert key into index
[**InsertKeyBatch**](GeoshardApiAPI.md#InsertKeyBatch) | **Put** /api/v1/shard/{index}/batch | Insert multiple keys into index
[**QueryRange**](GeoshardApiAPI.md#QueryRange) | **Get** /api/v1/shard/{index} | Search index for objects nearby
[**QueryRangeMany**](GeoshardApiAPI.md#QueryRangeMany) | **Get** /api/v1/shard | Search multiple indices for objects nearby
[**RemoveKey**](GeoshardApiAPI.md#RemoveKey) | **Patch** /api/v1/shard/{index} | Remove key from index
[**RemoveKeyBatch**](GeoshardApiAPI.md#RemoveKeyBatch) | **Patch** /api/v1/shard/{index}/batch | Remove multiple keys from index



## CreateIndex

> CreateIndexResponse CreateIndex(ctx, index).Execute()

Create geospatial index



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	index := "index_example" // string | Geospatial index name

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeoshardApiAPI.CreateIndex(context.Background(), index).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeoshardApiAPI.CreateIndex``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateIndex`: CreateIndexResponse
	fmt.Fprintf(os.Stdout, "Response from `GeoshardApiAPI.CreateIndex`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**index** | **string** | Geospatial index name | 

### Other Parameters

Other parameters are passed through a pointer to a apiCreateIndexRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**CreateIndexResponse**](CreateIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## DropIndex

> DropIndexResponse DropIndex(ctx, index).Execute()

Deletes geospatial index



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	index := "index_example" // string | Geospatial index name

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeoshardApiAPI.DropIndex(context.Background(), index).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeoshardApiAPI.DropIndex``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DropIndex`: DropIndexResponse
	fmt.Fprintf(os.Stdout, "Response from `GeoshardApiAPI.DropIndex`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**index** | **string** | Geospatial index name | 

### Other Parameters

Other parameters are passed through a pointer to a apiDropIndexRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**DropIndexResponse**](DropIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## InsertKey

> InsertKeyResponse InsertKey(ctx, index).InsertKey(insertKey).Execute()

Insert key into index



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	index := "index_example" // string | Geospatial index name
	insertKey := *openapiclient.NewInsertKey("Key_example", float64(123), float64(123)) // InsertKey | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeoshardApiAPI.InsertKey(context.Background(), index).InsertKey(insertKey).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeoshardApiAPI.InsertKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `InsertKey`: InsertKeyResponse
	fmt.Fprintf(os.Stdout, "Response from `GeoshardApiAPI.InsertKey`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**index** | **string** | Geospatial index name | 

### Other Parameters

Other parameters are passed through a pointer to a apiInsertKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **insertKey** | [**InsertKey**](InsertKey.md) |  | 

### Return type

[**InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## InsertKeyBatch

> InsertKeyBatchResponse InsertKeyBatch(ctx, index).InsertKeyBatch(insertKeyBatch).Execute()

Insert multiple keys into index



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	index := "index_example" // string | Geospatial index name
	insertKeyBatch := *openapiclient.NewInsertKeyBatch([]openapiclient.InsertKey{*openapiclient.NewInsertKey("Key_example", float64(123), float64(123))}, false) // InsertKeyBatch | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeoshardApiAPI.InsertKeyBatch(context.Background(), index).InsertKeyBatch(insertKeyBatch).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeoshardApiAPI.InsertKeyBatch``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `InsertKeyBatch`: InsertKeyBatchResponse
	fmt.Fprintf(os.Stdout, "Response from `GeoshardApiAPI.InsertKeyBatch`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**index** | **string** | Geospatial index name | 

### Other Parameters

Other parameters are passed through a pointer to a apiInsertKeyBatchRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **insertKeyBatch** | [**InsertKeyBatch**](InsertKeyBatch.md) |  | 

### Return type

[**InsertKeyBatchResponse**](InsertKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## QueryRange

> QueryRangeResponse QueryRange(ctx, index).Lat(lat).Lng(lng).Range_(range_).Count(count).Sorted(sorted).Execute()

Search index for objects nearby



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	index := "index_example" // string | Geospatial index name
	lat := float64(1.2) // float64 | Latitude
	lng := float64(1.2) // float64 | Longitude
	range_ := int32(56) // int32 | Search radius in kilometers
	count := int32(56) // int32 | Maximum number of neighbors that can be returned (default 100) (optional)
	sorted := true // bool | If enabled neighbors will be sorted by distance, nearest to furthest (default false) (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeoshardApiAPI.QueryRange(context.Background(), index).Lat(lat).Lng(lng).Range_(range_).Count(count).Sorted(sorted).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeoshardApiAPI.QueryRange``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `QueryRange`: QueryRangeResponse
	fmt.Fprintf(os.Stdout, "Response from `GeoshardApiAPI.QueryRange`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**index** | **string** | Geospatial index name | 

### Other Parameters

Other parameters are passed through a pointer to a apiQueryRangeRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **lat** | **float64** | Latitude | 
 **lng** | **float64** | Longitude | 
 **range_** | **int32** | Search radius in kilometers | 
 **count** | **int32** | Maximum number of neighbors that can be returned (default 100) | 
 **sorted** | **bool** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | 

### Return type

[**QueryRangeResponse**](QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## QueryRangeMany

> QueryRangeManyResponse QueryRangeMany(ctx).Indices(indices).Lat(lat).Lng(lng).Range_(range_).Count(count).Sorted(sorted).Execute()

Search multiple indices for objects nearby



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	indices := []string{"Inner_example"} // []string | List of indices to search
	lat := float64(1.2) // float64 | Latitude
	lng := float64(1.2) // float64 | Longitude
	range_ := int32(56) // int32 | Search radius in kilometers
	count := int32(56) // int32 | Maximum number of neighbors that can be returned (default 100) (optional)
	sorted := true // bool | If enabled neighbors will be sorted by distance, nearest to furthest (default false) (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeoshardApiAPI.QueryRangeMany(context.Background()).Indices(indices).Lat(lat).Lng(lng).Range_(range_).Count(count).Sorted(sorted).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeoshardApiAPI.QueryRangeMany``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `QueryRangeMany`: QueryRangeManyResponse
	fmt.Fprintf(os.Stdout, "Response from `GeoshardApiAPI.QueryRangeMany`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiQueryRangeManyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **indices** | **[]string** | List of indices to search | 
 **lat** | **float64** | Latitude | 
 **lng** | **float64** | Longitude | 
 **range_** | **int32** | Search radius in kilometers | 
 **count** | **int32** | Maximum number of neighbors that can be returned (default 100) | 
 **sorted** | **bool** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | 

### Return type

[**QueryRangeManyResponse**](QueryRangeManyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RemoveKey

> RemoveKeyResponse RemoveKey(ctx, index).RemoveKey(removeKey).Execute()

Remove key from index



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	index := "index_example" // string | Geospatial index name
	removeKey := *openapiclient.NewRemoveKey("Key_example") // RemoveKey | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeoshardApiAPI.RemoveKey(context.Background(), index).RemoveKey(removeKey).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeoshardApiAPI.RemoveKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RemoveKey`: RemoveKeyResponse
	fmt.Fprintf(os.Stdout, "Response from `GeoshardApiAPI.RemoveKey`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**index** | **string** | Geospatial index name | 

### Other Parameters

Other parameters are passed through a pointer to a apiRemoveKeyRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **removeKey** | [**RemoveKey**](RemoveKey.md) |  | 

### Return type

[**RemoveKeyResponse**](RemoveKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## RemoveKeyBatch

> RemoveKeyBatchResponse RemoveKeyBatch(ctx, index).RemoveKeyBatch(removeKeyBatch).Execute()

Remove multiple keys from index



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func main() {
	index := "index_example" // string | Geospatial index name
	removeKeyBatch := *openapiclient.NewRemoveKeyBatch([]string{"Keys_example"}) // RemoveKeyBatch | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeoshardApiAPI.RemoveKeyBatch(context.Background(), index).RemoveKeyBatch(removeKeyBatch).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeoshardApiAPI.RemoveKeyBatch``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `RemoveKeyBatch`: RemoveKeyBatchResponse
	fmt.Fprintf(os.Stdout, "Response from `GeoshardApiAPI.RemoveKeyBatch`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**index** | **string** | Geospatial index name | 

### Other Parameters

Other parameters are passed through a pointer to a apiRemoveKeyBatchRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **removeKeyBatch** | [**RemoveKeyBatch**](RemoveKeyBatch.md) |  | 

### Return type

[**RemoveKeyBatchResponse**](RemoveKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

