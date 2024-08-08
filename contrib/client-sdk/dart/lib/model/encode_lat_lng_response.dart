//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;

class EncodeLatLngResponse {
  /// Returns a new [EncodeLatLngResponse] instance.
  EncodeLatLngResponse({
    required this.geohash,
  });

  String geohash;

  @override
  bool operator ==(Object other) => identical(this, other) || other is EncodeLatLngResponse &&
    other.geohash == geohash;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (geohash.hashCode);

  @override
  String toString() => 'EncodeLatLngResponse[geohash=$geohash]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'geohash'] = this.geohash;
    return json;
  }

  /// Returns a new [EncodeLatLngResponse] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static EncodeLatLngResponse? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "EncodeLatLngResponse[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "EncodeLatLngResponse[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return EncodeLatLngResponse(
        geohash: mapValueOfType<String>(json, r'geohash')!,
      );
    }
    return null;
  }

  static List<EncodeLatLngResponse> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <EncodeLatLngResponse>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = EncodeLatLngResponse.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, EncodeLatLngResponse> mapFromJson(dynamic json) {
    final map = <String, EncodeLatLngResponse>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = EncodeLatLngResponse.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of EncodeLatLngResponse-objects as value to a dart map
  static Map<String, List<EncodeLatLngResponse>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<EncodeLatLngResponse>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = EncodeLatLngResponse.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'geohash',
  };
}

