# NOTE: This file is auto generated by OpenAPI Generator 7.7.0 (https://openapi-generator.tech).
# Do not edit this file manually.

defmodule GeoproxServer.Model.EncodeLatLng do
  @moduledoc """
  Arguments for encoding latitude/longitude as geohash
  """

  @derive Jason.Encoder
  defstruct [
    :depth,
    :lat,
    :lng
  ]

  @type t :: %__MODULE__{
    :depth => integer(),
    :lat => float(),
    :lng => float()
  }

  def decode(value) do
    value
  end
end

