//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'insert_key.g.dart';

/// Arguments for inserting a key
///
/// Properties:
/// * [key] - Object key
/// * [lat] - Latitude
/// * [lng] - Longitude
@BuiltValue()
abstract class InsertKey implements Built<InsertKey, InsertKeyBuilder> {
  /// Object key
  @BuiltValueField(wireName: r'key')
  String get key;

  /// Latitude
  @BuiltValueField(wireName: r'lat')
  double get lat;

  /// Longitude
  @BuiltValueField(wireName: r'lng')
  double get lng;

  InsertKey._();

  factory InsertKey([void updates(InsertKeyBuilder b)]) = _$InsertKey;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(InsertKeyBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<InsertKey> get serializer => _$InsertKeySerializer();
}

class _$InsertKeySerializer implements PrimitiveSerializer<InsertKey> {
  @override
  final Iterable<Type> types = const [InsertKey, _$InsertKey];

  @override
  final String wireName = r'InsertKey';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    InsertKey object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'key';
    yield serializers.serialize(
      object.key,
      specifiedType: const FullType(String),
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
  }

  @override
  Object serialize(
    Serializers serializers,
    InsertKey object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required InsertKeyBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'key':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.key = valueDes;
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
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  InsertKey deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = InsertKeyBuilder();
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

