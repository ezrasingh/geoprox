//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'remove_key_batch_response.g.dart';

/// Returns batch key deletion status
///
/// Properties:
/// * [deleted] - If true all keys were removed
@BuiltValue()
abstract class RemoveKeyBatchResponse implements Built<RemoveKeyBatchResponse, RemoveKeyBatchResponseBuilder> {
  /// If true all keys were removed
  @BuiltValueField(wireName: r'deleted')
  bool get deleted;

  RemoveKeyBatchResponse._();

  factory RemoveKeyBatchResponse([void updates(RemoveKeyBatchResponseBuilder b)]) = _$RemoveKeyBatchResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(RemoveKeyBatchResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<RemoveKeyBatchResponse> get serializer => _$RemoveKeyBatchResponseSerializer();
}

class _$RemoveKeyBatchResponseSerializer implements PrimitiveSerializer<RemoveKeyBatchResponse> {
  @override
  final Iterable<Type> types = const [RemoveKeyBatchResponse, _$RemoveKeyBatchResponse];

  @override
  final String wireName = r'RemoveKeyBatchResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    RemoveKeyBatchResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'deleted';
    yield serializers.serialize(
      object.deleted,
      specifiedType: const FullType(bool),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    RemoveKeyBatchResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required RemoveKeyBatchResponseBuilder result,
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
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  RemoveKeyBatchResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = RemoveKeyBatchResponseBuilder();
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

