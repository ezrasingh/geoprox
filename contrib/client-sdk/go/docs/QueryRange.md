# QueryRange

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Count** | Pointer to **NullableInt32** | Maximum number of neighbors that can be returned (default 100) | [optional] 
**Lat** | **float64** | Latitude | 
**Lng** | **float64** | Longitude | 
**Range** | **int32** | Search radius in kilometers | 
**Sorted** | Pointer to **NullableBool** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] 

## Methods

### NewQueryRange

`func NewQueryRange(lat float64, lng float64, range_ int32, ) *QueryRange`

NewQueryRange instantiates a new QueryRange object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewQueryRangeWithDefaults

`func NewQueryRangeWithDefaults() *QueryRange`

NewQueryRangeWithDefaults instantiates a new QueryRange object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCount

`func (o *QueryRange) GetCount() int32`

GetCount returns the Count field if non-nil, zero value otherwise.

### GetCountOk

`func (o *QueryRange) GetCountOk() (*int32, bool)`

GetCountOk returns a tuple with the Count field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCount

`func (o *QueryRange) SetCount(v int32)`

SetCount sets Count field to given value.

### HasCount

`func (o *QueryRange) HasCount() bool`

HasCount returns a boolean if a field has been set.

### SetCountNil

`func (o *QueryRange) SetCountNil(b bool)`

 SetCountNil sets the value for Count to be an explicit nil

### UnsetCount
`func (o *QueryRange) UnsetCount()`

UnsetCount ensures that no value is present for Count, not even an explicit nil
### GetLat

`func (o *QueryRange) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *QueryRange) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *QueryRange) SetLat(v float64)`

SetLat sets Lat field to given value.


### GetLng

`func (o *QueryRange) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *QueryRange) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *QueryRange) SetLng(v float64)`

SetLng sets Lng field to given value.


### GetRange

`func (o *QueryRange) GetRange() int32`

GetRange returns the Range field if non-nil, zero value otherwise.

### GetRangeOk

`func (o *QueryRange) GetRangeOk() (*int32, bool)`

GetRangeOk returns a tuple with the Range field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRange

`func (o *QueryRange) SetRange(v int32)`

SetRange sets Range field to given value.


### GetSorted

`func (o *QueryRange) GetSorted() bool`

GetSorted returns the Sorted field if non-nil, zero value otherwise.

### GetSortedOk

`func (o *QueryRange) GetSortedOk() (*bool, bool)`

GetSortedOk returns a tuple with the Sorted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSorted

`func (o *QueryRange) SetSorted(v bool)`

SetSorted sets Sorted field to given value.

### HasSorted

`func (o *QueryRange) HasSorted() bool`

HasSorted returns a boolean if a field has been set.

### SetSortedNil

`func (o *QueryRange) SetSortedNil(b bool)`

 SetSortedNil sets the value for Sorted to be an explicit nil

### UnsetSorted
`func (o *QueryRange) UnsetSorted()`

UnsetSorted ensures that no value is present for Sorted, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


