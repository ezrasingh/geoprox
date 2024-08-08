//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_collection/built_collection.dart';
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'query_range_many.g.dart';

/// Arguments for group range query
///
/// Properties:
/// * [count] - Maximum number of neighbors that can be returned (default 100)
/// * [indices] - List of indices to search
/// * [lat] - Latitude
/// * [lng] - Longitude
/// * [range] - Search radius in kilometers
/// * [sorted] - If enabled neighbors will be sorted by distance, nearest to furthest (default false)
@BuiltValue()
abstract class QueryRangeMany implements Built<QueryRangeMany, QueryRangeManyBuilder> {
  /// Maximum number of neighbors that can be returned (default 100)
  @BuiltValueField(wireName: r'count')
  int? get count;

  /// List of indices to search
  @BuiltValueField(wireName: r'indices')
  BuiltList<String> get indices;

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

  QueryRangeMany._();

  factory QueryRangeMany([void updates(QueryRangeManyBuilder b)]) = _$QueryRangeMany;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(QueryRangeManyBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<QueryRangeMany> get serializer => _$QueryRangeManySerializer();
}

class _$QueryRangeManySerializer implements PrimitiveSerializer<QueryRangeMany> {
  @override
  final Iterable<Type> types = const [QueryRangeMany, _$QueryRangeMany];

  @override
  final String wireName = r'QueryRangeMany';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    QueryRangeMany object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    if (object.count != null) {
      yield r'count';
      yield serializers.serialize(
        object.count,
        specifiedType: const FullType.nullable(int),
      );
    }
    yield r'indices';
    yield serializers.serialize(
      object.indices,
      specifiedType: const FullType(BuiltList, [FullType(String)]),
    );
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
    QueryRangeMany object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required QueryRangeManyBuilder result,
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
        case r'indices':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(BuiltList, [FullType(String)]),
          ) as BuiltList<String>;
          result.indices.replace(valueDes);
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
  QueryRangeMany deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = QueryRangeManyBuilder();
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

