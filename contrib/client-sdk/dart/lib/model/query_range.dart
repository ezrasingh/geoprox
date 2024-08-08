//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;

class QueryRange {
  /// Returns a new [QueryRange] instance.
  QueryRange({
    this.count,
    required this.lat,
    required this.lng,
    required this.range,
    this.sorted,
  });

  /// Maximum number of neighbors that can be returned (default 100)
  ///
  /// Minimum value: 1
  /// Maximum value: 65535
  int? count;

  /// Latitude
  double lat;

  /// Longitude
  double lng;

  /// Search radius in kilometers
  ///
  /// Minimum value: 0
  /// Maximum value: 65535
  int range;

  /// If enabled neighbors will be sorted by distance, nearest to furthest (default false)
  bool? sorted;

  @override
  bool operator ==(Object other) => identical(this, other) || other is QueryRange &&
    other.count == count &&
    other.lat == lat &&
    other.lng == lng &&
    other.range == range &&
    other.sorted == sorted;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (count == null ? 0 : count!.hashCode) +
    (lat.hashCode) +
    (lng.hashCode) +
    (range.hashCode) +
    (sorted == null ? 0 : sorted!.hashCode);

  @override
  String toString() => 'QueryRange[count=$count, lat=$lat, lng=$lng, range=$range, sorted=$sorted]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
    if (this.count != null) {
      json[r'count'] = this.count;
    } else {
      json[r'count'] = null;
    }
      json[r'lat'] = this.lat;
      json[r'lng'] = this.lng;
      json[r'range'] = this.range;
    if (this.sorted != null) {
      json[r'sorted'] = this.sorted;
    } else {
      json[r'sorted'] = null;
    }
    return json;
  }

  /// Returns a new [QueryRange] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static QueryRange? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "QueryRange[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "QueryRange[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return QueryRange(
        count: mapValueOfType<int>(json, r'count'),
        lat: mapValueOfType<double>(json, r'lat')!,
        lng: mapValueOfType<double>(json, r'lng')!,
        range: mapValueOfType<int>(json, r'range')!,
        sorted: mapValueOfType<bool>(json, r'sorted'),
      );
    }
    return null;
  }

  static List<QueryRange> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <QueryRange>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = QueryRange.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, QueryRange> mapFromJson(dynamic json) {
    final map = <String, QueryRange>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = QueryRange.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of QueryRange-objects as value to a dart map
  static Map<String, List<QueryRange>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<QueryRange>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = QueryRange.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'lat',
    'lng',
    'range',
  };
}

