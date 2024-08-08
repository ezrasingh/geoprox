//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;

class CreateIndexResponse {
  /// Returns a new [CreateIndexResponse] instance.
  CreateIndexResponse({
    required this.created,
    required this.existed,
  });

  /// If true index was created
  bool created;

  /// If true index alredy exist
  bool existed;

  @override
  bool operator ==(Object other) => identical(this, other) || other is CreateIndexResponse &&
    other.created == created &&
    other.existed == existed;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (created.hashCode) +
    (existed.hashCode);

  @override
  String toString() => 'CreateIndexResponse[created=$created, existed=$existed]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'created'] = this.created;
      json[r'existed'] = this.existed;
    return json;
  }

  /// Returns a new [CreateIndexResponse] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static CreateIndexResponse? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "CreateIndexResponse[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "CreateIndexResponse[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return CreateIndexResponse(
        created: mapValueOfType<bool>(json, r'created')!,
        existed: mapValueOfType<bool>(json, r'existed')!,
      );
    }
    return null;
  }

  static List<CreateIndexResponse> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <CreateIndexResponse>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = CreateIndexResponse.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, CreateIndexResponse> mapFromJson(dynamic json) {
    final map = <String, CreateIndexResponse>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = CreateIndexResponse.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of CreateIndexResponse-objects as value to a dart map
  static Map<String, List<CreateIndexResponse>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<CreateIndexResponse>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = CreateIndexResponse.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'created',
    'existed',
  };
}

