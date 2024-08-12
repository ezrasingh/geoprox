# \GeohashApiAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DecodeGeohash**](GeohashApiAPI.md#DecodeGeohash) | **Get** /api/v1/geohash/{ghash} | Decode geohash into coordinates.
[**EncodeLatlng**](GeohashApiAPI.md#EncodeLatlng) | **Get** /api/v1/geohash | Encode coordinates into geohash
[**GetNeighbors**](GeohashApiAPI.md#GetNeighbors) | **Get** /api/v1/geohash/{ghash}/neighbors | Neighboring regions



## DecodeGeohash

> DecodeGeohashResponse DecodeGeohash(ctx, ghash).Execute()

Decode geohash into coordinates.



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
	ghash := "ghash_example" // string | Geohash encoded region

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeohashApiAPI.DecodeGeohash(context.Background(), ghash).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeohashApiAPI.DecodeGeohash``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `DecodeGeohash`: DecodeGeohashResponse
	fmt.Fprintf(os.Stdout, "Response from `GeohashApiAPI.DecodeGeohash`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**ghash** | **string** | Geohash encoded region | 

### Other Parameters

Other parameters are passed through a pointer to a apiDecodeGeohashRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**DecodeGeohashResponse**](DecodeGeohashResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## EncodeLatlng

> EncodeLatLngResponse EncodeLatlng(ctx).Lat(lat).Lng(lng).Depth(depth).Execute()

Encode coordinates into geohash



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
	lat := float64(1.2) // float64 | Latitude
	lng := float64(1.2) // float64 | Longitude
	depth := int32(56) // int32 | Determines geohash length

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeohashApiAPI.EncodeLatlng(context.Background()).Lat(lat).Lng(lng).Depth(depth).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeohashApiAPI.EncodeLatlng``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `EncodeLatlng`: EncodeLatLngResponse
	fmt.Fprintf(os.Stdout, "Response from `GeohashApiAPI.EncodeLatlng`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiEncodeLatlngRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lat** | **float64** | Latitude | 
 **lng** | **float64** | Longitude | 
 **depth** | **int32** | Determines geohash length | 

### Return type

[**EncodeLatLngResponse**](EncodeLatLngResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetNeighbors

> GeohashNeighborsResponse GetNeighbors(ctx, ghash).Execute()

Neighboring regions



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
	ghash := "ghash_example" // string | Geohash encoded region

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.GeohashApiAPI.GetNeighbors(context.Background(), ghash).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `GeohashApiAPI.GetNeighbors``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetNeighbors`: GeohashNeighborsResponse
	fmt.Fprintf(os.Stdout, "Response from `GeohashApiAPI.GetNeighbors`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**ghash** | **string** | Geohash encoded region | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetNeighborsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**GeohashNeighborsResponse**](GeohashNeighborsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

