package org.geoprox.groovy.client;

import org.geoprox.groovy.client.ApiUtils
import org.geoprox.groovy.client.model.DecodeGeohashResponse
import org.geoprox.groovy.client.model.EncodeLatLngResponse
import org.geoprox.groovy.client.model.GeohashNeighborsResponse

class GeohashApiApi {
    String basePath = "http://localhost"
    String versionPath = ""
    ApiUtils apiUtils = new ApiUtils();

    def decodeGeohash ( String ghash, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/geohash/${ghash}"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (ghash == null) {
            throw new RuntimeException("missing required params ghash")
        }





        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "GET", "",
                    DecodeGeohashResponse.class )

    }

    def encodeLatlng ( Double lat, Double lng, Integer depth, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/geohash"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (lat == null) {
            throw new RuntimeException("missing required params lat")
        }
        // verify required params are set
        if (lng == null) {
            throw new RuntimeException("missing required params lng")
        }
        // verify required params are set
        if (depth == null) {
            throw new RuntimeException("missing required params depth")
        }

        if (lat != null) {
            queryParams.put("lat", lat)
        }
        if (lng != null) {
            queryParams.put("lng", lng)
        }
        if (depth != null) {
            queryParams.put("depth", depth)
        }




        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "GET", "",
                    EncodeLatLngResponse.class )

    }

    def getNeighbors ( String ghash, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/geohash/${ghash}/neighbors"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (ghash == null) {
            throw new RuntimeException("missing required params ghash")
        }





        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "GET", "",
                    GeohashNeighborsResponse.class )

    }

}
