//
// AUTO-GENERATED FILE, DO NOT MODIFY!
//
// @dart=2.18

// ignore_for_file: unused_element, unused_import
// ignore_for_file: always_put_required_named_parameters_first
// ignore_for_file: constant_identifier_names
// ignore_for_file: lines_longer_than_80_chars

part of geoprox_client.api;

class InsertKeyBatch {
  /// Returns a new [InsertKeyBatch] instance.
  InsertKeyBatch({
    this.keys = const [],
    required this.preserveOrder,
    this.ttl,
  });

  /// Object key
  List<InsertKey> keys;

  bool preserveOrder;

  /// The time-to-live (TTL) for these keys, in seconds
  ///
  /// Minimum value: 0
  int? ttl;

  @override
  bool operator ==(Object other) => identical(this, other) || other is InsertKeyBatch &&
    _deepEquality.equals(other.keys, keys) &&
    other.preserveOrder == preserveOrder &&
    other.ttl == ttl;

  @override
  int get hashCode =>
    // ignore: unnecessary_parenthesis
    (keys.hashCode) +
    (preserveOrder.hashCode) +
    (ttl == null ? 0 : ttl!.hashCode);

  @override
  String toString() => 'InsertKeyBatch[keys=$keys, preserveOrder=$preserveOrder, ttl=$ttl]';

  Map<String, dynamic> toJson() {
    final json = <String, dynamic>{};
      json[r'keys'] = this.keys;
      json[r'preserve_order'] = this.preserveOrder;
    if (this.ttl != null) {
      json[r'ttl'] = this.ttl;
    } else {
      json[r'ttl'] = null;
    }
    return json;
  }

  /// Returns a new [InsertKeyBatch] instance and imports its values from
  /// [value] if it's a [Map], null otherwise.
  // ignore: prefer_constructors_over_static_methods
  static InsertKeyBatch? fromJson(dynamic value) {
    if (value is Map) {
      final json = value.cast<String, dynamic>();

      // Ensure that the map contains the required keys.
      // Note 1: the values aren't checked for validity beyond being non-null.
      // Note 2: this code is stripped in release mode!
      assert(() {
        requiredKeys.forEach((key) {
          assert(json.containsKey(key), 'Required key "InsertKeyBatch[$key]" is missing from JSON.');
          assert(json[key] != null, 'Required key "InsertKeyBatch[$key]" has a null value in JSON.');
        });
        return true;
      }());

      return InsertKeyBatch(
        keys: InsertKey.listFromJson(json[r'keys']),
        preserveOrder: mapValueOfType<bool>(json, r'preserve_order')!,
        ttl: mapValueOfType<int>(json, r'ttl'),
      );
    }
    return null;
  }

  static List<InsertKeyBatch> listFromJson(dynamic json, {bool growable = false,}) {
    final result = <InsertKeyBatch>[];
    if (json is List && json.isNotEmpty) {
      for (final row in json) {
        final value = InsertKeyBatch.fromJson(row);
        if (value != null) {
          result.add(value);
        }
      }
    }
    return result.toList(growable: growable);
  }

  static Map<String, InsertKeyBatch> mapFromJson(dynamic json) {
    final map = <String, InsertKeyBatch>{};
    if (json is Map && json.isNotEmpty) {
      json = json.cast<String, dynamic>(); // ignore: parameter_assignments
      for (final entry in json.entries) {
        final value = InsertKeyBatch.fromJson(entry.value);
        if (value != null) {
          map[entry.key] = value;
        }
      }
    }
    return map;
  }

  // maps a json object with a list of InsertKeyBatch-objects as value to a dart map
  static Map<String, List<InsertKeyBatch>> mapListFromJson(dynamic json, {bool growable = false,}) {
    final map = <String, List<InsertKeyBatch>>{};
    if (json is Map && json.isNotEmpty) {
      // ignore: parameter_assignments
      json = json.cast<String, dynamic>();
      for (final entry in json.entries) {
        map[entry.key] = InsertKeyBatch.listFromJson(entry.value, growable: growable,);
      }
    }
    return map;
  }

  /// The list of required keys that must be present in a JSON.
  static const requiredKeys = <String>{
    'keys',
    'preserve_order',
  };
}

