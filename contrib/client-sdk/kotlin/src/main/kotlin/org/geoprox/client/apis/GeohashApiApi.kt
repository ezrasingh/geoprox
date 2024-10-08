/**
 *
 * Please note:
 * This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * Do not edit this file manually.
 *
 */

@file:Suppress(
    "ArrayInDataClass",
    "EnumEntryName",
    "RemoveRedundantQualifierName",
    "UnusedImport"
)

package org.geoprox.client.apis

import java.io.IOException
import okhttp3.OkHttpClient
import okhttp3.HttpUrl

import org.geoprox.client.models.DecodeGeohashResponse
import org.geoprox.client.models.EncodeLatLngResponse
import org.geoprox.client.models.GeohashNeighborsResponse

import com.squareup.moshi.Json

import org.geoprox.client.infrastructure.ApiClient
import org.geoprox.client.infrastructure.ApiResponse
import org.geoprox.client.infrastructure.ClientException
import org.geoprox.client.infrastructure.ClientError
import org.geoprox.client.infrastructure.ServerException
import org.geoprox.client.infrastructure.ServerError
import org.geoprox.client.infrastructure.MultiValueMap
import org.geoprox.client.infrastructure.PartConfig
import org.geoprox.client.infrastructure.RequestConfig
import org.geoprox.client.infrastructure.RequestMethod
import org.geoprox.client.infrastructure.ResponseType
import org.geoprox.client.infrastructure.Success
import org.geoprox.client.infrastructure.toMultiValue

class GeohashApiApi(basePath: kotlin.String = defaultBasePath, client: OkHttpClient = ApiClient.defaultClient) : ApiClient(basePath, client) {
    companion object {
        @JvmStatic
        val defaultBasePath: String by lazy {
            System.getProperties().getProperty(ApiClient.baseUrlKey, "http://localhost")
        }
    }

    /**
     * Decode geohash into coordinates.
     * Decode geohash by path param, returns coordinates with precision estimates.
     * @param ghash Geohash encoded region
     * @return DecodeGeohashResponse
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     * @throws UnsupportedOperationException If the API returns an informational or redirection response
     * @throws ClientException If the API returns a client error response
     * @throws ServerException If the API returns a server error response
     */
    @Suppress("UNCHECKED_CAST")
    @Throws(IllegalStateException::class, IOException::class, UnsupportedOperationException::class, ClientException::class, ServerException::class)
    fun decodeGeohash(ghash: kotlin.String) : DecodeGeohashResponse {
        val localVarResponse = decodeGeohashWithHttpInfo(ghash = ghash)

        return when (localVarResponse.responseType) {
            ResponseType.Success -> (localVarResponse as Success<*>).data as DecodeGeohashResponse
            ResponseType.Informational -> throw UnsupportedOperationException("Client does not support Informational responses.")
            ResponseType.Redirection -> throw UnsupportedOperationException("Client does not support Redirection responses.")
            ResponseType.ClientError -> {
                val localVarError = localVarResponse as ClientError<*>
                throw ClientException("Client error : ${localVarError.statusCode} ${localVarError.message.orEmpty()}", localVarError.statusCode, localVarResponse)
            }
            ResponseType.ServerError -> {
                val localVarError = localVarResponse as ServerError<*>
                throw ServerException("Server error : ${localVarError.statusCode} ${localVarError.message.orEmpty()} ${localVarError.body}", localVarError.statusCode, localVarResponse)
            }
        }
    }

    /**
     * Decode geohash into coordinates.
     * Decode geohash by path param, returns coordinates with precision estimates.
     * @param ghash Geohash encoded region
     * @return ApiResponse<DecodeGeohashResponse?>
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     */
    @Suppress("UNCHECKED_CAST")
    @Throws(IllegalStateException::class, IOException::class)
    fun decodeGeohashWithHttpInfo(ghash: kotlin.String) : ApiResponse<DecodeGeohashResponse?> {
        val localVariableConfig = decodeGeohashRequestConfig(ghash = ghash)

        return request<Unit, DecodeGeohashResponse>(
            localVariableConfig
        )
    }

