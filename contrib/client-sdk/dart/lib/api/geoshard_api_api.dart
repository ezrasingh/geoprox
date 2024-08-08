//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;


class GeoshardApiApi {
  GeoshardApiApi([ApiClient? apiClient]) : apiClient = apiClient ?? defaultApiClient;

  final ApiClient apiClient;

  /// Create geospatial index
  ///
  /// Creates an in-memory index within this geoshard
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  Future<Response> createIndexWithHttpInfo(String index,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/shard/{index}/'
      .replaceAll('{index}', index);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'POST',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Create geospatial index
  ///
  /// Creates an in-memory index within this geoshard
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  Future<CreateIndexResponse?> createIndex(String index,) async {
    final response = await createIndexWithHttpInfo(index,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'CreateIndexResponse',) as CreateIndexResponse;
    
    }
    return null;
  }

  /// Deletes geospatial index
  ///
  /// Drop index. All keys will be lost
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  Future<Response> dropIndexWithHttpInfo(String index,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/shard/{index}/'
      .replaceAll('{index}', index);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'DELETE',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Deletes geospatial index
  ///
  /// Drop index. All keys will be lost
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  Future<DropIndexResponse?> dropIndex(String index,) async {
    final response = await dropIndexWithHttpInfo(index,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'DropIndexResponse',) as DropIndexResponse;
    
    }
    return null;
  }

  /// Insert key into index
  ///
  /// Inserts key into geospatial index
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  ///
  /// * [InsertKey] insertKey (required):
  ///   
  Future<Response> insertKeyWithHttpInfo(String index, InsertKey insertKey,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/shard/{index}/'
      .replaceAll('{index}', index);

    // ignore: prefer_final_locals
    Object? postBody = insertKey;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PUT',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Insert key into index
  ///
  /// Inserts key into geospatial index
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  ///
  /// * [InsertKey] insertKey (required):
  ///   
  Future<InsertKeyResponse?> insertKey(String index, InsertKey insertKey,) async {
    final response = await insertKeyWithHttpInfo(index, insertKey,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'InsertKeyResponse',) as InsertKeyResponse;
    
    }
    return null;
  }

  /// Insert multiple keys into index
  ///
  /// Inserts multiple keys into geospatial index
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  ///
  /// * [InsertKeyBatch] insertKeyBatch (required):
  ///   
  Future<Response> insertKeyBatchWithHttpInfo(String index, InsertKeyBatch insertKeyBatch,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/shard/{index}/batch/'
      .replaceAll('{index}', index);

    // ignore: prefer_final_locals
    Object? postBody = insertKeyBatch;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PUT',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Insert multiple keys into index
  ///
  /// Inserts multiple keys into geospatial index
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  ///
  /// * [InsertKeyBatch] insertKeyBatch (required):
  ///   
  Future<InsertKeyBatchResponse?> insertKeyBatch(String index, InsertKeyBatch insertKeyBatch,) async {
    final response = await insertKeyBatchWithHttpInfo(index, insertKeyBatch,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'InsertKeyBatchResponse',) as InsertKeyBatchResponse;
    
    }
    return null;
  }

  /// Search index for objects nearby
  ///
  /// Search geospatial index for all keys within some distance
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  ///
  /// * [double] lat (required):
  ///   Latitude
  ///
  /// * [double] lng (required):
  ///   Longitude
  ///
  /// * [int] range (required):
  ///   Search radius in kilometers
  ///
  /// * [int] count:
  ///   Maximum number of neighbors that can be returned (default 100)
  ///
  /// * [bool] sorted:
  ///   If enabled neighbors will be sorted by distance, nearest to furthest (default false)
  Future<Response> queryRangeWithHttpInfo(String index, double lat, double lng, int range, { int? count, bool? sorted, }) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/shard/{index}/'
      .replaceAll('{index}', index);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

