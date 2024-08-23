//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:geoprox_client_dio/src/model/insert_key.dart';
import 'package:built_collection/built_collection.dart';
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'insert_key_batch.g.dart';

/// Arguments for inserting multiple keys
///
/// Properties:
/// * [keys] - Object key
/// * [preserveOrder] 
/// * [ttl] - The time-to-live (TTL) for these keys, in seconds
@BuiltValue()
abstract class InsertKeyBatch implements Built<InsertKeyBatch, InsertKeyBatchBuilder> {
  /// Object key
  @BuiltValueField(wireName: r'keys')
  BuiltList<InsertKey> get keys;

  @BuiltValueField(wireName: r'preserve_order')
  bool get preserveOrder;

  /// The time-to-live (TTL) for these keys, in seconds
  @BuiltValueField(wireName: r'ttl')
  int? get ttl;

  InsertKeyBatch._();

  factory InsertKeyBatch([void updates(InsertKeyBatchBuilder b)]) = _$InsertKeyBatch;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(InsertKeyBatchBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<InsertKeyBatch> get serializer => _$InsertKeyBatchSerializer();
}

class _$InsertKeyBatchSerializer implements PrimitiveSerializer<InsertKeyBatch> {
  @override
  final Iterable<Type> types = const [InsertKeyBatch, _$InsertKeyBatch];

  @override
  final String wireName = r'InsertKeyBatch';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    InsertKeyBatch object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'keys';
    yield serializers.serialize(
      object.keys,
      specifiedType: const FullType(BuiltList, [FullType(InsertKey)]),
    );
    yield r'preserve_order';
    yield serializers.serialize(
      object.preserveOrder,
      specifiedType: const FullType(bool),
    );
    if (object.ttl != null) {
      yield r'ttl';
      yield serializers.serialize(
        object.ttl,
        specifiedType: const FullType.nullable(int),
      );
    }
  }

  @override
  Object serialize(
    Serializers serializers,
    InsertKeyBatch object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required InsertKeyBatchBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'keys':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(BuiltList, [FullType(InsertKey)]),
          ) as BuiltList<InsertKey>;
          result.keys.replace(valueDes);
          break;
        case r'preserve_order':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(bool),
          ) as bool;
          result.preserveOrder = valueDes;
          break;
        case r'ttl':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType.nullable(int),
          ) as int?;
          if (valueDes == null) continue;
          result.ttl = valueDes;
          break;
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  InsertKeyBatch deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = InsertKeyBatchBuilder();
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

