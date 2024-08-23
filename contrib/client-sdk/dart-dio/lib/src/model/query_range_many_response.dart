//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_collection/built_collection.dart';
import 'package:geoprox_client_dio/src/model/neighbor.dart';
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'query_range_many_response.g.dart';

/// Returns indexes and object keys found with their distance
///
/// Properties:
/// * [errors] - Contains information about any errors occured during batch search.
/// * [results] - Object keys found within range
@BuiltValue()
abstract class QueryRangeManyResponse implements Built<QueryRangeManyResponse, QueryRangeManyResponseBuilder> {
  /// Contains information about any errors occured during batch search.
  @BuiltValueField(wireName: r'errors')
  BuiltMap<String, String> get errors;

  /// Object keys found within range
  @BuiltValueField(wireName: r'results')
  BuiltMap<String, BuiltList<Neighbor>> get results;

  QueryRangeManyResponse._();

  factory QueryRangeManyResponse([void updates(QueryRangeManyResponseBuilder b)]) = _$QueryRangeManyResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(QueryRangeManyResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<QueryRangeManyResponse> get serializer => _$QueryRangeManyResponseSerializer();
}

class _$QueryRangeManyResponseSerializer implements PrimitiveSerializer<QueryRangeManyResponse> {
  @override
  final Iterable<Type> types = const [QueryRangeManyResponse, _$QueryRangeManyResponse];

  @override
  final String wireName = r'QueryRangeManyResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    QueryRangeManyResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'errors';
    yield serializers.serialize(
      object.errors,
      specifiedType: const FullType(BuiltMap, [FullType(String), FullType(String)]),
    );
    yield r'results';
    yield serializers.serialize(
      object.results,
      specifiedType: const FullType(BuiltMap, [FullType(String), FullType(BuiltList, [FullType(Neighbor)])]),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    QueryRangeManyResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required QueryRangeManyResponseBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'errors':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(BuiltMap, [FullType(String), FullType(String)]),
          ) as BuiltMap<String, String>;
          result.errors.replace(valueDes);
          break;
        case r'results':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(BuiltMap, [FullType(String), FullType(BuiltList, [FullType(Neighbor)])]),
          ) as BuiltMap<String, BuiltList<Neighbor>>;
          result.results.replace(valueDes);
          break;
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  QueryRangeManyResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = QueryRangeManyResponseBuilder();
    final serializedList = (serialized as Iterable<Object?>).toList();
    final unhandled = <Object?>[];
    _deserializeProperties(
      serializers,
      serialized,
      specifiedType: specifiedType,
      serializedList: serializedList,
      unhandled: unhandled,
      result: result,
    );
    return result.build();
  }
}

