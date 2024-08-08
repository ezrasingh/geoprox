//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

library geoprox_client.api;

import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:collection/collection.dart';
import 'package:http/http.dart';
import 'package:intl/intl.dart';
import 'package:meta/meta.dart';

part 'api_client.dart';
part 'api_helper.dart';
part 'api_exception.dart';
part 'auth/authentication.dart';
part 'auth/api_key_auth.dart';
part 'auth/oauth.dart';
part 'auth/http_basic_auth.dart';
part 'auth/http_bearer_auth.dart';

part 'api/geohash_api_api.dart';
part 'api/geoshard_api_api.dart';

part 'model/create_index_response.dart';
part 'model/decode_geohash_response.dart';
part 'model/drop_index_response.dart';
part 'model/encode_lat_lng.dart';
part 'model/encode_lat_lng_response.dart';
part 'model/geohash_neighbors_response.dart';
part 'model/insert_key.dart';
part 'model/insert_key_batch.dart';
part 'model/insert_key_batch_response.dart';
part 'model/insert_key_response.dart';
part 'model/neighbor.dart';
part 'model/query_range.dart';
part 'model/query_range_many.dart';
part 'model/query_range_many_response.dart';
part 'model/query_range_response.dart';
part 'model/remove_key.dart';
part 'model/remove_key_batch.dart';
part 'model/remove_key_batch_response.dart';
part 'model/remove_key_response.dart';


/// An [ApiClient] instance that uses the default values obtained from
/// the OpenAPI specification file.
var defaultApiClient = ApiClient();

const _delimiters = {'csv': ',', 'ssv': ' ', 'tsv': '\t', 'pipes': '|'};
const _dateEpochMarker = 'epoch';
const _deepEquality = DeepCollectionEquality();
final _dateFormatter = DateFormat('yyyy-MM-dd');
final _regList = RegExp(r'^List<(.*)>$');
final _regSet = RegExp(r'^Set<(.*)>$');
final _regMap = RegExp(r'^Map<String,(.*)>$');

bool _isEpochMarker(String? pattern) => pattern == _dateEpochMarker || pattern == '/$_dateEpochMarker/';
