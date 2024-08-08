# geoprox_client_dio.model.QueryRangeMany

## Load the model package
```dart
import 'package:geoprox_client_dio/api.dart';
```

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | **int** | Maximum number of neighbors that can be returned (default 100) | [optional] 
**indices** | **BuiltList&lt;String&gt;** | List of indices to search | 
**lat** | **double** | Latitude | 
**lng** | **double** | Longitude | 
**range** | **int** | Search radius in kilometers | 
**sorted** | **bool** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


