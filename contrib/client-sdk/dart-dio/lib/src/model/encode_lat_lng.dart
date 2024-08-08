//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'encode_lat_lng.g.dart';

/// Arguments for encoding latitude/longitude as geohash
///
/// Properties:
/// * [depth] - Determines geohash length
/// * [lat] - Latitude
/// * [lng] - Longitude
@BuiltValue()
abstract class EncodeLatLng implements Built<EncodeLatLng, EncodeLatLngBuilder> {
  /// Determines geohash length
  @BuiltValueField(wireName: r'depth')
  int get depth;

  /// Latitude
  @BuiltValueField(wireName: r'lat')
  double get lat;

  /// Longitude
  @BuiltValueField(wireName: r'lng')
  double get lng;

  EncodeLatLng._();

  factory EncodeLatLng([void updates(EncodeLatLngBuilder b)]) = _$EncodeLatLng;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(EncodeLatLngBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<EncodeLatLng> get serializer => _$EncodeLatLngSerializer();
}

class _$EncodeLatLngSerializer implements PrimitiveSerializer<EncodeLatLng> {
  @override
  final Iterable<Type> types = const [EncodeLatLng, _$EncodeLatLng];

  @override
  final String wireName = r'EncodeLatLng';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    EncodeLatLng object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'depth';
    yield serializers.serialize(
      object.depth,
      specifiedType: const FullType(int),
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
    EncodeLatLng object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required EncodeLatLngBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'depth':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(int),
          ) as int;
          result.depth = valueDes;
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
  EncodeLatLng deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = EncodeLatLngBuilder();
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

