//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_collection/built_collection.dart';
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'insert_key_batch_response.g.dart';

/// Returns results and errors of batch key insert
///
/// Properties:
/// * [errors] - Contains information about which keys failed to be inserted and the associated error details.
/// * [results] - Object keys that have been inserted in the index and their geohash.
@BuiltValue()
abstract class InsertKeyBatchResponse implements Built<InsertKeyBatchResponse, InsertKeyBatchResponseBuilder> {
  /// Contains information about which keys failed to be inserted and the associated error details.
  @BuiltValueField(wireName: r'errors')
  BuiltMap<String, String> get errors;

  /// Object keys that have been inserted in the index and their geohash.
  @BuiltValueField(wireName: r'results')
  BuiltMap<String, String> get results;

  InsertKeyBatchResponse._();

  factory InsertKeyBatchResponse([void updates(InsertKeyBatchResponseBuilder b)]) = _$InsertKeyBatchResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(InsertKeyBatchResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<InsertKeyBatchResponse> get serializer => _$InsertKeyBatchResponseSerializer();
}

class _$InsertKeyBatchResponseSerializer implements PrimitiveSerializer<InsertKeyBatchResponse> {
  @override
  final Iterable<Type> types = const [InsertKeyBatchResponse, _$InsertKeyBatchResponse];

  @override
  final String wireName = r'InsertKeyBatchResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    InsertKeyBatchResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'errors';
    yield serializers.serialize(
      object.errors,
      specifiedType: const FullType(BuiltMap, [FullType(String), FullType(String)]),
    );
    yield r'results';
    yield serializers.serialize(
      object.results,
      specifiedType: const FullType(BuiltMap, [FullType(String), FullType(String)]),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    InsertKeyBatchResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required InsertKeyBatchResponseBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'errors':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(BuiltMap, [FullType(String), FullType(String)]),
          ) as BuiltMap<String, String>;
          result.errors.replace(valueDes);
          break;
        case r'results':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(BuiltMap, [FullType(String), FullType(String)]),
          ) as BuiltMap<String, String>;
          result.results.replace(valueDes);
          break;
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  InsertKeyBatchResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = InsertKeyBatchResponseBuilder();
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

