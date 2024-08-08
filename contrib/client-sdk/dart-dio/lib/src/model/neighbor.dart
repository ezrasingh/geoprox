//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'neighbor.g.dart';

/// Nearby object
///
/// Properties:
/// * [distance] - Distance in kilometers
/// * [key] - Object key
@BuiltValue()
abstract class Neighbor implements Built<Neighbor, NeighborBuilder> {
  /// Distance in kilometers
  @BuiltValueField(wireName: r'distance')
  double get distance;

  /// Object key
  @BuiltValueField(wireName: r'key')
  String get key;

  Neighbor._();

  factory Neighbor([void updates(NeighborBuilder b)]) = _$Neighbor;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(NeighborBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<Neighbor> get serializer => _$NeighborSerializer();
}

class _$NeighborSerializer implements PrimitiveSerializer<Neighbor> {
  @override
  final Iterable<Type> types = const [Neighbor, _$Neighbor];

  @override
  final String wireName = r'Neighbor';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    Neighbor object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'distance';
    yield serializers.serialize(
      object.distance,
      specifiedType: const FullType(double),
    );
    yield r'key';
    yield serializers.serialize(
      object.key,
      specifiedType: const FullType(String),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    Neighbor object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required NeighborBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'distance':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(double),
          ) as double;
          result.distance = valueDes;
          break;
        case r'key':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.key = valueDes;
          break;
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  Neighbor deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = NeighborBuilder();
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

