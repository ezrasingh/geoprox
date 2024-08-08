import 'package:test/test.dart';
import 'package:geoprox_client_dio/geoprox_client_dio.dart';


/// tests for GeohashApiApi
void main() {
  final instance = GeoproxClientDio().getGeohashApiApi();

  group(GeohashApiApi, () {
    // Decode geohash into coordinates.
    //
    // Decode geohash by path param, returns coordinates with precision estimates.
    //
    //Future<DecodeGeohashResponse> decodeGeohash(String ghash) async
    test('test decodeGeohash', () async {
      // TODO
    });

    // Encode coordinates into geohash
    //
    // Encode coordinates by query params, returns geohash.
    //
    //Future<EncodeLatLngResponse> encodeLatlng(double lat, double lng, int depth) async
    test('test encodeLatlng', () async {
      // TODO
    });

    // Neighboring regions
    //
    // Returns geohash neighbors in all cardinal directions.
    //
    //Future<GeohashNeighborsResponse> getNeighbors(String ghash) async
    test('test getNeighbors', () async {
      // TODO
    });

  });
}
