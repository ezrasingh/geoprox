=begin
#geoprox-server

#No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

The version of the OpenAPI document: 0.1.0

Generated by: https://openapi-generator.tech
Generator version: 7.7.0

=end

require 'spec_helper'
require 'json'

# Unit tests for GeoproxClient::GeohashApiApi
# Automatically generated by openapi-generator (https://openapi-generator.tech)
# Please update as you see appropriate
describe 'GeohashApiApi' do
  before do
    # run before each test
    @api_instance = GeoproxClient::GeohashApiApi.new
  end

  after do
    # run after each test
  end

  describe 'test an instance of GeohashApiApi' do
    it 'should create an instance of GeohashApiApi' do
      expect(@api_instance).to be_instance_of(GeoproxClient::GeohashApiApi)
    end
  end

  # unit tests for decode_geohash
  # Decode geohash into coordinates.
  # Decode geohash by path param, returns coordinates with precision estimates.
  # @param ghash Geohash encoded region
  # @param [Hash] opts the optional parameters
  # @return [DecodeGeohashResponse]
  describe 'decode_geohash test' do
    it 'should work' do
      # assertion here. ref: https://rspec.info/features/3-12/rspec-expectations/built-in-matchers/
    end
  end

  # unit tests for encode_latlng
  # Encode coordinates into geohash
  # Encode coordinates by query params, returns geohash.
  # @param lat Latitude
  # @param lng Longitude
  # @param depth Determines geohash length
  # @param [Hash] opts the optional parameters
  # @return [EncodeLatLngResponse]
  describe 'encode_latlng test' do
    it 'should work' do
      # assertion here. ref: https://rspec.info/features/3-12/rspec-expectations/built-in-matchers/
    end
  end

  # unit tests for get_neighbors
  # Neighboring regions
  # Returns geohash neighbors in all cardinal directions.
  # @param ghash Geohash encoded region
  # @param [Hash] opts the optional parameters
  # @return [GeohashNeighborsResponse]
  describe 'get_neighbors test' do
    it 'should work' do
      # assertion here. ref: https://rspec.info/features/3-12/rspec-expectations/built-in-matchers/
    end
  end

end