import 'package:test/test.dart';
import 'package:geoprox_client_dio/geoprox_client_dio.dart';

// tests for InsertKeyBatchResponse
void main() {
  final instance = InsertKeyBatchResponseBuilder();
  // TODO add properties to the builder and call build()

  group(InsertKeyBatchResponse, () {
    // Contains information about which keys failed to be inserted and the associated error details.
    // BuiltMap<String, String> errors
    test('to test the property `errors`', () async {
      // TODO
    });

    // Object keys that have been inserted in the index and their geohash.
    // BuiltMap<String, String> results
    test('to test the property `results`', () async {
      // TODO
    });

  });
}