    /**
     * To obtain the request config of the operation decodeGeohash
     *
     * @param ghash Geohash encoded region
     * @return RequestConfig
     */
    fun decodeGeohashRequestConfig(ghash: kotlin.String) : RequestConfig<Unit> {
        val localVariableBody = null
        val localVariableQuery: MultiValueMap = mutableMapOf()
        val localVariableHeaders: MutableMap<String, String> = mutableMapOf()
        localVariableHeaders["Accept"] = "application/json"

        return RequestConfig(
            method = RequestMethod.GET,
            path = "/api/v1/geohash/{ghash}".replace("{"+"ghash"+"}", encodeURIComponent(ghash.toString())),
            query = localVariableQuery,
            headers = localVariableHeaders,
            requiresAuthentication = false,
            body = localVariableBody
        )
    }

    /**
     * Encode coordinates into geohash
     * Encode coordinates by query params, returns geohash.
     * @param lat Latitude
     * @param lng Longitude
     * @param depth Determines geohash length
     * @return EncodeLatLngResponse
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     * @throws UnsupportedOperationException If the API returns an informational or redirection response
     * @throws ClientException If the API returns a client error response
     * @throws ServerException If the API returns a server error response
     */
    @Suppress("UNCHECKED_CAST")
    @Throws(IllegalStateException::class, IOException::class, UnsupportedOperationException::class, ClientException::class, ServerException::class)
    fun encodeLatlng(lat: kotlin.Double, lng: kotlin.Double, depth: kotlin.Int) : EncodeLatLngResponse {
        val localVarResponse = encodeLatlngWithHttpInfo(lat = lat, lng = lng, depth = depth)

        return when (localVarResponse.responseType) {
            ResponseType.Success -> (localVarResponse as Success<*>).data as EncodeLatLngResponse
            ResponseType.Informational -> throw UnsupportedOperationException("Client does not support Informational responses.")
            ResponseType.Redirection -> throw UnsupportedOperationException("Client does not support Redirection responses.")
            ResponseType.ClientError -> {
                val localVarError = localVarResponse as ClientError<*>
                throw ClientException("Client error : ${localVarError.statusCode} ${localVarError.message.orEmpty()}", localVarError.statusCode, localVarResponse)
            }
            ResponseType.ServerError -> {
                val localVarError = localVarResponse as ServerError<*>
                throw ServerException("Server error : ${localVarError.statusCode} ${localVarError.message.orEmpty()} ${localVarError.body}", localVarError.statusCode, localVarResponse)
            }
        }
    }

    /**
     * Encode coordinates into geohash
     * Encode coordinates by query params, returns geohash.
     * @param lat Latitude
     * @param lng Longitude
     * @param depth Determines geohash length
     * @return ApiResponse<EncodeLatLngResponse?>
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     */
    @Suppress("UNCHECKED_CAST")
    @Throws(IllegalStateException::class, IOException::class)
    fun encodeLatlngWithHttpInfo(lat: kotlin.Double, lng: kotlin.Double, depth: kotlin.Int) : ApiResponse<EncodeLatLngResponse?> {
        val localVariableConfig = encodeLatlngRequestConfig(lat = lat, lng = lng, depth = depth)

        return request<Unit, EncodeLatLngResponse>(
            localVariableConfig
        )
    }

