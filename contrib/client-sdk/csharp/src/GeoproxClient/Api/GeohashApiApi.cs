/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.1
 * Contact: singhezra@gmail.com
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */


using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Net;
using System.Net.Mime;
using GeoproxClient.Client;
using GeoproxClient.Model;

namespace GeoproxClient.Api
{

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface IGeohashApiApiSync : IApiAccessor
    {
        #region Synchronous Operations
        /// <summary>
        /// Decode geohash into coordinates.
        /// </summary>
        /// <remarks>
        /// Decode geohash by path param, returns coordinates with precision estimates.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>DecodeGeohashResponse</returns>
        DecodeGeohashResponse DecodeGeohash(string ghash, int operationIndex = 0);

        /// <summary>
        /// Decode geohash into coordinates.
        /// </summary>
        /// <remarks>
        /// Decode geohash by path param, returns coordinates with precision estimates.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>ApiResponse of DecodeGeohashResponse</returns>
        ApiResponse<DecodeGeohashResponse> DecodeGeohashWithHttpInfo(string ghash, int operationIndex = 0);
        /// <summary>
        /// Encode coordinates into geohash
        /// </summary>
        /// <remarks>
        /// Encode coordinates by query params, returns geohash.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="lat">Latitude</param>
        /// <param name="lng">Longitude</param>
        /// <param name="depth">Determines geohash length</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>EncodeLatLngResponse</returns>
        EncodeLatLngResponse EncodeLatlng(double lat, double lng, int depth, int operationIndex = 0);

        /// <summary>
        /// Encode coordinates into geohash
        /// </summary>
        /// <remarks>
        /// Encode coordinates by query params, returns geohash.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="lat">Latitude</param>
        /// <param name="lng">Longitude</param>
        /// <param name="depth">Determines geohash length</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>ApiResponse of EncodeLatLngResponse</returns>
        ApiResponse<EncodeLatLngResponse> EncodeLatlngWithHttpInfo(double lat, double lng, int depth, int operationIndex = 0);
        /// <summary>
        /// Neighboring regions
        /// </summary>
        /// <remarks>
        /// Returns geohash neighbors in all cardinal directions.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>GeohashNeighborsResponse</returns>
        GeohashNeighborsResponse GetNeighbors(string ghash, int operationIndex = 0);

        /// <summary>
        /// Neighboring regions
        /// </summary>
        /// <remarks>
        /// Returns geohash neighbors in all cardinal directions.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>ApiResponse of GeohashNeighborsResponse</returns>
        ApiResponse<GeohashNeighborsResponse> GetNeighborsWithHttpInfo(string ghash, int operationIndex = 0);
        #endregion Synchronous Operations
    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface IGeohashApiApiAsync : IApiAccessor
    {
        #region Asynchronous Operations
        /// <summary>
        /// Decode geohash into coordinates.
        /// </summary>
        /// <remarks>
        /// Decode geohash by path param, returns coordinates with precision estimates.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of DecodeGeohashResponse</returns>
        System.Threading.Tasks.Task<DecodeGeohashResponse> DecodeGeohashAsync(string ghash, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken));

        /// <summary>
        /// Decode geohash into coordinates.
        /// </summary>
        /// <remarks>
        /// Decode geohash by path param, returns coordinates with precision estimates.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (DecodeGeohashResponse)</returns>
        System.Threading.Tasks.Task<ApiResponse<DecodeGeohashResponse>> DecodeGeohashWithHttpInfoAsync(string ghash, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken));
        /// <summary>
        /// Encode coordinates into geohash
        /// </summary>
        /// <remarks>
        /// Encode coordinates by query params, returns geohash.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="lat">Latitude</param>
        /// <param name="lng">Longitude</param>
        /// <param name="depth">Determines geohash length</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of EncodeLatLngResponse</returns>
        System.Threading.Tasks.Task<EncodeLatLngResponse> EncodeLatlngAsync(double lat, double lng, int depth, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken));

        /// <summary>
        /// Encode coordinates into geohash
        /// </summary>
        /// <remarks>
        /// Encode coordinates by query params, returns geohash.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="lat">Latitude</param>
        /// <param name="lng">Longitude</param>
        /// <param name="depth">Determines geohash length</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (EncodeLatLngResponse)</returns>
        System.Threading.Tasks.Task<ApiResponse<EncodeLatLngResponse>> EncodeLatlngWithHttpInfoAsync(double lat, double lng, int depth, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken));
        /// <summary>
        /// Neighboring regions
        /// </summary>
        /// <remarks>
        /// Returns geohash neighbors in all cardinal directions.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of GeohashNeighborsResponse</returns>
        System.Threading.Tasks.Task<GeohashNeighborsResponse> GetNeighborsAsync(string ghash, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken));

        /// <summary>
        /// Neighboring regions
        /// </summary>
        /// <remarks>
        /// Returns geohash neighbors in all cardinal directions.
        /// </remarks>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (GeohashNeighborsResponse)</returns>
        System.Threading.Tasks.Task<ApiResponse<GeohashNeighborsResponse>> GetNeighborsWithHttpInfoAsync(string ghash, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken));
        #endregion Asynchronous Operations
    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface IGeohashApiApi : IGeohashApiApiSync, IGeohashApiApiAsync
    {

    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public partial class GeohashApiApi : IGeohashApiApi
    {
        private GeoproxClient.Client.ExceptionFactory _exceptionFactory = (name, response) => null;

        /// <summary>
        /// Initializes a new instance of the <see cref="GeohashApiApi"/> class.
        /// </summary>
        /// <returns></returns>
        public GeohashApiApi() : this((string)null)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="GeohashApiApi"/> class.
        /// </summary>
        /// <returns></returns>
        public GeohashApiApi(string basePath)
        {
            this.Configuration = GeoproxClient.Client.Configuration.MergeConfigurations(
                GeoproxClient.Client.GlobalConfiguration.Instance,
                new GeoproxClient.Client.Configuration { BasePath = basePath }
            );
            this.Client = new GeoproxClient.Client.ApiClient(this.Configuration.BasePath);
            this.AsynchronousClient = new GeoproxClient.Client.ApiClient(this.Configuration.BasePath);
            this.ExceptionFactory = GeoproxClient.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="GeohashApiApi"/> class
        /// using Configuration object
        /// </summary>
        /// <param name="configuration">An instance of Configuration</param>
        /// <returns></returns>
        public GeohashApiApi(GeoproxClient.Client.Configuration configuration)
        {
            if (configuration == null) throw new ArgumentNullException("configuration");

            this.Configuration = GeoproxClient.Client.Configuration.MergeConfigurations(
                GeoproxClient.Client.GlobalConfiguration.Instance,
                configuration
            );
            this.Client = new GeoproxClient.Client.ApiClient(this.Configuration.BasePath);
            this.AsynchronousClient = new GeoproxClient.Client.ApiClient(this.Configuration.BasePath);
            ExceptionFactory = GeoproxClient.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="GeohashApiApi"/> class
        /// using a Configuration object and client instance.
        /// </summary>
        /// <param name="client">The client interface for synchronous API access.</param>
        /// <param name="asyncClient">The client interface for asynchronous API access.</param>
        /// <param name="configuration">The configuration object.</param>
        public GeohashApiApi(GeoproxClient.Client.ISynchronousClient client, GeoproxClient.Client.IAsynchronousClient asyncClient, GeoproxClient.Client.IReadableConfiguration configuration)
        {
            if (client == null) throw new ArgumentNullException("client");
            if (asyncClient == null) throw new ArgumentNullException("asyncClient");
            if (configuration == null) throw new ArgumentNullException("configuration");

            this.Client = client;
            this.AsynchronousClient = asyncClient;
            this.Configuration = configuration;
            this.ExceptionFactory = GeoproxClient.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// The client for accessing this underlying API asynchronously.
        /// </summary>
        public GeoproxClient.Client.IAsynchronousClient AsynchronousClient { get; set; }

        /// <summary>
        /// The client for accessing this underlying API synchronously.
        /// </summary>
        public GeoproxClient.Client.ISynchronousClient Client { get; set; }

        /// <summary>
        /// Gets the base path of the API client.
        /// </summary>
        /// <value>The base path</value>
        public string GetBasePath()
        {
            return this.Configuration.BasePath;
        }

        /// <summary>
        /// Gets or sets the configuration object
        /// </summary>
        /// <value>An instance of the Configuration</value>
        public GeoproxClient.Client.IReadableConfiguration Configuration { get; set; }

        /// <summary>
        /// Provides a factory method hook for the creation of exceptions.
        /// </summary>
        public GeoproxClient.Client.ExceptionFactory ExceptionFactory
        {
            get
            {
                if (_exceptionFactory != null && _exceptionFactory.GetInvocationList().Length > 1)
                {
                    throw new InvalidOperationException("Multicast delegate for ExceptionFactory is unsupported.");
                }
                return _exceptionFactory;
            }
            set { _exceptionFactory = value; }
        }

        /// <summary>
        /// Decode geohash into coordinates. Decode geohash by path param, returns coordinates with precision estimates.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>DecodeGeohashResponse</returns>
        public DecodeGeohashResponse DecodeGeohash(string ghash, int operationIndex = 0)
        {
            GeoproxClient.Client.ApiResponse<DecodeGeohashResponse> localVarResponse = DecodeGeohashWithHttpInfo(ghash);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Decode geohash into coordinates. Decode geohash by path param, returns coordinates with precision estimates.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>ApiResponse of DecodeGeohashResponse</returns>
        public GeoproxClient.Client.ApiResponse<DecodeGeohashResponse> DecodeGeohashWithHttpInfo(string ghash, int operationIndex = 0)
        {
            // verify the required parameter 'ghash' is set
            if (ghash == null)
            {
                throw new GeoproxClient.Client.ApiException(400, "Missing required parameter 'ghash' when calling GeohashApiApi->DecodeGeohash");
            }

            GeoproxClient.Client.RequestOptions localVarRequestOptions = new GeoproxClient.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = GeoproxClient.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);
            }

            var localVarAccept = GeoproxClient.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);
            }

            localVarRequestOptions.PathParameters.Add("ghash", GeoproxClient.Client.ClientUtils.ParameterToString(ghash)); // path parameter

            localVarRequestOptions.Operation = "GeohashApiApi.DecodeGeohash";
            localVarRequestOptions.OperationIndex = operationIndex;


            // make the HTTP request
            var localVarResponse = this.Client.Get<DecodeGeohashResponse>("/api/v1/geohash/{ghash}/", localVarRequestOptions, this.Configuration);
            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("DecodeGeohash", localVarResponse);
                if (_exception != null)
                {
                    throw _exception;
                }
            }

            return localVarResponse;
        }

        /// <summary>
        /// Decode geohash into coordinates. Decode geohash by path param, returns coordinates with precision estimates.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of DecodeGeohashResponse</returns>
        public async System.Threading.Tasks.Task<DecodeGeohashResponse> DecodeGeohashAsync(string ghash, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken))
        {
            GeoproxClient.Client.ApiResponse<DecodeGeohashResponse> localVarResponse = await DecodeGeohashWithHttpInfoAsync(ghash, operationIndex, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Decode geohash into coordinates. Decode geohash by path param, returns coordinates with precision estimates.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (DecodeGeohashResponse)</returns>
        public async System.Threading.Tasks.Task<GeoproxClient.Client.ApiResponse<DecodeGeohashResponse>> DecodeGeohashWithHttpInfoAsync(string ghash, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken))
        {
            // verify the required parameter 'ghash' is set
            if (ghash == null)
            {
                throw new GeoproxClient.Client.ApiException(400, "Missing required parameter 'ghash' when calling GeohashApiApi->DecodeGeohash");
            }


            GeoproxClient.Client.RequestOptions localVarRequestOptions = new GeoproxClient.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = GeoproxClient.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);
            }

            var localVarAccept = GeoproxClient.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);
            }

            localVarRequestOptions.PathParameters.Add("ghash", GeoproxClient.Client.ClientUtils.ParameterToString(ghash)); // path parameter

            localVarRequestOptions.Operation = "GeohashApiApi.DecodeGeohash";
            localVarRequestOptions.OperationIndex = operationIndex;


            // make the HTTP request
            var localVarResponse = await this.AsynchronousClient.GetAsync<DecodeGeohashResponse>("/api/v1/geohash/{ghash}/", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("DecodeGeohash", localVarResponse);
                if (_exception != null)
                {
                    throw _exception;
                }
            }

            return localVarResponse;
        }

        /// <summary>
        /// Encode coordinates into geohash Encode coordinates by query params, returns geohash.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="lat">Latitude</param>
        /// <param name="lng">Longitude</param>
        /// <param name="depth">Determines geohash length</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>EncodeLatLngResponse</returns>
        public EncodeLatLngResponse EncodeLatlng(double lat, double lng, int depth, int operationIndex = 0)
        {
            GeoproxClient.Client.ApiResponse<EncodeLatLngResponse> localVarResponse = EncodeLatlngWithHttpInfo(lat, lng, depth);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Encode coordinates into geohash Encode coordinates by query params, returns geohash.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="lat">Latitude</param>
        /// <param name="lng">Longitude</param>
        /// <param name="depth">Determines geohash length</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>ApiResponse of EncodeLatLngResponse</returns>
        public GeoproxClient.Client.ApiResponse<EncodeLatLngResponse> EncodeLatlngWithHttpInfo(double lat, double lng, int depth, int operationIndex = 0)
        {
            GeoproxClient.Client.RequestOptions localVarRequestOptions = new GeoproxClient.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = GeoproxClient.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);
            }

            var localVarAccept = GeoproxClient.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);
            }

            localVarRequestOptions.QueryParameters.Add(GeoproxClient.Client.ClientUtils.ParameterToMultiMap("", "lat", lat));
            localVarRequestOptions.QueryParameters.Add(GeoproxClient.Client.ClientUtils.ParameterToMultiMap("", "lng", lng));
            localVarRequestOptions.QueryParameters.Add(GeoproxClient.Client.ClientUtils.ParameterToMultiMap("", "depth", depth));

            localVarRequestOptions.Operation = "GeohashApiApi.EncodeLatlng";
            localVarRequestOptions.OperationIndex = operationIndex;


            // make the HTTP request
            var localVarResponse = this.Client.Get<EncodeLatLngResponse>("/api/v1/geohash/", localVarRequestOptions, this.Configuration);
            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("EncodeLatlng", localVarResponse);
                if (_exception != null)
                {
                    throw _exception;
                }
            }

            return localVarResponse;
        }

        /// <summary>
        /// Encode coordinates into geohash Encode coordinates by query params, returns geohash.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="lat">Latitude</param>
        /// <param name="lng">Longitude</param>
        /// <param name="depth">Determines geohash length</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of EncodeLatLngResponse</returns>
        public async System.Threading.Tasks.Task<EncodeLatLngResponse> EncodeLatlngAsync(double lat, double lng, int depth, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken))
        {
            GeoproxClient.Client.ApiResponse<EncodeLatLngResponse> localVarResponse = await EncodeLatlngWithHttpInfoAsync(lat, lng, depth, operationIndex, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Encode coordinates into geohash Encode coordinates by query params, returns geohash.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="lat">Latitude</param>
        /// <param name="lng">Longitude</param>
        /// <param name="depth">Determines geohash length</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (EncodeLatLngResponse)</returns>
        public async System.Threading.Tasks.Task<GeoproxClient.Client.ApiResponse<EncodeLatLngResponse>> EncodeLatlngWithHttpInfoAsync(double lat, double lng, int depth, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken))
        {

            GeoproxClient.Client.RequestOptions localVarRequestOptions = new GeoproxClient.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = GeoproxClient.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);
            }

            var localVarAccept = GeoproxClient.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);
            }

            localVarRequestOptions.QueryParameters.Add(GeoproxClient.Client.ClientUtils.ParameterToMultiMap("", "lat", lat));
            localVarRequestOptions.QueryParameters.Add(GeoproxClient.Client.ClientUtils.ParameterToMultiMap("", "lng", lng));
            localVarRequestOptions.QueryParameters.Add(GeoproxClient.Client.ClientUtils.ParameterToMultiMap("", "depth", depth));

            localVarRequestOptions.Operation = "GeohashApiApi.EncodeLatlng";
            localVarRequestOptions.OperationIndex = operationIndex;


            // make the HTTP request
            var localVarResponse = await this.AsynchronousClient.GetAsync<EncodeLatLngResponse>("/api/v1/geohash/", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("EncodeLatlng", localVarResponse);
                if (_exception != null)
                {
                    throw _exception;
                }
            }

            return localVarResponse;
        }

        /// <summary>
        /// Neighboring regions Returns geohash neighbors in all cardinal directions.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>GeohashNeighborsResponse</returns>
        public GeohashNeighborsResponse GetNeighbors(string ghash, int operationIndex = 0)
        {
            GeoproxClient.Client.ApiResponse<GeohashNeighborsResponse> localVarResponse = GetNeighborsWithHttpInfo(ghash);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Neighboring regions Returns geohash neighbors in all cardinal directions.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <returns>ApiResponse of GeohashNeighborsResponse</returns>
        public GeoproxClient.Client.ApiResponse<GeohashNeighborsResponse> GetNeighborsWithHttpInfo(string ghash, int operationIndex = 0)
        {
            // verify the required parameter 'ghash' is set
            if (ghash == null)
            {
                throw new GeoproxClient.Client.ApiException(400, "Missing required parameter 'ghash' when calling GeohashApiApi->GetNeighbors");
            }

            GeoproxClient.Client.RequestOptions localVarRequestOptions = new GeoproxClient.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = GeoproxClient.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);
            }

            var localVarAccept = GeoproxClient.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);
            }

            localVarRequestOptions.PathParameters.Add("ghash", GeoproxClient.Client.ClientUtils.ParameterToString(ghash)); // path parameter

            localVarRequestOptions.Operation = "GeohashApiApi.GetNeighbors";
            localVarRequestOptions.OperationIndex = operationIndex;


            // make the HTTP request
            var localVarResponse = this.Client.Get<GeohashNeighborsResponse>("/api/v1/geohash/{ghash}/neighbors/", localVarRequestOptions, this.Configuration);
            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("GetNeighbors", localVarResponse);
                if (_exception != null)
                {
                    throw _exception;
                }
            }

            return localVarResponse;
        }

        /// <summary>
        /// Neighboring regions Returns geohash neighbors in all cardinal directions.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of GeohashNeighborsResponse</returns>
        public async System.Threading.Tasks.Task<GeohashNeighborsResponse> GetNeighborsAsync(string ghash, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken))
        {
            GeoproxClient.Client.ApiResponse<GeohashNeighborsResponse> localVarResponse = await GetNeighborsWithHttpInfoAsync(ghash, operationIndex, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Neighboring regions Returns geohash neighbors in all cardinal directions.
        /// </summary>
        /// <exception cref="GeoproxClient.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="ghash">Geohash encoded region</param>
        /// <param name="operationIndex">Index associated with the operation.</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (GeohashNeighborsResponse)</returns>
        public async System.Threading.Tasks.Task<GeoproxClient.Client.ApiResponse<GeohashNeighborsResponse>> GetNeighborsWithHttpInfoAsync(string ghash, int operationIndex = 0, System.Threading.CancellationToken cancellationToken = default(System.Threading.CancellationToken))
        {
            // verify the required parameter 'ghash' is set
            if (ghash == null)
            {
                throw new GeoproxClient.Client.ApiException(400, "Missing required parameter 'ghash' when calling GeohashApiApi->GetNeighbors");
            }


            GeoproxClient.Client.RequestOptions localVarRequestOptions = new GeoproxClient.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = GeoproxClient.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);
            }

            var localVarAccept = GeoproxClient.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null)
            {
                localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);
            }

            localVarRequestOptions.PathParameters.Add("ghash", GeoproxClient.Client.ClientUtils.ParameterToString(ghash)); // path parameter

            localVarRequestOptions.Operation = "GeohashApiApi.GetNeighbors";
            localVarRequestOptions.OperationIndex = operationIndex;


            // make the HTTP request
            var localVarResponse = await this.AsynchronousClient.GetAsync<GeohashNeighborsResponse>("/api/v1/geohash/{ghash}/neighbors/", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("GetNeighbors", localVarResponse);
                if (_exception != null)
                {
                    throw _exception;
                }
            }

            return localVarResponse;
        }

    }
}