      queryParams.addAll(_queryParams('', 'lat', lat));
      queryParams.addAll(_queryParams('', 'lng', lng));
      queryParams.addAll(_queryParams('', 'range', range));
    if (count != null) {
      queryParams.addAll(_queryParams('', 'count', count));
    }
    if (sorted != null) {
      queryParams.addAll(_queryParams('', 'sorted', sorted));
    }

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Search index for objects nearby
  ///
  /// Search geospatial index for all keys within some distance
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  ///
  /// * [double] lat (required):
  ///   Latitude
  ///
  /// * [double] lng (required):
  ///   Longitude
  ///
  /// * [int] range (required):
  ///   Search radius in kilometers
  ///
  /// * [int] count:
  ///   Maximum number of neighbors that can be returned (default 100)
  ///
  /// * [bool] sorted:
  ///   If enabled neighbors will be sorted by distance, nearest to furthest (default false)
  Future<QueryRangeResponse?> queryRange(String index, double lat, double lng, int range, { int? count, bool? sorted, }) async {
    final response = await queryRangeWithHttpInfo(index, lat, lng, range,  count: count, sorted: sorted, );
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'QueryRangeResponse',) as QueryRangeResponse;
    
    }
    return null;
  }

  /// Search multiple indices for objects nearby
  ///
  /// Search geospatial many indices for all keys within some distance
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [List<String>] indices (required):
  ///   List of indices to search
  ///
  /// * [double] lat (required):
  ///   Latitude
  ///
  /// * [double] lng (required):
  ///   Longitude
  ///
  /// * [int] range (required):
  ///   Search radius in kilometers
  ///
  /// * [int] count:
  ///   Maximum number of neighbors that can be returned (default 100)
  ///
  /// * [bool] sorted:
  ///   If enabled neighbors will be sorted by distance, nearest to furthest (default false)
  Future<Response> queryRangeManyWithHttpInfo(List<String> indices, double lat, double lng, int range, { int? count, bool? sorted, }) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/shard/';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

      queryParams.addAll(_queryParams('multi', 'indices', indices));
      queryParams.addAll(_queryParams('', 'lat', lat));
      queryParams.addAll(_queryParams('', 'lng', lng));
      queryParams.addAll(_queryParams('', 'range', range));
    if (count != null) {
      queryParams.addAll(_queryParams('', 'count', count));
    }
    if (sorted != null) {
      queryParams.addAll(_queryParams('', 'sorted', sorted));
    }

    const contentTypes = <String>[];


    return apiClient.invokeAPI(
      path,
      'GET',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Search multiple indices for objects nearby
  ///
  /// Search geospatial many indices for all keys within some distance
  ///
  /// Parameters:
  ///
  /// * [List<String>] indices (required):
  ///   List of indices to search
  ///
  /// * [double] lat (required):
  ///   Latitude
  ///
  /// * [double] lng (required):
  ///   Longitude
  ///
  /// * [int] range (required):
  ///   Search radius in kilometers
  ///
  /// * [int] count:
  ///   Maximum number of neighbors that can be returned (default 100)
  ///
  /// * [bool] sorted:
  ///   If enabled neighbors will be sorted by distance, nearest to furthest (default false)
  Future<QueryRangeManyResponse?> queryRangeMany(List<String> indices, double lat, double lng, int range, { int? count, bool? sorted, }) async {
    final response = await queryRangeManyWithHttpInfo(indices, lat, lng, range,  count: count, sorted: sorted, );
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'QueryRangeManyResponse',) as QueryRangeManyResponse;
    
    }
    return null;
  }

  /// Remove key from index
  ///
  /// Removes key from geospatial index
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  ///
  /// * [RemoveKey] removeKey (required):
  ///   
  Future<Response> removeKeyWithHttpInfo(String index, RemoveKey removeKey,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/shard/{index}/'
      .replaceAll('{index}', index);

    // ignore: prefer_final_locals
    Object? postBody = removeKey;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PATCH',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Remove key from index
  ///
  /// Removes key from geospatial index
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  ///
  /// * [RemoveKey] removeKey (required):
  ///   
  Future<RemoveKeyResponse?> removeKey(String index, RemoveKey removeKey,) async {
    final response = await removeKeyWithHttpInfo(index, removeKey,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RemoveKeyResponse',) as RemoveKeyResponse;
    
    }
    return null;
  }

  /// Remove multiple keys from index
  ///
  /// Removes multiple keys from geospatial index
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  ///
  /// * [RemoveKeyBatch] removeKeyBatch (required):
  ///   
  Future<Response> removeKeyBatchWithHttpInfo(String index, RemoveKeyBatch removeKeyBatch,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/shard/{index}/batch/'
      .replaceAll('{index}', index);

    // ignore: prefer_final_locals
    Object? postBody = removeKeyBatch;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

    const contentTypes = <String>['application/json'];


    return apiClient.invokeAPI(
      path,
      'PATCH',
      queryParams,
      postBody,
      headerParams,
      formParams,
      contentTypes.isEmpty ? null : contentTypes.first,
    );
  }

  /// Remove multiple keys from index
  ///
  /// Removes multiple keys from geospatial index
  ///
  /// Parameters:
  ///
  /// * [String] index (required):
  ///   Geospatial index name
  ///
  /// * [RemoveKeyBatch] removeKeyBatch (required):
  ///   
  Future<RemoveKeyBatchResponse?> removeKeyBatch(String index, RemoveKeyBatch removeKeyBatch,) async {
    final response = await removeKeyBatchWithHttpInfo(index, removeKeyBatch,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'RemoveKeyBatchResponse',) as RemoveKeyBatchResponse;
    
    }
    return null;
  }
}
