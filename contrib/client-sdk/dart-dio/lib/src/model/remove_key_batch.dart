//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_collection/built_collection.dart';
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'remove_key_batch.g.dart';

/// Arguments for removing multiple keys
///
/// Properties:
/// * [keys] - Object key
@BuiltValue()
abstract class RemoveKeyBatch implements Built<RemoveKeyBatch, RemoveKeyBatchBuilder> {
  /// Object key
  @BuiltValueField(wireName: r'keys')
  BuiltList<String> get keys;

  RemoveKeyBatch._();

  factory RemoveKeyBatch([void updates(RemoveKeyBatchBuilder b)]) = _$RemoveKeyBatch;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(RemoveKeyBatchBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<RemoveKeyBatch> get serializer => _$RemoveKeyBatchSerializer();
}

class _$RemoveKeyBatchSerializer implements PrimitiveSerializer<RemoveKeyBatch> {
  @override
  final Iterable<Type> types = const [RemoveKeyBatch, _$RemoveKeyBatch];

  @override
  final String wireName = r'RemoveKeyBatch';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    RemoveKeyBatch object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'keys';
    yield serializers.serialize(
      object.keys,
      specifiedType: const FullType(BuiltList, [FullType(String)]),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    RemoveKeyBatch object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required RemoveKeyBatchBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'keys':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(BuiltList, [FullType(String)]),
          ) as BuiltList<String>;
          result.keys.replace(valueDes);
          break;
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  RemoveKeyBatch deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = RemoveKeyBatchBuilder();
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

