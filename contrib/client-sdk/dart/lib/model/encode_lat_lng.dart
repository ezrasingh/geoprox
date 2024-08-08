//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;

class EncodeLatLng {
  /// Returns a new [EncodeLatLng] instance.
  EncodeLatLng({
    required this.depth,
    required this.lat,
    required this.lng,
  });

  /// Determines geohash length
  ///
  /// Minimum value: 1
  /// Maximum value: 10
  int depth;

  /// Latitude
  double lat;

  /// Longitude
  double lng;

  @override
  bool operator ==(Object other) => identical(this, other) || other is EncodeLatLng &&
    other.depth == depth &&
    other.lat == lat &&
    other.lng == lng;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (depth.hashCode) +
    (lat.hashCode) +
    (lng.hashCode);

  @override
  String toString() => 'EncodeLatLng[depth=$depth, lat=$lat, lng=$lng]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'depth'] = this.depth;
      json[r'lat'] = this.lat;
      json[r'lng'] = this.lng;
    return json;
  }

  /// Returns a new [EncodeLatLng] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static EncodeLatLng? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "EncodeLatLng[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "EncodeLatLng[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return EncodeLatLng(
        depth: mapValueOfType<int>(json, r'depth')!,
        lat: mapValueOfType<double>(json, r'lat')!,
        lng: mapValueOfType<double>(json, r'lng')!,
      );
    }
    return null;
  }

  static List<EncodeLatLng> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <EncodeLatLng>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = EncodeLatLng.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, EncodeLatLng> mapFromJson(dynamic json) {
    final map = <String, EncodeLatLng>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = EncodeLatLng.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of EncodeLatLng-objects as value to a dart map
  static Map<String, List<EncodeLatLng>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<EncodeLatLng>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = EncodeLatLng.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'depth',
    'lat',
    'lng',
  };
}

