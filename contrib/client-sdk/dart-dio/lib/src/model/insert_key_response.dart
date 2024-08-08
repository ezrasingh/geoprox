//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'insert_key_response.g.dart';

/// Returns key and geohash
///
/// Properties:
/// * [geohash] - Geohash encoded latitude/longitude
/// * [key] - Object key
@BuiltValue()
abstract class InsertKeyResponse implements Built<InsertKeyResponse, InsertKeyResponseBuilder> {
  /// Geohash encoded latitude/longitude
  @BuiltValueField(wireName: r'geohash')
  String get geohash;

  /// Object key
  @BuiltValueField(wireName: r'key')
  String get key;

  InsertKeyResponse._();

  factory InsertKeyResponse([void updates(InsertKeyResponseBuilder b)]) = _$InsertKeyResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(InsertKeyResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<InsertKeyResponse> get serializer => _$InsertKeyResponseSerializer();
}

class _$InsertKeyResponseSerializer implements PrimitiveSerializer<InsertKeyResponse> {
  @override
  final Iterable<Type> types = const [InsertKeyResponse, _$InsertKeyResponse];

  @override
  final String wireName = r'InsertKeyResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    InsertKeyResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'geohash';
    yield serializers.serialize(
      object.geohash,
      specifiedType: const FullType(String),
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
    InsertKeyResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required InsertKeyResponseBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'geohash':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(String),
          ) as String;
          result.geohash = valueDes;
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
  InsertKeyResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = InsertKeyResponseBuilder();
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

