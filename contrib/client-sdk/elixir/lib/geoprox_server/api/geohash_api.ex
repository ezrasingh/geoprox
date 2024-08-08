# NOTE: This file is auto generated by OpenAPI Generator 7.7.0 (https://openapi-generator.tech).
# Do not edit this file manually.

defmodule GeoproxServer.Api.GeohashApi do
  @moduledoc """
  API calls for all endpoints tagged `GeohashApi`.
  """

  alias GeoproxServer.Connection
  import GeoproxServer.RequestBuilder

  @doc """
  Decode geohash into coordinates.
  Decode geohash by path param, returns coordinates with precision estimates.

  ### Parameters

  - `connection` (GeoproxServer.Connection): Connection to server
  - `ghash` (String.t): Geohash encoded region
  - `opts` (keyword): Optional parameters

  ### Returns

  - `{:ok, GeoproxServer.Model.DecodeGeohashResponse.t}` on success
  - `{:error, Tesla.Env.t}` on failure
  """
  @spec decode_geohash(Tesla.Env.client, String.t, keyword()) :: {:ok, GeoproxServer.Model.DecodeGeohashResponse.t} | {:error, Tesla.Env.t}
  def decode_geohash(connection, ghash, _opts \\ []) do
    request =
      %{}
      |> method(:get)
      |> url("/api/v1/geohash/#{ghash}/")
      |> Enum.into([])

    connection
    |> Connection.request(request)
    |> evaluate_response([
      {200, GeoproxServer.Model.DecodeGeohashResponse}
    ])
  end

  @doc """
  Encode coordinates into geohash
  Encode coordinates by query params, returns geohash.

  ### Parameters

  - `connection` (GeoproxServer.Connection): Connection to server
  - `lat` (float()): Latitude
  - `lng` (float()): Longitude
  - `depth` (integer()): Determines geohash length
  - `opts` (keyword): Optional parameters

  ### Returns

  - `{:ok, GeoproxServer.Model.EncodeLatLngResponse.t}` on success
  - `{:error, Tesla.Env.t}` on failure
  """
  @spec encode_latlng(Tesla.Env.client, float(), float(), integer(), keyword()) :: {:ok, GeoproxServer.Model.EncodeLatLngResponse.t} | {:error, Tesla.Env.t}
  def encode_latlng(connection, lat, lng, depth, _opts \\ []) do
    request =
      %{}
      |> method(:get)
      |> url("/api/v1/geohash/")
      |> add_param(:query, :lat, lat)
      |> add_param(:query, :lng, lng)
      |> add_param(:query, :depth, depth)
      |> Enum.into([])

    connection
    |> Connection.request(request)
    |> evaluate_response([
      {200, GeoproxServer.Model.EncodeLatLngResponse}
    ])
  end

  @doc """
  Neighboring regions
  Returns geohash neighbors in all cardinal directions.

  ### Parameters

  - `connection` (GeoproxServer.Connection): Connection to server
  - `ghash` (String.t): Geohash encoded region
  - `opts` (keyword): Optional parameters

  ### Returns

  - `{:ok, GeoproxServer.Model.GeohashNeighborsResponse.t}` on success
  - `{:error, Tesla.Env.t}` on failure
  """
  @spec get_neighbors(Tesla.Env.client, String.t, keyword()) :: {:ok, GeoproxServer.Model.GeohashNeighborsResponse.t} | {:error, Tesla.Env.t}
  def get_neighbors(connection, ghash, _opts \\ []) do
    request =
      %{}
      |> method(:get)
      |> url("/api/v1/geohash/#{ghash}/neighbors/")
      |> Enum.into([])

    connection
    |> Connection.request(request)
    |> evaluate_response([
      {200, GeoproxServer.Model.GeohashNeighborsResponse}
    ])
  end
end