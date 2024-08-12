package org.geoprox.groovy.client;

import org.geoprox.groovy.client.ApiUtils
import org.geoprox.groovy.client.model.CreateIndexResponse
import org.geoprox.groovy.client.model.DropIndexResponse
import org.geoprox.groovy.client.model.InsertKey
import org.geoprox.groovy.client.model.InsertKeyBatch
import org.geoprox.groovy.client.model.InsertKeyBatchResponse
import org.geoprox.groovy.client.model.InsertKeyResponse
import org.geoprox.groovy.client.model.QueryRangeManyResponse
import org.geoprox.groovy.client.model.QueryRangeResponse
import org.geoprox.groovy.client.model.RemoveKey
import org.geoprox.groovy.client.model.RemoveKeyBatch
import org.geoprox.groovy.client.model.RemoveKeyBatchResponse
import org.geoprox.groovy.client.model.RemoveKeyResponse

class GeoshardApiApi {
    String basePath = "http://localhost"
    String versionPath = ""
    ApiUtils apiUtils = new ApiUtils();

    def createIndex ( String index, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/shard/${index}"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (index == null) {
            throw new RuntimeException("missing required params index")
        }





        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "POST", "",
                    CreateIndexResponse.class )

    }

    def dropIndex ( String index, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/shard/${index}"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (index == null) {
            throw new RuntimeException("missing required params index")
        }





        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "DELETE", "",
                    DropIndexResponse.class )

    }

    def insertKey ( String index, InsertKey insertKey, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/shard/${index}"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (index == null) {
            throw new RuntimeException("missing required params index")
        }
        // verify required params are set
        if (insertKey == null) {
            throw new RuntimeException("missing required params insertKey")
        }



        contentType = 'application/json';
        bodyParams = insertKey


        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "PUT", "",
                    InsertKeyResponse.class )

    }

    def insertKeyBatch ( String index, InsertKeyBatch insertKeyBatch, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/shard/${index}/batch"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (index == null) {
            throw new RuntimeException("missing required params index")
        }
        // verify required params are set
        if (insertKeyBatch == null) {
            throw new RuntimeException("missing required params insertKeyBatch")
        }



        contentType = 'application/json';
        bodyParams = insertKeyBatch


        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "PUT", "",
                    InsertKeyBatchResponse.class )

    }

    def queryRange ( String index, Double lat, Double lng, Integer range, Integer count, Boolean sorted, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/shard/${index}"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (index == null) {
            throw new RuntimeException("missing required params index")
        }
        // verify required params are set
        if (lat == null) {
            throw new RuntimeException("missing required params lat")
        }
        // verify required params are set
        if (lng == null) {
            throw new RuntimeException("missing required params lng")
        }
        // verify required params are set
        if (range == null) {
            throw new RuntimeException("missing required params range")
        }

        if (lat != null) {
            queryParams.put("lat", lat)
        }
        if (lng != null) {
            queryParams.put("lng", lng)
        }
        if (range != null) {
            queryParams.put("range", range)
        }
        if (count != null) {
            queryParams.put("count", count)
        }
        if (sorted != null) {
            queryParams.put("sorted", sorted)
        }




        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "GET", "",
                    QueryRangeResponse.class )

    }

    def queryRangeMany ( List<String> indices, Double lat, Double lng, Integer range, Integer count, Boolean sorted, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/shard"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (indices == null) {
            throw new RuntimeException("missing required params indices")
        }
        // verify required params are set
        if (lat == null) {
            throw new RuntimeException("missing required params lat")
        }
        // verify required params are set
        if (lng == null) {
            throw new RuntimeException("missing required params lng")
        }
        // verify required params are set
        if (range == null) {
            throw new RuntimeException("missing required params range")
        }

        if (indices != null) {
            queryParams.put("indices", indices)
        }
        if (lat != null) {
            queryParams.put("lat", lat)
        }
        if (lng != null) {
            queryParams.put("lng", lng)
        }
        if (range != null) {
            queryParams.put("range", range)
        }
        if (count != null) {
            queryParams.put("count", count)
        }
        if (sorted != null) {
            queryParams.put("sorted", sorted)
        }




        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "GET", "",
                    QueryRangeManyResponse.class )

    }

    def removeKey ( String index, RemoveKey removeKey, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/shard/${index}"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (index == null) {
            throw new RuntimeException("missing required params index")
        }
        // verify required params are set
        if (removeKey == null) {
            throw new RuntimeException("missing required params removeKey")
        }



        contentType = 'application/json';
        bodyParams = removeKey


        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "PATCH", "",
                    RemoveKeyResponse.class )

    }

    def removeKeyBatch ( String index, RemoveKeyBatch removeKeyBatch, Closure onSuccess, Closure onFailure)  {
        String resourcePath = "/api/v1/shard/${index}/batch"

        // params
        def queryParams = [:]
        def headerParams = [:]
        def bodyParams
        def contentType

        // verify required params are set
        if (index == null) {
            throw new RuntimeException("missing required params index")
        }
        // verify required params are set
        if (removeKeyBatch == null) {
            throw new RuntimeException("missing required params removeKeyBatch")
        }



        contentType = 'application/json';
        bodyParams = removeKeyBatch


        apiUtils.invokeApi(onSuccess, onFailure, basePath, versionPath, resourcePath, queryParams, headerParams, bodyParams, contentType,
                    "PATCH", "",
                    RemoveKeyBatchResponse.class )

    }

}
