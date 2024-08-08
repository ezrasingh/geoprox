//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;

class DropIndexResponse {
  /// Returns a new [DropIndexResponse] instance.
  DropIndexResponse({
    required this.deleted,
  });

  /// If true index was deleted
  bool deleted;

  @override
  bool operator ==(Object other) => identical(this, other) || other is DropIndexResponse &&
    other.deleted == deleted;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (deleted.hashCode);

  @override
  String toString() => 'DropIndexResponse[deleted=$deleted]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'deleted'] = this.deleted;
    return json;
  }

  /// Returns a new [DropIndexResponse] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static DropIndexResponse? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "DropIndexResponse[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "DropIndexResponse[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return DropIndexResponse(
        deleted: mapValueOfType<bool>(json, r'deleted')!,
      );
    }
    return null;
  }

  static List<DropIndexResponse> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <DropIndexResponse>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = DropIndexResponse.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, DropIndexResponse> mapFromJson(dynamic json) {
    final map = <String, DropIndexResponse>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = DropIndexResponse.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of DropIndexResponse-objects as value to a dart map
  static Map<String, List<DropIndexResponse>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<DropIndexResponse>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = DropIndexResponse.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'deleted',
  };
}

