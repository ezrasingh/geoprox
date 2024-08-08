//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;

class DecodeGeohashResponse {
  /// Returns a new [DecodeGeohashResponse] instance.
  DecodeGeohashResponse({
    required this.lat,
    required this.latError,
    required this.lng,
    required this.lngError,
  });

  /// Latitude
  double lat;

  /// Latitude error
  double latError;

  /// Longitude
  double lng;

  /// Longitude error
  double lngError;

  @override
  bool operator ==(Object other) => identical(this, other) || other is DecodeGeohashResponse &&
    other.lat == lat &&
    other.latError == latError &&
    other.lng == lng &&
    other.lngError == lngError;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (lat.hashCode) +
    (latError.hashCode) +
    (lng.hashCode) +
    (lngError.hashCode);

  @override
  String toString() => 'DecodeGeohashResponse[lat=$lat, latError=$latError, lng=$lng, lngError=$lngError]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'lat'] = this.lat;
      json[r'lat_error'] = this.latError;
      json[r'lng'] = this.lng;
      json[r'lng_error'] = this.lngError;
    return json;
  }

  /// Returns a new [DecodeGeohashResponse] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static DecodeGeohashResponse? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "DecodeGeohashResponse[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "DecodeGeohashResponse[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return DecodeGeohashResponse(
        lat: mapValueOfType<double>(json, r'lat')!,
        latError: mapValueOfType<double>(json, r'lat_error')!,
        lng: mapValueOfType<double>(json, r'lng')!,
        lngError: mapValueOfType<double>(json, r'lng_error')!,
      );
    }
    return null;
  }

  static List<DecodeGeohashResponse> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <DecodeGeohashResponse>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = DecodeGeohashResponse.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, DecodeGeohashResponse> mapFromJson(dynamic json) {
    final map = <String, DecodeGeohashResponse>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = DecodeGeohashResponse.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of DecodeGeohashResponse-objects as value to a dart map
  static Map<String, List<DecodeGeohashResponse>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<DecodeGeohashResponse>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = DecodeGeohashResponse.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'lat',
    'lat_error',
    'lng',
    'lng_error',
  };
}

