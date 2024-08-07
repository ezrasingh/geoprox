# NOTE: This file is auto generated by OpenAPI Generator 7.7.0 (https://openapi-generator.tech).
# Do not edit this file manually.

defmodule GeoproxServer.Model.EncodeLatLngResponse do
  @moduledoc """
  Returns geohash encoded latitude/longitude
  """

  @derive Jason.Encoder
  defstruct [
    :geohash
  ]

  @type t :: %__MODULE__{
    :geohash => String.t
  }

  def decode(value) do
    value
  end
end

