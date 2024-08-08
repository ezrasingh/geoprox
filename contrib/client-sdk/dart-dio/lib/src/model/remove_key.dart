//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'remove_key.g.dart';

/// Arguments for removing a key
///
/// Properties:
/// * [key] - Object key
@BuiltValue()
abstract class RemoveKey implements Built<RemoveKey, RemoveKeyBuilder> {
  /// Object key
  @BuiltValueField(wireName: r'key')
  String get key;

  RemoveKey._();

  factory RemoveKey([void updates(RemoveKeyBuilder b)]) = _$RemoveKey;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(RemoveKeyBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<RemoveKey> get serializer => _$RemoveKeySerializer();
}

class _$RemoveKeySerializer implements PrimitiveSerializer<RemoveKey> {
  @override
  final Iterable<Type> types = const [RemoveKey, _$RemoveKey];

  @override
  final String wireName = r'RemoveKey';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    RemoveKey object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'key';
    yield serializers.serialize(
      object.key,
      specifiedType: const FullType(String),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    RemoveKey object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required RemoveKeyBuilder result,
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
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  RemoveKey deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = RemoveKeyBuilder();
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

