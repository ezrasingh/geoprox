# GeoproxClient.Api.GeoshardApiApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|--------|--------------|-------------|
| [**CreateIndex**](GeoshardApiApi.md#createindex) | **POST** /api/v1/shard/{index}/ | Create geospatial index |
| [**DropIndex**](GeoshardApiApi.md#dropindex) | **DELETE** /api/v1/shard/{index}/ | Deletes geospatial index |
| [**InsertKey**](GeoshardApiApi.md#insertkey) | **PUT** /api/v1/shard/{index}/ | Insert key into index |
| [**InsertKeyBatch**](GeoshardApiApi.md#insertkeybatch) | **PUT** /api/v1/shard/{index}/batch/ | Insert multiple keys into index |
| [**QueryRange**](GeoshardApiApi.md#queryrange) | **GET** /api/v1/shard/{index}/ | Search index for objects nearby |
| [**QueryRangeMany**](GeoshardApiApi.md#queryrangemany) | **GET** /api/v1/shard/ | Search multiple indices for objects nearby |
| [**RemoveKey**](GeoshardApiApi.md#removekey) | **PATCH** /api/v1/shard/{index}/ | Remove key from index |
| [**RemoveKeyBatch**](GeoshardApiApi.md#removekeybatch) | **PATCH** /api/v1/shard/{index}/batch/ | Remove multiple keys from index |

<a id="createindex"></a>
# **CreateIndex**
> CreateIndexResponse CreateIndex (string index)

Create geospatial index

Creates an in-memory index within this geoshard

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class CreateIndexExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeoshardApiApi(config);
            var index = "index_example";  // string | Geospatial index name

            try
            {
                // Create geospatial index
                CreateIndexResponse result = apiInstance.CreateIndex(index);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeoshardApiApi.CreateIndex: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the CreateIndexWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Create geospatial index
    ApiResponse<CreateIndexResponse> response = apiInstance.CreateIndexWithHttpInfo(index);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeoshardApiApi.CreateIndexWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **index** | **string** | Geospatial index name |  |

### Return type

[**CreateIndexResponse**](CreateIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Created an index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a id="dropindex"></a>
# **DropIndex**
> DropIndexResponse DropIndex (string index)

Deletes geospatial index

Drop index. All keys will be lost

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class DropIndexExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeoshardApiApi(config);
            var index = "index_example";  // string | Geospatial index name

            try
            {
                // Deletes geospatial index
                DropIndexResponse result = apiInstance.DropIndex(index);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeoshardApiApi.DropIndex: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the DropIndexWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Deletes geospatial index
    ApiResponse<DropIndexResponse> response = apiInstance.DropIndexWithHttpInfo(index);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeoshardApiApi.DropIndexWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **index** | **string** | Geospatial index name |  |

### Return type

[**DropIndexResponse**](DropIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **202** | Index deleted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a id="insertkey"></a>
# **InsertKey**
> InsertKeyResponse InsertKey (string index, InsertKey insertKey)

Insert key into index

Inserts key into geospatial index

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class InsertKeyExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeoshardApiApi(config);
            var index = "index_example";  // string | Geospatial index name
            var insertKey = new InsertKey(); // InsertKey | 

            try
            {
                // Insert key into index
                InsertKeyResponse result = apiInstance.InsertKey(index, insertKey);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeoshardApiApi.InsertKey: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the InsertKeyWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Insert key into index
    ApiResponse<InsertKeyResponse> response = apiInstance.InsertKeyWithHttpInfo(index, insertKey);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeoshardApiApi.InsertKeyWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **index** | **string** | Geospatial index name |  |
| **insertKey** | [**InsertKey**](InsertKey.md) |  |  |

### Return type

[**InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Inserted key into the index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a id="insertkeybatch"></a>
# **InsertKeyBatch**
> InsertKeyBatchResponse InsertKeyBatch (string index, InsertKeyBatch insertKeyBatch)

Insert multiple keys into index

Inserts multiple keys into geospatial index

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class InsertKeyBatchExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeoshardApiApi(config);
            var index = "index_example";  // string | Geospatial index name
            var insertKeyBatch = new InsertKeyBatch(); // InsertKeyBatch | 

            try
            {
                // Insert multiple keys into index
                InsertKeyBatchResponse result = apiInstance.InsertKeyBatch(index, insertKeyBatch);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeoshardApiApi.InsertKeyBatch: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the InsertKeyBatchWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Insert multiple keys into index
    ApiResponse<InsertKeyBatchResponse> response = apiInstance.InsertKeyBatchWithHttpInfo(index, insertKeyBatch);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeoshardApiApi.InsertKeyBatchWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **index** | **string** | Geospatial index name |  |
| **insertKeyBatch** | [**InsertKeyBatch**](InsertKeyBatch.md) |  |  |

### Return type

[**InsertKeyBatchResponse**](InsertKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **201** | Inserted key batch into the index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a id="queryrange"></a>
# **QueryRange**
> QueryRangeResponse QueryRange (string index, double lat, double lng, int range, int? count = null, bool? sorted = null)

Search index for objects nearby

Search geospatial index for all keys within some distance

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class QueryRangeExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeoshardApiApi(config);
            var index = "index_example";  // string | Geospatial index name
            var lat = 1.2D;  // double | Latitude
            var lng = 1.2D;  // double | Longitude
            var range = 56;  // int | Search radius in kilometers
            var count = 56;  // int? | Maximum number of neighbors that can be returned (default 100) (optional) 
            var sorted = true;  // bool? | If enabled neighbors will be sorted by distance, nearest to furthest (default false) (optional) 

            try
            {
                // Search index for objects nearby
                QueryRangeResponse result = apiInstance.QueryRange(index, lat, lng, range, count, sorted);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeoshardApiApi.QueryRange: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the QueryRangeWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Search index for objects nearby
    ApiResponse<QueryRangeResponse> response = apiInstance.QueryRangeWithHttpInfo(index, lat, lng, range, count, sorted);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeoshardApiApi.QueryRangeWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **index** | **string** | Geospatial index name |  |
| **lat** | **double** | Latitude |  |
| **lng** | **double** | Longitude |  |
| **range** | **int** | Search radius in kilometers |  |
| **count** | **int?** | Maximum number of neighbors that can be returned (default 100) | [optional]  |
| **sorted** | **bool?** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional]  |

### Return type

[**QueryRangeResponse**](QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Nearby objects found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a id="queryrangemany"></a>
# **QueryRangeMany**
> QueryRangeManyResponse QueryRangeMany (List<string> indices, double lat, double lng, int range, int? count = null, bool? sorted = null)

Search multiple indices for objects nearby

Search geospatial many indices for all keys within some distance

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class QueryRangeManyExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeoshardApiApi(config);
            var indices = new List<string>(); // List<string> | List of indices to search
            var lat = 1.2D;  // double | Latitude
            var lng = 1.2D;  // double | Longitude
            var range = 56;  // int | Search radius in kilometers
            var count = 56;  // int? | Maximum number of neighbors that can be returned (default 100) (optional) 
            var sorted = true;  // bool? | If enabled neighbors will be sorted by distance, nearest to furthest (default false) (optional) 

            try
            {
                // Search multiple indices for objects nearby
                QueryRangeManyResponse result = apiInstance.QueryRangeMany(indices, lat, lng, range, count, sorted);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeoshardApiApi.QueryRangeMany: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the QueryRangeManyWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Search multiple indices for objects nearby
    ApiResponse<QueryRangeManyResponse> response = apiInstance.QueryRangeManyWithHttpInfo(indices, lat, lng, range, count, sorted);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeoshardApiApi.QueryRangeManyWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **indices** | [**List&lt;string&gt;**](string.md) | List of indices to search |  |
| **lat** | **double** | Latitude |  |
| **lng** | **double** | Longitude |  |
| **range** | **int** | Search radius in kilometers |  |
| **count** | **int?** | Maximum number of neighbors that can be returned (default 100) | [optional]  |
| **sorted** | **bool?** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional]  |

### Return type

[**QueryRangeManyResponse**](QueryRangeManyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Nearby objects found across indices |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a id="removekey"></a>
# **RemoveKey**
> RemoveKeyResponse RemoveKey (string index, RemoveKey removeKey)

Remove key from index

Removes key from geospatial index

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class RemoveKeyExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeoshardApiApi(config);
            var index = "index_example";  // string | Geospatial index name
            var removeKey = new RemoveKey(); // RemoveKey | 

            try
            {
                // Remove key from index
                RemoveKeyResponse result = apiInstance.RemoveKey(index, removeKey);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeoshardApiApi.RemoveKey: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the RemoveKeyWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Remove key from index
    ApiResponse<RemoveKeyResponse> response = apiInstance.RemoveKeyWithHttpInfo(index, removeKey);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeoshardApiApi.RemoveKeyWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **index** | **string** | Geospatial index name |  |
| **removeKey** | [**RemoveKey**](RemoveKey.md) |  |  |

### Return type

[**RemoveKeyResponse**](RemoveKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Key removed from the index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a id="removekeybatch"></a>
# **RemoveKeyBatch**
> RemoveKeyBatchResponse RemoveKeyBatch (string index, RemoveKeyBatch removeKeyBatch)

Remove multiple keys from index

Removes multiple keys from geospatial index

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using GeoproxClient.Api;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace Example
{
    public class RemoveKeyBatchExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new GeoshardApiApi(config);
            var index = "index_example";  // string | Geospatial index name
            var removeKeyBatch = new RemoveKeyBatch(); // RemoveKeyBatch | 

            try
            {
                // Remove multiple keys from index
                RemoveKeyBatchResponse result = apiInstance.RemoveKeyBatch(index, removeKeyBatch);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling GeoshardApiApi.RemoveKeyBatch: " + e.Message);
                Debug.Print("Status Code: " + e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

#### Using the RemoveKeyBatchWithHttpInfo variant
This returns an ApiResponse object which contains the response data, status code and headers.

```csharp
try
{
    // Remove multiple keys from index
    ApiResponse<RemoveKeyBatchResponse> response = apiInstance.RemoveKeyBatchWithHttpInfo(index, removeKeyBatch);
    Debug.Write("Status Code: " + response.StatusCode);
    Debug.Write("Response Headers: " + response.Headers);
    Debug.Write("Response Body: " + response.Data);
}
catch (ApiException e)
{
    Debug.Print("Exception when calling GeoshardApiApi.RemoveKeyBatchWithHttpInfo: " + e.Message);
    Debug.Print("Status Code: " + e.ErrorCode);
    Debug.Print(e.StackTrace);
}
```

### Parameters

| Name | Type | Description | Notes |
|------|------|-------------|-------|
| **index** | **string** | Geospatial index name |  |
| **removeKeyBatch** | [**RemoveKeyBatch**](RemoveKeyBatch.md) |  |  |

### Return type

[**RemoveKeyBatchResponse**](RemoveKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | All keys were removed from the index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

