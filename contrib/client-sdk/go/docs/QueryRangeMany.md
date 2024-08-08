# QueryRangeMany

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Count** | Pointer to **NullableInt32** | Maximum number of neighbors that can be returned (default 100) | [optional] 
**Indices** | **[]string** | List of indices to search | 
**Lat** | **float64** | Latitude | 
**Lng** | **float64** | Longitude | 
**Range** | **int32** | Search radius in kilometers | 
**Sorted** | Pointer to **NullableBool** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] 

## Methods

### NewQueryRangeMany

`func NewQueryRangeMany(indices []string, lat float64, lng float64, range_ int32, ) *QueryRangeMany`

NewQueryRangeMany instantiates a new QueryRangeMany object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewQueryRangeManyWithDefaults

`func NewQueryRangeManyWithDefaults() *QueryRangeMany`

NewQueryRangeManyWithDefaults instantiates a new QueryRangeMany object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCount

`func (o *QueryRangeMany) GetCount() int32`

GetCount returns the Count field if non-nil, zero value otherwise.

### GetCountOk

`func (o *QueryRangeMany) GetCountOk() (*int32, bool)`

GetCountOk returns a tuple with the Count field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCount

`func (o *QueryRangeMany) SetCount(v int32)`

SetCount sets Count field to given value.

### HasCount

`func (o *QueryRangeMany) HasCount() bool`

HasCount returns a boolean if a field has been set.

### SetCountNil

`func (o *QueryRangeMany) SetCountNil(b bool)`

 SetCountNil sets the value for Count to be an explicit nil

### UnsetCount
`func (o *QueryRangeMany) UnsetCount()`

UnsetCount ensures that no value is present for Count, not even an explicit nil
### GetIndices

`func (o *QueryRangeMany) GetIndices() []string`

GetIndices returns the Indices field if non-nil, zero value otherwise.

### GetIndicesOk

`func (o *QueryRangeMany) GetIndicesOk() (*[]string, bool)`

GetIndicesOk returns a tuple with the Indices field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIndices

`func (o *QueryRangeMany) SetIndices(v []string)`

SetIndices sets Indices field to given value.


### GetLat

`func (o *QueryRangeMany) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *QueryRangeMany) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *QueryRangeMany) SetLat(v float64)`

SetLat sets Lat field to given value.


### GetLng

`func (o *QueryRangeMany) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *QueryRangeMany) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *QueryRangeMany) SetLng(v float64)`

SetLng sets Lng field to given value.


### GetRange

`func (o *QueryRangeMany) GetRange() int32`

GetRange returns the Range field if non-nil, zero value otherwise.

### GetRangeOk

`func (o *QueryRangeMany) GetRangeOk() (*int32, bool)`

GetRangeOk returns a tuple with the Range field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRange

`func (o *QueryRangeMany) SetRange(v int32)`

SetRange sets Range field to given value.


### GetSorted

`func (o *QueryRangeMany) GetSorted() bool`

GetSorted returns the Sorted field if non-nil, zero value otherwise.

### GetSortedOk

`func (o *QueryRangeMany) GetSortedOk() (*bool, bool)`

GetSortedOk returns a tuple with the Sorted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSorted

`func (o *QueryRangeMany) SetSorted(v bool)`

SetSorted sets Sorted field to given value.

### HasSorted

`func (o *QueryRangeMany) HasSorted() bool`

HasSorted returns a boolean if a field has been set.

### SetSortedNil

`func (o *QueryRangeMany) SetSortedNil(b bool)`

 SetSortedNil sets the value for Sorted to be an explicit nil

### UnsetSorted
`func (o *QueryRangeMany) UnsetSorted()`

UnsetSorted ensures that no value is present for Sorted, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


