# NOTE: This file is auto generated by OpenAPI Generator 7.7.0 (https://openapi-generator.tech).
# Do not edit this file manually.

defmodule GeoproxServer.Model.RemoveKeyBatchResponse do
  @moduledoc """
  Returns batch key deletion status
  """

  @derive Jason.Encoder
  defstruct [
    :deleted
  ]

  @type t :: %__MODULE__{
    :deleted => boolean()
  }

  def decode(value) do
    value
  end
end
