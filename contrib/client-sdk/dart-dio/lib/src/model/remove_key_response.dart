//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'remove_key_response.g.dart';

/// Returns key and deletion status
///
/// Properties:
/// * [deleted] - If true key was removed
/// * [key] - Object key
@BuiltValue()
abstract class RemoveKeyResponse implements Built<RemoveKeyResponse, RemoveKeyResponseBuilder> {
  /// If true key was removed
  @BuiltValueField(wireName: r'deleted')
  bool get deleted;

  /// Object key
  @BuiltValueField(wireName: r'key')
  String get key;

  RemoveKeyResponse._();

  factory RemoveKeyResponse([void updates(RemoveKeyResponseBuilder b)]) = _$RemoveKeyResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(RemoveKeyResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<RemoveKeyResponse> get serializer => _$RemoveKeyResponseSerializer();
}

class _$RemoveKeyResponseSerializer implements PrimitiveSerializer<RemoveKeyResponse> {
  @override
  final Iterable<Type> types = const [RemoveKeyResponse, _$RemoveKeyResponse];

  @override
  final String wireName = r'RemoveKeyResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    RemoveKeyResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'deleted';
    yield serializers.serialize(
      object.deleted,
      specifiedType: const FullType(bool),
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
    RemoveKeyResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required RemoveKeyResponseBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'deleted':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(bool),
          ) as bool;
          result.deleted = valueDes;
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
  RemoveKeyResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = RemoveKeyResponseBuilder();
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

