//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;

class QueryRangeManyResponse {
  /// Returns a new [QueryRangeManyResponse] instance.
  QueryRangeManyResponse({
    this.errors = const {},
    this.results = const {},
  });

  /// Contains information about any errors occured during batch search.
  Map<String, String> errors;

  /// Object keys found within range
  Map<String, List<Neighbor>> results;

  @override
  bool operator ==(Object other) => identical(this, other) || other is QueryRangeManyResponse &&
    _deepEquality.equals(other.errors, errors) &&
    _deepEquality.equals(other.results, results);

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (errors.hashCode) +
    (results.hashCode);

  @override
  String toString() => 'QueryRangeManyResponse[errors=$errors, results=$results]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'errors'] = this.errors;
      json[r'results'] = this.results;
    return json;
  }

  /// Returns a new [QueryRangeManyResponse] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static QueryRangeManyResponse? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "QueryRangeManyResponse[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "QueryRangeManyResponse[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return QueryRangeManyResponse(
        errors: mapCastOfType<String, String>(json, r'errors')!,
        results: json[r'results'] == null
          ? const {}
            : Neighbor.mapListFromJson(json[r'results']),
      );
    }
    return null;
  }

  static List<QueryRangeManyResponse> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <QueryRangeManyResponse>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = QueryRangeManyResponse.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, QueryRangeManyResponse> mapFromJson(dynamic json) {
    final map = <String, QueryRangeManyResponse>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = QueryRangeManyResponse.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of QueryRangeManyResponse-objects as value to a dart map
  static Map<String, List<QueryRangeManyResponse>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<QueryRangeManyResponse>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = QueryRangeManyResponse.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'errors',
    'results',
  };
}

