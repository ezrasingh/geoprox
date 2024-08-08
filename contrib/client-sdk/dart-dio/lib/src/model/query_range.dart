//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'query_range.g.dart';

/// Arguments for range query
///
/// Properties:
/// * [count] - Maximum number of neighbors that can be returned (default 100)
/// * [lat] - Latitude
/// * [lng] - Longitude
/// * [range] - Search radius in kilometers
/// * [sorted] - If enabled neighbors will be sorted by distance, nearest to furthest (default false)
@BuiltValue()
abstract class QueryRange implements Built<QueryRange, QueryRangeBuilder> {
  /// Maximum number of neighbors that can be returned (default 100)
  @BuiltValueField(wireName: r'count')
  int? get count;

  /// Latitude
  @BuiltValueField(wireName: r'lat')
  double get lat;

  /// Longitude
  @BuiltValueField(wireName: r'lng')
  double get lng;

  /// Search radius in kilometers
  @BuiltValueField(wireName: r'range')
  int get range;

  /// If enabled neighbors will be sorted by distance, nearest to furthest (default false)
  @BuiltValueField(wireName: r'sorted')
  bool? get sorted;

  QueryRange._();

  factory QueryRange([void updates(QueryRangeBuilder b)]) = _$QueryRange;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(QueryRangeBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<QueryRange> get serializer => _$QueryRangeSerializer();
}

class _$QueryRangeSerializer implements PrimitiveSerializer<QueryRange> {
  @override
  final Iterable<Type> types = const [QueryRange, _$QueryRange];

  @override
  final String wireName = r'QueryRange';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    QueryRange object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    if (object.count != null) {
      yield r'count';
      yield serializers.serialize(
        object.count,
        specifiedType: const FullType.nullable(int),
      );
    }
    yield r'lat';
    yield serializers.serialize(
      object.lat,
      specifiedType: const FullType(double),
    );
    yield r'lng';
    yield serializers.serialize(
      object.lng,
      specifiedType: const FullType(double),
    );
    yield r'range';
    yield serializers.serialize(
      object.range,
      specifiedType: const FullType(int),
    );
    if (object.sorted != null) {
      yield r'sorted';
      yield serializers.serialize(
        object.sorted,
        specifiedType: const FullType.nullable(bool),
      );
    }
  }

  @override
  Object serialize(
    Serializers serializers,
    QueryRange object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required QueryRangeBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'count':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType.nullable(int),
          ) as int?;
          if (valueDes == null) continue;
          result.count = valueDes;
          break;
        case r'lat':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(double),
          ) as double;
          result.lat = valueDes;
          break;
        case r'lng':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(double),
          ) as double;
          result.lng = valueDes;
          break;
        case r'range':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(int),
          ) as int;
          result.range = valueDes;
          break;
        case r'sorted':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType.nullable(bool),
          ) as bool?;
          if (valueDes == null) continue;
          result.sorted = valueDes;
          break;
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  QueryRange deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = QueryRangeBuilder();
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