    /**
     * To obtain the request config of the operation encodeLatlng
     *
     * @param lat Latitude
     * @param lng Longitude
     * @param depth Determines geohash length
     * @return RequestConfig
     */
    fun encodeLatlngRequestConfig(lat: kotlin.Double, lng: kotlin.Double, depth: kotlin.Int) : RequestConfig<Unit> {
        val localVariableBody = null
        val localVariableQuery: MultiValueMap = mutableMapOf<kotlin.String, kotlin.collections.List<kotlin.String>>()
            .apply {
                put("lat", listOf(lat.toString()))
                put("lng", listOf(lng.toString()))
                put("depth", listOf(depth.toString()))
            }
        val localVariableHeaders: MutableMap<String, String> = mutableMapOf()
        localVariableHeaders["Accept"] = "application/json"

        return RequestConfig(
            method = RequestMethod.GET,
            path = "/api/v1/geohash",
            query = localVariableQuery,
            headers = localVariableHeaders,
            requiresAuthentication = false,
            body = localVariableBody
        )
    }

    /**
     * Neighboring regions
     * Returns geohash neighbors in all cardinal directions.
     * @param ghash Geohash encoded region
     * @return GeohashNeighborsResponse
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     * @throws UnsupportedOperationException If the API returns an informational or redirection response
     * @throws ClientException If the API returns a client error response
     * @throws ServerException If the API returns a server error response
     */
    @Suppress("UNCHECKED_CAST")
    @Throws(IllegalStateException::class, IOException::class, UnsupportedOperationException::class, ClientException::class, ServerException::class)
    fun getNeighbors(ghash: kotlin.String) : GeohashNeighborsResponse {
        val localVarResponse = getNeighborsWithHttpInfo(ghash = ghash)

        return when (localVarResponse.responseType) {
            ResponseType.Success -> (localVarResponse as Success<*>).data as GeohashNeighborsResponse
            ResponseType.Informational -> throw UnsupportedOperationException("Client does not support Informational responses.")
            ResponseType.Redirection -> throw UnsupportedOperationException("Client does not support Redirection responses.")
            ResponseType.ClientError -> {
                val localVarError = localVarResponse as ClientError<*>
                throw ClientException("Client error : ${localVarError.statusCode} ${localVarError.message.orEmpty()}", localVarError.statusCode, localVarResponse)
            }
            ResponseType.ServerError -> {
                val localVarError = localVarResponse as ServerError<*>
                throw ServerException("Server error : ${localVarError.statusCode} ${localVarError.message.orEmpty()} ${localVarError.body}", localVarError.statusCode, localVarResponse)
            }
        }
    }

    /**
     * Neighboring regions
     * Returns geohash neighbors in all cardinal directions.
     * @param ghash Geohash encoded region
     * @return ApiResponse<GeohashNeighborsResponse?>
     * @throws IllegalStateException If the request is not correctly configured
     * @throws IOException Rethrows the OkHttp execute method exception
     */
    @Suppress("UNCHECKED_CAST")
    @Throws(IllegalStateException::class, IOException::class)
    fun getNeighborsWithHttpInfo(ghash: kotlin.String) : ApiResponse<GeohashNeighborsResponse?> {
        val localVariableConfig = getNeighborsRequestConfig(ghash = ghash)

        return request<Unit, GeohashNeighborsResponse>(
            localVariableConfig
        )
    }

    /**
     * To obtain the request config of the operation getNeighbors
     *
     * @param ghash Geohash encoded region
     * @return RequestConfig
     */
    fun getNeighborsRequestConfig(ghash: kotlin.String) : RequestConfig<Unit> {
        val localVariableBody = null
        val localVariableQuery: MultiValueMap = mutableMapOf()
        val localVariableHeaders: MutableMap<String, String> = mutableMapOf()
        localVariableHeaders["Accept"] = "application/json"

        return RequestConfig(
            method = RequestMethod.GET,
            path = "/api/v1/geohash/{ghash}/neighbors".replace("{"+"ghash"+"}", encodeURIComponent(ghash.toString())),
            query = localVariableQuery,
            headers = localVariableHeaders,
            requiresAuthentication = false,
            body = localVariableBody
        )
    }


    private fun encodeURIComponent(uriComponent: kotlin.String): kotlin.String =
        HttpUrl.Builder().scheme("http").host("localhost").addPathSegment(uriComponent).build().encodedPathSegments[0]
}
