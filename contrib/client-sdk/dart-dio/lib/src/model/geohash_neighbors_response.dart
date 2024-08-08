//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'geohash_neighbors_response.g.dart';

/// Neighboring geohash regions
///
/// Properties:
/// * [e] - East
/// * [n] - North
/// * [ne] - North East
/// * [nw] - North West
/// * [s] - South
/// * [se] - South East
/// * [sw] - South West
/// * [w] - West
@BuiltValue()
abstract class GeohashNeighborsResponse implements Built<GeohashNeighborsResponse, GeohashNeighborsResponseBuilder> {
  /// East
  @BuiltValueField(wireName: r'e')
  String get e;

  /// North
  @BuiltValueField(wireName: r'n')
  String get n;

  /// North East
  @BuiltValueField(wireName: r'ne')
  String get ne;

  /// North West
  @BuiltValueField(wireName: r'nw')
  String get nw;

  /// South
  @BuiltValueField(wireName: r's')
  String get s;

  /// South East
  @BuiltValueField(wireName: r'se')
  String get se;

  /// South West
  @BuiltValueField(wireName: r'sw')
  String get sw;

  /// West
  @BuiltValueField(wireName: r'w')
  String get w;

  GeohashNeighborsResponse._();

  factory GeohashNeighborsResponse([void updates(GeohashNeighborsResponseBuilder b)]) = _$GeohashNeighborsResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(GeohashNeighborsResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<GeohashNeighborsResponse> get serializer => _$GeohashNeighborsResponseSerializer();
}

class _$GeohashNeighborsResponseSerializer implements PrimitiveSerializer<GeohashNeighborsResponse> {
  @override
  final Iterable<Type> types = const [GeohashNeighborsResponse, _$GeohashNeighborsResponse];

  @override
  final String wireName = r'GeohashNeighborsResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    GeohashNeighborsResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'e';
    yield serializers.serialize(
      object.e,
      specifiedType: const FullType(String),
    );
    yield r'n';
    yield serializers.serialize(
      object.n,
      specifiedType: const FullType(String),
    );
    yield r'ne';
    yield serializers.serialize(
      object.ne,
      specifiedType: const FullType(String),
    );
    yield r'nw';
    yield serializers.serialize(
      object.nw,
      specifiedType: const FullType(String),
    );
    yield r's';
    yield serializers.serialize(
      object.s,
      specifiedType: const FullType(String),
    );
    yield r'se';
    yield serializers.serialize(
      object.se,
      specifiedType: const FullType(String),
    );
    yield r'sw';
    yield serializers.serialize(
      object.sw,
      specifiedType: const FullType(String),
    );
    yield r'w';
    yield serializers.serialize(
      object.w,
      specifiedType: const FullType(String),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    GeohashNeighborsResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required GeohashNeighborsResponseBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'e':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.e = valueDes;
          break;
        case r'n':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.n = valueDes;
          break;
        case r'ne':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.ne = valueDes;
          break;
        case r'nw':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.nw = valueDes;
          break;
        case r's':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.s = valueDes;
          break;
        case r'se':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.se = valueDes;
          break;
        case r'sw':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.sw = valueDes;
          break;
        case r'w':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.w = valueDes;
          break;
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  GeohashNeighborsResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = GeohashNeighborsResponseBuilder();
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

