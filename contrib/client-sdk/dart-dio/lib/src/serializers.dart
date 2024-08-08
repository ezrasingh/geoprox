//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_import

import 'package:one_of_serializer/any_of_serializer.dart';
import 'package:one_of_serializer/one_of_serializer.dart';
import 'package:built_collection/built_collection.dart';
import 'package:built_value/json_object.dart';
import 'package:built_value/serializer.dart';
import 'package:built_value/standard_json_plugin.dart';
import 'package:built_value/iso_8601_date_time_serializer.dart';
import 'package:geoprox_client_dio/src/date_serializer.dart';
import 'package:geoprox_client_dio/src/model/date.dart';

import 'package:geoprox_client_dio/src/model/create_index_response.dart';
import 'package:geoprox_client_dio/src/model/decode_geohash_response.dart';
import 'package:geoprox_client_dio/src/model/drop_index_response.dart';
import 'package:geoprox_client_dio/src/model/encode_lat_lng.dart';
import 'package:geoprox_client_dio/src/model/encode_lat_lng_response.dart';
import 'package:geoprox_client_dio/src/model/geohash_neighbors_response.dart';
import 'package:geoprox_client_dio/src/model/insert_key.dart';
import 'package:geoprox_client_dio/src/model/insert_key_batch.dart';
import 'package:geoprox_client_dio/src/model/insert_key_batch_response.dart';
import 'package:geoprox_client_dio/src/model/insert_key_response.dart';
import 'package:geoprox_client_dio/src/model/neighbor.dart';
import 'package:geoprox_client_dio/src/model/query_range.dart';
import 'package:geoprox_client_dio/src/model/query_range_many.dart';
import 'package:geoprox_client_dio/src/model/query_range_many_response.dart';
import 'package:geoprox_client_dio/src/model/query_range_response.dart';
import 'package:geoprox_client_dio/src/model/remove_key.dart';
import 'package:geoprox_client_dio/src/model/remove_key_batch.dart';
import 'package:geoprox_client_dio/src/model/remove_key_batch_response.dart';
import 'package:geoprox_client_dio/src/model/remove_key_response.dart';

part 'serializers.g.dart';

@SerializersFor([
  CreateIndexResponse,
  DecodeGeohashResponse,
  DropIndexResponse,
  EncodeLatLng,
  EncodeLatLngResponse,
  GeohashNeighborsResponse,
  InsertKey,
  InsertKeyBatch,
  InsertKeyBatchResponse,
  InsertKeyResponse,
  Neighbor,
  QueryRange,
  QueryRangeMany,
  QueryRangeManyResponse,
  QueryRangeResponse,
  RemoveKey,
  RemoveKeyBatch,
  RemoveKeyBatchResponse,
  RemoveKeyResponse,
])
Serializers serializers = (_$serializers.toBuilder()
      ..addBuilderFactory(
        const FullType(BuiltList, [FullType(String)]),
        () => ListBuilder<String>(),
      )
      ..add(const OneOfSerializer())
      ..add(const AnyOfSerializer())
      ..add(const DateSerializer())
      ..add(Iso8601DateTimeSerializer()))
    .build();

Serializers standardSerializers =
    (serializers.toBuilder()..addPlugin(StandardJsonPlugin())).build();
