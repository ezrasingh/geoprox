//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;


class GeohashApiApi {
  GeohashApiApi([ApiClient? apiClient]) : apiClient = apiClient ?? defaultApiClient;

  final ApiClient apiClient;

  /// Decode geohash into coordinates.
  ///
  /// Decode geohash by path param, returns coordinates with precision estimates.
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] ghash (required):
  ///   Geohash encoded region
  Future<Response> decodeGeohashWithHttpInfo(String ghash,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/geohash/{ghash}/'
      .replaceAll('{ghash}', ghash);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

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

  /// Decode geohash into coordinates.
  ///
  /// Decode geohash by path param, returns coordinates with precision estimates.
  ///
  /// Parameters:
  ///
  /// * [String] ghash (required):
  ///   Geohash encoded region
  Future<DecodeGeohashResponse?> decodeGeohash(String ghash,) async {
    final response = await decodeGeohashWithHttpInfo(ghash,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'DecodeGeohashResponse',) as DecodeGeohashResponse;
    
    }
    return null;
  }

  /// Encode coordinates into geohash
  ///
  /// Encode coordinates by query params, returns geohash.
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [double] lat (required):
  ///   Latitude
  ///
  /// * [double] lng (required):
  ///   Longitude
  ///
  /// * [int] depth (required):
  ///   Determines geohash length
  Future<Response> encodeLatlngWithHttpInfo(double lat, double lng, int depth,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/geohash/';

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

      queryParams.addAll(_queryParams('', 'lat', lat));
      queryParams.addAll(_queryParams('', 'lng', lng));
      queryParams.addAll(_queryParams('', 'depth', depth));

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

  /// Encode coordinates into geohash
  ///
  /// Encode coordinates by query params, returns geohash.
  ///
  /// Parameters:
  ///
  /// * [double] lat (required):
  ///   Latitude
  ///
  /// * [double] lng (required):
  ///   Longitude
  ///
  /// * [int] depth (required):
  ///   Determines geohash length
  Future<EncodeLatLngResponse?> encodeLatlng(double lat, double lng, int depth,) async {
    final response = await encodeLatlngWithHttpInfo(lat, lng, depth,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'EncodeLatLngResponse',) as EncodeLatLngResponse;
    
    }
    return null;
  }

  /// Neighboring regions
  ///
  /// Returns geohash neighbors in all cardinal directions.
  ///
  /// Note: This method returns the HTTP [Response].
  ///
  /// Parameters:
  ///
  /// * [String] ghash (required):
  ///   Geohash encoded region
  Future<Response> getNeighborsWithHttpInfo(String ghash,) async {
    // ignore: prefer_const_declarations
    final path = r'/api/v1/geohash/{ghash}/neighbors/'
      .replaceAll('{ghash}', ghash);

    // ignore: prefer_final_locals
    Object? postBody;

    final queryParams = <QueryParam>[];
    final headerParams = <String, String>{};
    final formParams = <String, String>{};

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

  /// Neighboring regions
  ///
  /// Returns geohash neighbors in all cardinal directions.
  ///
  /// Parameters:
  ///
  /// * [String] ghash (required):
  ///   Geohash encoded region
  Future<GeohashNeighborsResponse?> getNeighbors(String ghash,) async {
    final response = await getNeighborsWithHttpInfo(ghash,);
    if (response.statusCode >= HttpStatus.badRequest) {
      throw ApiException(response.statusCode, await _decodeBodyBytes(response));
    }
    // When a remote server returns no body with a status of 204, we shall not decode it.
    // At the time of writing this, `dart:convert` will throw an "Unexpected end of input"
    // FormatException when trying to decode an empty string.
    if (response.body.isNotEmpty && response.statusCode != HttpStatus.noContent) {
      return await apiClient.deserializeAsync(await _decodeBodyBytes(response), 'GeohashNeighborsResponse',) as GeohashNeighborsResponse;
    
    }
    return null;
  }
}
