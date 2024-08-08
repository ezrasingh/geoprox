# GeoproxClient.Api.GeohashApiApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|--------|--------------|-------------|
| [**DecodeGeohash**](GeohashApiApi.md#decodegeohash) | **GET** /api/v1/geohash/{ghash}/ | Decode geohash into coordinates. |
| [**EncodeLatlng**](GeohashApiApi.md#encodelatlng) | **GET** /api/v1/geohash/ | Encode coordinates into geohash |
| [**GetNeighbors**](GeohashApiApi.md#getneighbors) | **GET** /api/v1/geohash/{ghash}/neighbors/ | Neighboring regions |

<a id="decodegeohash"></a>
# **DecodeGeohash**
> DecodeGeohashResponse DecodeGeohash (string ghash)

Decode geohash into coordinates.

Decode geohash by path param, returns coordinates with precision estimates.

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class DecodeGeohashExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeohashApiApi(config);
            var ghash = "ghash_example";  // string | Geohash encoded region

            try
            {
                // Decode geohash into coordinates.
                DecodeGeohashResponse result = apiInstance.DecodeGeohash(ghash);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeohashApiApi.DecodeGeohash: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the DecodeGeohashWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Decode geohash into coordinates.
    ApiResponse<DecodeGeohashResponse> response = apiInstance.DecodeGeohashWithHttpInfo(ghash);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeohashApiApi.DecodeGeohashWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **ghash** | **string** | Geohash encoded region |  |

### Return type

[**DecodeGeohashResponse**](DecodeGeohashResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Object with latitude/longitude pair and precision errors |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a id="encodelatlng"></a>
# **EncodeLatlng**
> EncodeLatLngResponse EncodeLatlng (double lat, double lng, int depth)

Encode coordinates into geohash

Encode coordinates by query params, returns geohash.

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class EncodeLatlngExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeohashApiApi(config);
            var lat = 1.2D;  // double | Latitude
            var lng = 1.2D;  // double | Longitude
            var depth = 56;  // int | Determines geohash length

            try
            {
                // Encode coordinates into geohash
                EncodeLatLngResponse result = apiInstance.EncodeLatlng(lat, lng, depth);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeohashApiApi.EncodeLatlng: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the EncodeLatlngWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Encode coordinates into geohash
    ApiResponse<EncodeLatLngResponse> response = apiInstance.EncodeLatlngWithHttpInfo(lat, lng, depth);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeohashApiApi.EncodeLatlngWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **lat** | **double** | Latitude |  |
| **lng** | **double** | Longitude |  |
| **depth** | **int** | Determines geohash length |  |

### Return type

[**EncodeLatLngResponse**](EncodeLatLngResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Object with geohash encoded latitude/longitude |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a id="getneighbors"></a>
# **GetNeighbors**
> GeohashNeighborsResponse GetNeighbors (string ghash)

Neighboring regions

Returns geohash neighbors in all cardinal directions.

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class GetNeighborsExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeohashApiApi(config);
            var ghash = "ghash_example";  // string | Geohash encoded region

            try
            {
                // Neighboring regions
                GeohashNeighborsResponse result = apiInstance.GetNeighbors(ghash);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeohashApiApi.GetNeighbors: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the GetNeighborsWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Neighboring regions
    ApiResponse<GeohashNeighborsResponse> response = apiInstance.GetNeighborsWithHttpInfo(ghash);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeohashApiApi.GetNeighborsWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **ghash** | **string** | Geohash encoded region |  |

### Return type

[**GeohashNeighborsResponse**](GeohashNeighborsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Object with geohash neighbors |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

