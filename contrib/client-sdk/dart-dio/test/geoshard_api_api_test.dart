import 'package:test/test.dart';
import 'package:geoprox_client_dio/geoprox_client_dio.dart';


/// tests for GeoshardApiApi
void main() {
  final instance = GeoproxClientDio().getGeoshardApiApi();

  group(GeoshardApiApi, () {
    // Create geospatial index
    //
    // Creates an in-memory index within this geoshard
    //
    //Future<CreateIndexResponse> createIndex(String index) async
    test('test createIndex', () async {
      // TODO
    });

    // Deletes geospatial index
    //
    // Drop index. All keys will be lost
    //
    //Future<DropIndexResponse> dropIndex(String index) async
    test('test dropIndex', () async {
      // TODO
    });

    // Insert key into index
    //
    // Inserts key into geospatial index
    //
    //Future<InsertKeyResponse> insertKey(String index, InsertKey insertKey) async
    test('test insertKey', () async {
      // TODO
    });

    // Insert multiple keys into index
    //
    // Inserts multiple keys into geospatial index
    //
    //Future<InsertKeyBatchResponse> insertKeyBatch(String index, InsertKeyBatch insertKeyBatch) async
    test('test insertKeyBatch', () async {
      // TODO
    });

    // Search index for objects nearby
    //
    // Search geospatial index for all keys within some distance
    //
    //Future<QueryRangeResponse> queryRange(String index, double lat, double lng, int range, { int count, bool sorted }) async
    test('test queryRange', () async {
      // TODO
    });

    // Search multiple indices for objects nearby
    //
    // Search geospatial many indices for all keys within some distance
    //
    //Future<QueryRangeManyResponse> queryRangeMany(BuiltList<String> indices, double lat, double lng, int range, { int count, bool sorted }) async
    test('test queryRangeMany', () async {
      // TODO
    });

    // Remove key from index
    //
    // Removes key from geospatial index
    //
    //Future<RemoveKeyResponse> removeKey(String index, RemoveKey removeKey) async
    test('test removeKey', () async {
      // TODO
    });

    // Remove multiple keys from index
    //
    // Removes multiple keys from geospatial index
    //
    //Future<RemoveKeyBatchResponse> removeKeyBatch(String index, RemoveKeyBatch removeKeyBatch) async
    test('test removeKeyBatch', () async {
      // TODO
    });

  });
}
