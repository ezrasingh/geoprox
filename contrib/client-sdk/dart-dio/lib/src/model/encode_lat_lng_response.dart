//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'encode_lat_lng_response.g.dart';

/// Returns geohash encoded latitude/longitude
///
/// Properties:
/// * [geohash] 
@BuiltValue()
abstract class EncodeLatLngResponse implements Built<EncodeLatLngResponse, EncodeLatLngResponseBuilder> {
  @BuiltValueField(wireName: r'geohash')
  String get geohash;

  EncodeLatLngResponse._();

  factory EncodeLatLngResponse([void updates(EncodeLatLngResponseBuilder b)]) = _$EncodeLatLngResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(EncodeLatLngResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<EncodeLatLngResponse> get serializer => _$EncodeLatLngResponseSerializer();
}

class _$EncodeLatLngResponseSerializer implements PrimitiveSerializer<EncodeLatLngResponse> {
  @override
  final Iterable<Type> types = const [EncodeLatLngResponse, _$EncodeLatLngResponse];

  @override
  final String wireName = r'EncodeLatLngResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    EncodeLatLngResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'geohash';
    yield serializers.serialize(
      object.geohash,
      specifiedType: const FullType(String),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    EncodeLatLngResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required EncodeLatLngResponseBuilder result,
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
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  EncodeLatLngResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = EncodeLatLngResponseBuilder();
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

