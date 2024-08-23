//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;

class InsertKey {
  /// Returns a new [InsertKey] instance.
  InsertKey({
    required this.key,
    required this.lat,
    required this.lng,
    this.ttl,
  });

  /// Object key
  String key;

  /// Latitude
  double lat;

  /// Longitude
  double lng;

  /// The time-to-live (TTL) for this key, in seconds
  ///
  /// Minimum value: 0
  int? ttl;

  @override
  bool operator ==(Object other) => identical(this, other) || other is InsertKey &&
    other.key == key &&
    other.lat == lat &&
    other.lng == lng &&
    other.ttl == ttl;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (key.hashCode) +
    (lat.hashCode) +
    (lng.hashCode) +
    (ttl == null ? 0 : ttl!.hashCode);

  @override
  String toString() => 'InsertKey[key=$key, lat=$lat, lng=$lng, ttl=$ttl]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'key'] = this.key;
      json[r'lat'] = this.lat;
      json[r'lng'] = this.lng;
    if (this.ttl != null) {
      json[r'ttl'] = this.ttl;
    } else {
      json[r'ttl'] = null;
    }
    return json;
  }

  /// Returns a new [InsertKey] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static InsertKey? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "InsertKey[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "InsertKey[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return InsertKey(
        key: mapValueOfType<String>(json, r'key')!,
        lat: mapValueOfType<double>(json, r'lat')!,
        lng: mapValueOfType<double>(json, r'lng')!,
        ttl: mapValueOfType<int>(json, r'ttl'),
      );
    }
    return null;
  }

  static List<InsertKey> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <InsertKey>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = InsertKey.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, InsertKey> mapFromJson(dynamic json) {
    final map = <String, InsertKey>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = InsertKey.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of InsertKey-objects as value to a dart map
  static Map<String, List<InsertKey>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<InsertKey>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = InsertKey.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'key',
    'lat',
    'lng',
  };
}

