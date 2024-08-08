//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//

// ignore_for_file: unused_element
import 'package:built_value/built_value.dart';
import 'package:built_value/serializer.dart';

part 'create_index_response.g.dart';

/// Returns index creation status
///
/// Properties:
/// * [created] - If true index was created
/// * [existed] - If true index alredy exist
@BuiltValue()
abstract class CreateIndexResponse implements Built<CreateIndexResponse, CreateIndexResponseBuilder> {
  /// If true index was created
  @BuiltValueField(wireName: r'created')
  bool get created;

  /// If true index alredy exist
  @BuiltValueField(wireName: r'existed')
  bool get existed;

  CreateIndexResponse._();

  factory CreateIndexResponse([void updates(CreateIndexResponseBuilder b)]) = _$CreateIndexResponse;

  @BuiltValueHook(initializeBuilder: true)
  static void _defaults(CreateIndexResponseBuilder b) => b;

  @BuiltValueSerializer(custom: true)
  static Serializer<CreateIndexResponse> get serializer => _$CreateIndexResponseSerializer();
}

class _$CreateIndexResponseSerializer implements PrimitiveSerializer<CreateIndexResponse> {
  @override
  final Iterable<Type> types = const [CreateIndexResponse, _$CreateIndexResponse];

  @override
  final String wireName = r'CreateIndexResponse';

  Iterable<Object?> _serializeProperties(
    Serializers serializers,
    CreateIndexResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) sync* {
    yield r'created';
    yield serializers.serialize(
      object.created,
      specifiedType: const FullType(bool),
    );
    yield r'existed';
    yield serializers.serialize(
      object.existed,
      specifiedType: const FullType(bool),
    );
  }

  @override
  Object serialize(
    Serializers serializers,
    CreateIndexResponse object, {
    FullType specifiedType = FullType.unspecified,
  }) {
    return _serializeProperties(serializers, object, specifiedType: specifiedType).toList();
  }

  void _deserializeProperties(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
    required List<Object?> serializedList,
    required CreateIndexResponseBuilder result,
    required List<Object?> unhandled,
  }) {
    for (var i = 0; i < serializedList.length; i += 2) {
      final key = serializedList[i] as String;
      final value = serializedList[i + 1];
      switch (key) {
        case r'created':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(bool),
          ) as bool;
          result.created = valueDes;
          break;
        case r'existed':
          final valueDes = serializers.deserialize(
            value,
            specifiedType: const FullType(bool),
          ) as bool;
          result.existed = valueDes;
          break;
        default:
          unhandled.add(key);
          unhandled.add(value);
          break;
      }
    }
  }

  @override
  CreateIndexResponse deserialize(
    Serializers serializers,
    Object serialized, {
    FullType specifiedType = FullType.unspecified,
  }) {
    final result = CreateIndexResponseBuilder();
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

