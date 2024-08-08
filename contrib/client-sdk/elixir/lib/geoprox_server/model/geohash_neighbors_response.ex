# NOTE: This file is auto generated by OpenAPI Generator 7.7.0 (https://openapi-generator.tech).
# Do not edit this file manually.

defmodule GeoproxServer.Model.GeohashNeighborsResponse do
  @moduledoc """
  Neighboring geohash regions
  """

  @derive Jason.Encoder
  defstruct [
    :e,
    :n,
    :ne,
    :nw,
    :s,
    :se,
    :sw,
    :w
  ]

  @type t :: %__MODULE__{
    :e => String.t,
    :n => String.t,
    :ne => String.t,
    :nw => String.t,
    :s => String.t,
    :se => String.t,
    :sw => String.t,
    :w => String.t
  }

  def decode(value) do
    value
  end
end

