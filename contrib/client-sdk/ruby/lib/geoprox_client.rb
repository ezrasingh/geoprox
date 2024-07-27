=begin
#geoprox-server

#No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

The version of the OpenAPI document: 0.1.0

Generated by: https://openapi-generator.tech
Generator version: 7.7.0

=end

# Common files
require 'geoprox_client/api_client'
require 'geoprox_client/api_error'
require 'geoprox_client/version'
require 'geoprox_client/configuration'

# Models
require 'geoprox_client/models/create_index_response'
require 'geoprox_client/models/decode_geohash_response'
require 'geoprox_client/models/drop_index_response'
require 'geoprox_client/models/encode_lat_lng'
require 'geoprox_client/models/encode_lat_lng_response'
require 'geoprox_client/models/geohash_neighbors_response'
require 'geoprox_client/models/insert_key'
require 'geoprox_client/models/insert_key_response'
require 'geoprox_client/models/keys_found_inner'
require 'geoprox_client/models/query_range'
require 'geoprox_client/models/query_range_response'
require 'geoprox_client/models/remove_key'
require 'geoprox_client/models/remove_key_response'

# APIs
require 'geoprox_client/api/geohash_api_api'
require 'geoprox_client/api/geoshard_api_api'

module GeoproxClient
  class << self
    # Customize default settings for the SDK using block.
    #   GeoproxClient.configure do |config|
    #     config.username = "xxx"
    #     config.password = "xxx"
    #   end
    # If no block given, return the default Configuration object.
    def configure
      if block_given?
        yield(Configuration.default)
      else
        Configuration.default
      end
    end
  end
end
