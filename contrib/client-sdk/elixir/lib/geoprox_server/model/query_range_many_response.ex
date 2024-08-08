# NOTE: This file is auto generated by OpenAPI Generator 7.7.0 (https://openapi-generator.tech).
# Do not edit this file manually.

defmodule GeoproxServer.Model.QueryRangeManyResponse do
  @moduledoc """
  Returns indices and object keys found with their distance
  """

  @derive Jason.Encoder
  defstruct [
    :errors,
    :results
  ]

  @type t :: %__MODULE__{
    :errors => %{optional(String.t) => String.t},
    :results => %{optional(String.t) => [GeoproxServer.Model.Neighbor.t]}
  }

  def decode(value) do
    value
  end
end

