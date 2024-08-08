//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'decode_geohash_response.g.dart';

/// Returns geohash decoded as latitude/longitude with precision errors
///
/// Properties:
/// * [lat] - Latitude
/// * [latError] - Latitude error
/// * [lng] - Longitude
/// * [lngError] - Longitude error
@BuiltValue()
abstract class DecodeGeohashResponse implements Built<DecodeGeohashResponse, DecodeGeohashResponseBuilder> {
  /// Latitude
  @BuiltValueField(wireName: r'lat')
  double get lat;

  /// Latitude error
  @BuiltValueField(wireName: r'lat_error')
  double get latError;

  /// Longitude
  @BuiltValueField(wireName: r'lng')
  double get lng;

  /// Longitude error
  @BuiltValueField(wireName: r'lng_error')
  double get lngError;

  DecodeGeohashResponse._();

  factory DecodeGeohashResponse([void updates(DecodeGeohashResponseBuilder b)]) = _$DecodeGeohashResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(DecodeGeohashResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<DecodeGeohashResponse> get serializer => _$DecodeGeohashResponseSerializer();
}

class _$DecodeGeohashResponseSerializer implements PrimitiveSerializer<DecodeGeohashResponse> {
  @override
  final Iterable<Type> types = const [DecodeGeohashResponse, _$DecodeGeohashResponse];

  @override
  final String wireName = r'DecodeGeohashResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    DecodeGeohashResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'lat';
    yield serializers.serialize(
      object.lat,
      specifiedType: const FullType(double),
    );
    yield r'lat_error';
    yield serializers.serialize(
      object.latError,
      specifiedType: const FullType(double),
    );
    yield r'lng';
    yield serializers.serialize(
      object.lng,
      specifiedType: const FullType(double),
    );
    yield r'lng_error';
    yield serializers.serialize(
      object.lngError,
      specifiedType: const FullType(double),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    DecodeGeohashResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required DecodeGeohashResponseBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'lat':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(double),
          ) as double;
          result.lat = valueDes;
          break;
        case r'lat_error':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(double),
          ) as double;
          result.latError = valueDes;
          break;
        case r'lng':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(double),
          ) as double;
          result.lng = valueDes;
          break;
        case r'lng_error':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(double),
          ) as double;
          result.lngError = valueDes;
          break;
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  DecodeGeohashResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = DecodeGeohashResponseBuilder();
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

