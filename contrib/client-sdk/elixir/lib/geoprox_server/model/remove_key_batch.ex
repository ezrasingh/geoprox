# NOTE: This file is auto generated by OpenAPI Generator 7.7.0 (https://openapi-generator.tech).
# Do not edit this file manually.

defmodule GeoproxServer.Model.RemoveKeyBatch do
  @moduledoc """
  Arguments for removing multiple keys
  """

  @derive Jason.Encoder
  defstruct [
    :keys
  ]

  @type t :: %__MODULE__{
    :keys => [String.t]
  }

  def decode(value) do
    value
  end
end

