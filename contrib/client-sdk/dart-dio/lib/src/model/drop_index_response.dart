//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'drop_index_response.g.dart';

/// Returns index deletion status
///
/// Properties:
/// * [deleted] - If true index was deleted
@BuiltValue()
abstract class DropIndexResponse implements Built<DropIndexResponse, DropIndexResponseBuilder> {
  /// If true index was deleted
  @BuiltValueField(wireName: r'deleted')
  bool get deleted;

  DropIndexResponse._();

  factory DropIndexResponse([void updates(DropIndexResponseBuilder b)]) = _$DropIndexResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(DropIndexResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<DropIndexResponse> get serializer => _$DropIndexResponseSerializer();
}

class _$DropIndexResponseSerializer implements PrimitiveSerializer<DropIndexResponse> {
  @override
  final Iterable<Type> types = const [DropIndexResponse, _$DropIndexResponse];

  @override
  final String wireName = r'DropIndexResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    DropIndexResponse object, {
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
    DropIndexResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required DropIndexResponseBuilder result,
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
  DropIndexResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = DropIndexResponseBuilder();
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

