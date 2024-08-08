//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;

class InsertKeyResponse {
  /// Returns a new [InsertKeyResponse] instance.
  InsertKeyResponse({
    required this.geohash,
    required this.key,
  });

  /// Geohash encoded latitude/longitude
  String geohash;

  /// Object key
  String key;

  @override
  bool operator ==(Object other) => identical(this, other) || other is InsertKeyResponse &&
    other.geohash == geohash &&
    other.key == key;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (geohash.hashCode) +
    (key.hashCode);

  @override
  String toString() => 'InsertKeyResponse[geohash=$geohash, key=$key]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'geohash'] = this.geohash;
      json[r'key'] = this.key;
    return json;
  }

  /// Returns a new [InsertKeyResponse] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static InsertKeyResponse? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "InsertKeyResponse[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "InsertKeyResponse[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return InsertKeyResponse(
        geohash: mapValueOfType<String>(json, r'geohash')!,
        key: mapValueOfType<String>(json, r'key')!,
      );
    }
    return null;
  }

  static List<InsertKeyResponse> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <InsertKeyResponse>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = InsertKeyResponse.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, InsertKeyResponse> mapFromJson(dynamic json) {
    final map = <String, InsertKeyResponse>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = InsertKeyResponse.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of InsertKeyResponse-objects as value to a dart map
  static Map<String, List<InsertKeyResponse>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<InsertKeyResponse>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = InsertKeyResponse.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'geohash',
    'key',
  };
}

