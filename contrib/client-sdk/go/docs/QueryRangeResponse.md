# QueryRangeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Found** | [**[]Neighbor**](Neighbor.md) | Object keys found within range | 

## Methods

### NewQueryRangeResponse

`func NewQueryRangeResponse(found []Neighbor, ) *QueryRangeResponse`

NewQueryRangeResponse instantiates a new QueryRangeResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewQueryRangeResponseWithDefaults

`func NewQueryRangeResponseWithDefaults() *QueryRangeResponse`

NewQueryRangeResponseWithDefaults instantiates a new QueryRangeResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetFound

`func (o *QueryRangeResponse) GetFound() []Neighbor`

GetFound returns the Found field if non-nil, zero value otherwise.

### GetFoundOk

`func (o *QueryRangeResponse) GetFoundOk() (*[]Neighbor, bool)`

GetFoundOk returns a tuple with the Found field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFound

`func (o *QueryRangeResponse) SetFound(v []Neighbor)`

SetFound sets Found field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


