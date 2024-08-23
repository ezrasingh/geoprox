=begin
#geoprox-server

#Geoprox server implementation providing a HTTP API for geospatial queries and position tracking

The version of the OpenAPI document: 0.5.0
Contact: singhezra@gmail.com
Generated by: https://openapi-generator.tech
Generator version: 7.7.0

=end

require 'cgi'

module GeoproxClient
  class GeoshardApiApi
    attr_accessor :api_client

    def initialize(api_client = ApiClient.default)
      @api_client = api_client
    end
    # Create geospatial index
    # Creates an in-memory index within this geoshard
    # @param index [String] Geospatial index name
    # @param [Hash] opts the optional parameters
    # @return [CreateIndexResponse]
    def create_index(index, opts = {})
      data, _status_code, _headers = create_index_with_http_info(index, opts)
      data
    end

    # Create geospatial index
    # Creates an in-memory index within this geoshard
    # @param index [String] Geospatial index name
    # @param [Hash] opts the optional parameters
    # @return [Array<(CreateIndexResponse, Integer, Hash)>] CreateIndexResponse data, response status code and response headers
    def create_index_with_http_info(index, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: GeoshardApiApi.create_index ...'
      end
      # verify the required parameter 'index' is set
      if @api_client.config.client_side_validation && index.nil?
        fail ArgumentError, "Missing the required parameter 'index' when calling GeoshardApiApi.create_index"
      end
      # resource path
      local_var_path = '/api/v1/shard/{index}'.sub('{' + 'index' + '}', CGI.escape(index.to_s))

      # query parameters
      query_params = opts[:query_params] || {}

      # header parameters
      header_params = opts[:header_params] || {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])

      # form parameters
      form_params = opts[:form_params] || {}

      # http body (model)
      post_body = opts[:debug_body]

      # return_type
      return_type = opts[:debug_return_type] || 'CreateIndexResponse'

      # auth_names
      auth_names = opts[:debug_auth_names] || []

      new_options = opts.merge(
        :operation => :"GeoshardApiApi.create_index",
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => return_type
      )

      data, status_code, headers = @api_client.call_api(:POST, local_var_path, new_options)
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: GeoshardApiApi#create_index\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end

    # Deletes geospatial index
    # Drop index. All keys will be lost
    # @param index [String] Geospatial index name
    # @param [Hash] opts the optional parameters
    # @return [DropIndexResponse]
    def drop_index(index, opts = {})
      data, _status_code, _headers = drop_index_with_http_info(index, opts)
      data
    end

    # Deletes geospatial index
    # Drop index. All keys will be lost
    # @param index [String] Geospatial index name
    # @param [Hash] opts the optional parameters
    # @return [Array<(DropIndexResponse, Integer, Hash)>] DropIndexResponse data, response status code and response headers
    def drop_index_with_http_info(index, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: GeoshardApiApi.drop_index ...'
      end
      # verify the required parameter 'index' is set
      if @api_client.config.client_side_validation && index.nil?
        fail ArgumentError, "Missing the required parameter 'index' when calling GeoshardApiApi.drop_index"
      end
      # resource path
      local_var_path = '/api/v1/shard/{index}'.sub('{' + 'index' + '}', CGI.escape(index.to_s))

      # query parameters
      query_params = opts[:query_params] || {}

      # header parameters
      header_params = opts[:header_params] || {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])

      # form parameters
      form_params = opts[:form_params] || {}

      # http body (model)
      post_body = opts[:debug_body]

      # return_type
      return_type = opts[:debug_return_type] || 'DropIndexResponse'

      # auth_names
      auth_names = opts[:debug_auth_names] || []

      new_options = opts.merge(
        :operation => :"GeoshardApiApi.drop_index",
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => return_type
      )

      data, status_code, headers = @api_client.call_api(:DELETE, local_var_path, new_options)
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: GeoshardApiApi#drop_index\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end

    # Insert key into index
    # Inserts key into geospatial index
    # @param index [String] Geospatial index name
    # @param insert_key [InsertKey] 
    # @param [Hash] opts the optional parameters
    # @return [InsertKeyResponse]
    def insert_key(index, insert_key, opts = {})
      data, _status_code, _headers = insert_key_with_http_info(index, insert_key, opts)
      data
    end

    # Insert key into index
    # Inserts key into geospatial index
    # @param index [String] Geospatial index name
    # @param insert_key [InsertKey] 
    # @param [Hash] opts the optional parameters
    # @return [Array<(InsertKeyResponse, Integer, Hash)>] InsertKeyResponse data, response status code and response headers
    def insert_key_with_http_info(index, insert_key, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: GeoshardApiApi.insert_key ...'
      end
      # verify the required parameter 'index' is set
      if @api_client.config.client_side_validation && index.nil?
        fail ArgumentError, "Missing the required parameter 'index' when calling GeoshardApiApi.insert_key"
      end
      # verify the required parameter 'insert_key' is set
      if @api_client.config.client_side_validation && insert_key.nil?
        fail ArgumentError, "Missing the required parameter 'insert_key' when calling GeoshardApiApi.insert_key"
      end
      # resource path
      local_var_path = '/api/v1/shard/{index}'.sub('{' + 'index' + '}', CGI.escape(index.to_s))

      # query parameters
      query_params = opts[:query_params] || {}

      # header parameters
      header_params = opts[:header_params] || {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])
      # HTTP header 'Content-Type'
      content_type = @api_client.select_header_content_type(['application/json'])
      if !content_type.nil?
          header_params['Content-Type'] = content_type
      end

      # form parameters
      form_params = opts[:form_params] || {}

      # http body (model)
      post_body = opts[:debug_body] || @api_client.object_to_http_body(insert_key)

      # return_type
      return_type = opts[:debug_return_type] || 'InsertKeyResponse'

      # auth_names
      auth_names = opts[:debug_auth_names] || []

      new_options = opts.merge(
        :operation => :"GeoshardApiApi.insert_key",
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => return_type
      )

      data, status_code, headers = @api_client.call_api(:PUT, local_var_path, new_options)
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: GeoshardApiApi#insert_key\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end

    # Insert multiple keys into index
    # Inserts multiple keys into geospatial index
    # @param index [String] Geospatial index name
    # @param insert_key_batch [InsertKeyBatch] 
    # @param [Hash] opts the optional parameters
    # @return [InsertKeyBatchResponse]
    def insert_key_batch(index, insert_key_batch, opts = {})
      data, _status_code, _headers = insert_key_batch_with_http_info(index, insert_key_batch, opts)
      data
    end

    # Insert multiple keys into index
    # Inserts multiple keys into geospatial index
    # @param index [String] Geospatial index name
    # @param insert_key_batch [InsertKeyBatch] 
    # @param [Hash] opts the optional parameters
    # @return [Array<(InsertKeyBatchResponse, Integer, Hash)>] InsertKeyBatchResponse data, response status code and response headers
    def insert_key_batch_with_http_info(index, insert_key_batch, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: GeoshardApiApi.insert_key_batch ...'
      end
      # verify the required parameter 'index' is set
      if @api_client.config.client_side_validation && index.nil?
        fail ArgumentError, "Missing the required parameter 'index' when calling GeoshardApiApi.insert_key_batch"
      end
      # verify the required parameter 'insert_key_batch' is set
      if @api_client.config.client_side_validation && insert_key_batch.nil?
        fail ArgumentError, "Missing the required parameter 'insert_key_batch' when calling GeoshardApiApi.insert_key_batch"
      end
      # resource path
      local_var_path = '/api/v1/shard/{index}/batch'.sub('{' + 'index' + '}', CGI.escape(index.to_s))

      # query parameters
      query_params = opts[:query_params] || {}

      # header parameters
      header_params = opts[:header_params] || {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])
      # HTTP header 'Content-Type'
      content_type = @api_client.select_header_content_type(['application/json'])
      if !content_type.nil?
          header_params['Content-Type'] = content_type
      end

      # form parameters
      form_params = opts[:form_params] || {}

      # http body (model)
      post_body = opts[:debug_body] || @api_client.object_to_http_body(insert_key_batch)

      # return_type
      return_type = opts[:debug_return_type] || 'InsertKeyBatchResponse'

      # auth_names
      auth_names = opts[:debug_auth_names] || []

      new_options = opts.merge(
        :operation => :"GeoshardApiApi.insert_key_batch",
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => return_type
      )

      data, status_code, headers = @api_client.call_api(:PUT, local_var_path, new_options)
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: GeoshardApiApi#insert_key_batch\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end

    # Search index for objects nearby
    # Search geospatial index for all keys within some distance
    # @param index [String] Geospatial index name
    # @param lat [Float] Latitude
    # @param lng [Float] Longitude
    # @param range [Integer] Search radius in kilometers
    # @param [Hash] opts the optional parameters
    # @option opts [Integer] :count Maximum number of neighbors that can be returned (default 100)
    # @option opts [Boolean] :sorted If enabled neighbors will be sorted by distance, nearest to furthest (default false)
    # @return [QueryRangeResponse]
    def query_range(index, lat, lng, range, opts = {})
      data, _status_code, _headers = query_range_with_http_info(index, lat, lng, range, opts)
      data
    end

    # Search index for objects nearby
    # Search geospatial index for all keys within some distance
    # @param index [String] Geospatial index name
    # @param lat [Float] Latitude
    # @param lng [Float] Longitude
    # @param range [Integer] Search radius in kilometers
    # @param [Hash] opts the optional parameters
    # @option opts [Integer] :count Maximum number of neighbors that can be returned (default 100)
    # @option opts [Boolean] :sorted If enabled neighbors will be sorted by distance, nearest to furthest (default false)
    # @return [Array<(QueryRangeResponse, Integer, Hash)>] QueryRangeResponse data, response status code and response headers
    def query_range_with_http_info(index, lat, lng, range, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: GeoshardApiApi.query_range ...'
      end
      # verify the required parameter 'index' is set
      if @api_client.config.client_side_validation && index.nil?
        fail ArgumentError, "Missing the required parameter 'index' when calling GeoshardApiApi.query_range"
      end
      # verify the required parameter 'lat' is set
      if @api_client.config.client_side_validation && lat.nil?
        fail ArgumentError, "Missing the required parameter 'lat' when calling GeoshardApiApi.query_range"
      end
      # verify the required parameter 'lng' is set
      if @api_client.config.client_side_validation && lng.nil?
        fail ArgumentError, "Missing the required parameter 'lng' when calling GeoshardApiApi.query_range"
      end
      # verify the required parameter 'range' is set
      if @api_client.config.client_side_validation && range.nil?
        fail ArgumentError, "Missing the required parameter 'range' when calling GeoshardApiApi.query_range"
      end
      if @api_client.config.client_side_validation && range < 0
        fail ArgumentError, 'invalid value for "range" when calling GeoshardApiApi.query_range, must be greater than or equal to 0.'
      end

      if @api_client.config.client_side_validation && !opts[:'count'].nil? && opts[:'count'] < 0
        fail ArgumentError, 'invalid value for "opts[:"count"]" when calling GeoshardApiApi.query_range, must be greater than or equal to 0.'
      end

      # resource path
      local_var_path = '/api/v1/shard/{index}'.sub('{' + 'index' + '}', CGI.escape(index.to_s))

      # query parameters
      query_params = opts[:query_params] || {}
      query_params[:'lat'] = lat
      query_params[:'lng'] = lng
      query_params[:'range'] = range
      query_params[:'count'] = opts[:'count'] if !opts[:'count'].nil?
      query_params[:'sorted'] = opts[:'sorted'] if !opts[:'sorted'].nil?

      # header parameters
      header_params = opts[:header_params] || {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])

      # form parameters
      form_params = opts[:form_params] || {}

      # http body (model)
      post_body = opts[:debug_body]

      # return_type
      return_type = opts[:debug_return_type] || 'QueryRangeResponse'

      # auth_names
      auth_names = opts[:debug_auth_names] || []

      new_options = opts.merge(
        :operation => :"GeoshardApiApi.query_range",
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => return_type
      )

      data, status_code, headers = @api_client.call_api(:GET, local_var_path, new_options)
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: GeoshardApiApi#query_range\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end

    # Search multiple indices for objects nearby
    # Search geospatial many indices for all keys within some distance
    # @param indices [Array<String>] List of indices to search
    # @param lat [Float] Latitude
    # @param lng [Float] Longitude
    # @param range [Integer] Search radius in kilometers
    # @param [Hash] opts the optional parameters
    # @option opts [Integer] :count Maximum number of neighbors that can be returned (default 100)
    # @option opts [Boolean] :sorted If enabled neighbors will be sorted by distance, nearest to furthest (default false)
    # @return [QueryRangeManyResponse]
    def query_range_many(indices, lat, lng, range, opts = {})
      data, _status_code, _headers = query_range_many_with_http_info(indices, lat, lng, range, opts)
      data
    end

    # Search multiple indices for objects nearby
    # Search geospatial many indices for all keys within some distance
    # @param indices [Array<String>] List of indices to search
    # @param lat [Float] Latitude
    # @param lng [Float] Longitude
    # @param range [Integer] Search radius in kilometers
    # @param [Hash] opts the optional parameters
    # @option opts [Integer] :count Maximum number of neighbors that can be returned (default 100)
    # @option opts [Boolean] :sorted If enabled neighbors will be sorted by distance, nearest to furthest (default false)
    # @return [Array<(QueryRangeManyResponse, Integer, Hash)>] QueryRangeManyResponse data, response status code and response headers
    def query_range_many_with_http_info(indices, lat, lng, range, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: GeoshardApiApi.query_range_many ...'
      end
      # verify the required parameter 'indices' is set
      if @api_client.config.client_side_validation && indices.nil?
        fail ArgumentError, "Missing the required parameter 'indices' when calling GeoshardApiApi.query_range_many"
      end
      # verify the required parameter 'lat' is set
      if @api_client.config.client_side_validation && lat.nil?
        fail ArgumentError, "Missing the required parameter 'lat' when calling GeoshardApiApi.query_range_many"
      end
      # verify the required parameter 'lng' is set
      if @api_client.config.client_side_validation && lng.nil?
        fail ArgumentError, "Missing the required parameter 'lng' when calling GeoshardApiApi.query_range_many"
      end
      # verify the required parameter 'range' is set
      if @api_client.config.client_side_validation && range.nil?
        fail ArgumentError, "Missing the required parameter 'range' when calling GeoshardApiApi.query_range_many"
      end
      if @api_client.config.client_side_validation && range < 0
        fail ArgumentError, 'invalid value for "range" when calling GeoshardApiApi.query_range_many, must be greater than or equal to 0.'
      end

      if @api_client.config.client_side_validation && !opts[:'count'].nil? && opts[:'count'] < 0
        fail ArgumentError, 'invalid value for "opts[:"count"]" when calling GeoshardApiApi.query_range_many, must be greater than or equal to 0.'
      end

      # resource path
      local_var_path = '/api/v1/shard'

      # query parameters
      query_params = opts[:query_params] || {}
      query_params[:'indices'] = @api_client.build_collection_param(indices, :multi)
      query_params[:'lat'] = lat
      query_params[:'lng'] = lng
      query_params[:'range'] = range
      query_params[:'count'] = opts[:'count'] if !opts[:'count'].nil?
      query_params[:'sorted'] = opts[:'sorted'] if !opts[:'sorted'].nil?

      # header parameters
      header_params = opts[:header_params] || {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])

      # form parameters
      form_params = opts[:form_params] || {}

      # http body (model)
      post_body = opts[:debug_body]

      # return_type
      return_type = opts[:debug_return_type] || 'QueryRangeManyResponse'

      # auth_names
      auth_names = opts[:debug_auth_names] || []

      new_options = opts.merge(
        :operation => :"GeoshardApiApi.query_range_many",
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => return_type
      )

      data, status_code, headers = @api_client.call_api(:GET, local_var_path, new_options)
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: GeoshardApiApi#query_range_many\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end

    # Remove key from index
    # Removes key from geospatial index
    # @param index [String] Geospatial index name
    # @param remove_key [RemoveKey] 
    # @param [Hash] opts the optional parameters
    # @return [RemoveKeyResponse]
    def remove_key(index, remove_key, opts = {})
      data, _status_code, _headers = remove_key_with_http_info(index, remove_key, opts)
      data
    end

    # Remove key from index
    # Removes key from geospatial index
    # @param index [String] Geospatial index name
    # @param remove_key [RemoveKey] 
    # @param [Hash] opts the optional parameters
    # @return [Array<(RemoveKeyResponse, Integer, Hash)>] RemoveKeyResponse data, response status code and response headers
    def remove_key_with_http_info(index, remove_key, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: GeoshardApiApi.remove_key ...'
      end
      # verify the required parameter 'index' is set
      if @api_client.config.client_side_validation && index.nil?
        fail ArgumentError, "Missing the required parameter 'index' when calling GeoshardApiApi.remove_key"
      end
      # verify the required parameter 'remove_key' is set
      if @api_client.config.client_side_validation && remove_key.nil?
        fail ArgumentError, "Missing the required parameter 'remove_key' when calling GeoshardApiApi.remove_key"
      end
      # resource path
      local_var_path = '/api/v1/shard/{index}'.sub('{' + 'index' + '}', CGI.escape(index.to_s))

      # query parameters
      query_params = opts[:query_params] || {}

      # header parameters
      header_params = opts[:header_params] || {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])
      # HTTP header 'Content-Type'
      content_type = @api_client.select_header_content_type(['application/json'])
      if !content_type.nil?
          header_params['Content-Type'] = content_type
      end

      # form parameters
      form_params = opts[:form_params] || {}

      # http body (model)
      post_body = opts[:debug_body] || @api_client.object_to_http_body(remove_key)

      # return_type
      return_type = opts[:debug_return_type] || 'RemoveKeyResponse'

      # auth_names
      auth_names = opts[:debug_auth_names] || []

      new_options = opts.merge(
        :operation => :"GeoshardApiApi.remove_key",
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => return_type
      )

      data, status_code, headers = @api_client.call_api(:PATCH, local_var_path, new_options)
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: GeoshardApiApi#remove_key\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end

    # Remove multiple keys from index
    # Removes multiple keys from geospatial index
    # @param index [String] Geospatial index name
    # @param remove_key_batch [RemoveKeyBatch] 
    # @param [Hash] opts the optional parameters
    # @return [RemoveKeyBatchResponse]
    def remove_key_batch(index, remove_key_batch, opts = {})
      data, _status_code, _headers = remove_key_batch_with_http_info(index, remove_key_batch, opts)
      data
    end

    # Remove multiple keys from index
    # Removes multiple keys from geospatial index
    # @param index [String] Geospatial index name
    # @param remove_key_batch [RemoveKeyBatch] 
    # @param [Hash] opts the optional parameters
    # @return [Array<(RemoveKeyBatchResponse, Integer, Hash)>] RemoveKeyBatchResponse data, response status code and response headers
    def remove_key_batch_with_http_info(index, remove_key_batch, opts = {})
      if @api_client.config.debugging
        @api_client.config.logger.debug 'Calling API: GeoshardApiApi.remove_key_batch ...'
      end
      # verify the required parameter 'index' is set
      if @api_client.config.client_side_validation && index.nil?
        fail ArgumentError, "Missing the required parameter 'index' when calling GeoshardApiApi.remove_key_batch"
      end
      # verify the required parameter 'remove_key_batch' is set
      if @api_client.config.client_side_validation && remove_key_batch.nil?
        fail ArgumentError, "Missing the required parameter 'remove_key_batch' when calling GeoshardApiApi.remove_key_batch"
      end
      # resource path
      local_var_path = '/api/v1/shard/{index}/batch'.sub('{' + 'index' + '}', CGI.escape(index.to_s))

      # query parameters
      query_params = opts[:query_params] || {}

      # header parameters
      header_params = opts[:header_params] || {}
      # HTTP header 'Accept' (if needed)
      header_params['Accept'] = @api_client.select_header_accept(['application/json'])
      # HTTP header 'Content-Type'
      content_type = @api_client.select_header_content_type(['application/json'])
      if !content_type.nil?
          header_params['Content-Type'] = content_type
      end

      # form parameters
      form_params = opts[:form_params] || {}

      # http body (model)
      post_body = opts[:debug_body] || @api_client.object_to_http_body(remove_key_batch)

      # return_type
      return_type = opts[:debug_return_type] || 'RemoveKeyBatchResponse'

      # auth_names
      auth_names = opts[:debug_auth_names] || []

      new_options = opts.merge(
        :operation => :"GeoshardApiApi.remove_key_batch",
        :header_params => header_params,
        :query_params => query_params,
        :form_params => form_params,
        :body => post_body,
        :auth_names => auth_names,
        :return_type => return_type
      )

      data, status_code, headers = @api_client.call_api(:PATCH, local_var_path, new_options)
      if @api_client.config.debugging
        @api_client.config.logger.debug "API called: GeoshardApiApi#remove_key_batch\nData: #{data.inspect}\nStatus code: #{status_code}\nHeaders: #{headers}"
      end
      return data, status_code, headers
    end
  end
end
