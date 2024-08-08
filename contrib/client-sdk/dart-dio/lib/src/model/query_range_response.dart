//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_collection/built_collection.dart';
import 'package:geoprox_client_dio/src/model/neighbor.dart';
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'query_range_response.g.dart';

/// Returns object keys found with their distance
///
/// Properties:
/// * [found] - Object keys found within range
@BuiltValue()
abstract class QueryRangeResponse implements Built<QueryRangeResponse, QueryRangeResponseBuilder> {
  /// Object keys found within range
  @BuiltValueField(wireName: r'found')
  BuiltList<Neighbor> get found;

  QueryRangeResponse._();

  factory QueryRangeResponse([void updates(QueryRangeResponseBuilder b)]) = _$QueryRangeResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(QueryRangeResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<QueryRangeResponse> get serializer => _$QueryRangeResponseSerializer();
}

class _$QueryRangeResponseSerializer implements PrimitiveSerializer<QueryRangeResponse> {
  @override
  final Iterable<Type> types = const [QueryRangeResponse, _$QueryRangeResponse];

  @override
  final String wireName = r'QueryRangeResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    QueryRangeResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'found';
    yield serializers.serialize(
      object.found,
      specifiedType: const FullType(BuiltList, [FullType(Neighbor)]),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    QueryRangeResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required QueryRangeResponseBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'found':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(BuiltList, [FullType(Neighbor)]),
          ) as BuiltList<Neighbor>;
          result.found.replace(valueDes);
          break;
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  QueryRangeResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = QueryRangeResponseBuilder();
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

