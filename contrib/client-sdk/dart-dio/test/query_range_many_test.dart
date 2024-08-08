import 'package:test/test.dart';
import 'package:geoprox_client_dio/geoprox_client_dio.dart';

// tests for QueryRangeMany
void main() {
  final instance = QueryRangeManyBuilder();
  // TODO add properties to the builder and call build()

  group(QueryRangeMany, () {
    // Maximum number of neighbors that can be returned (default 100)
    // int count
    test('to test the property `count`', () async {
      // TODO
    });

    // List of indices to search
    // BuiltList<String> indices
    test('to test the property `indices`', () async {
      // TODO
    });

    // Latitude
    // double lat
    test('to test the property `lat`', () async {
      // TODO
    });

    // Longitude
    // double lng
    test('to test the property `lng`', () async {
      // TODO
    });

    // Search radius in kilometers
    // int range
    test('to test the property `range`', () async {
      // TODO
    });

    // If enabled neighbors will be sorted by distance, nearest to furthest (default false)
    // bool sorted
    test('to test the property `sorted`', () async {
      // TODO
    });

  });
}
