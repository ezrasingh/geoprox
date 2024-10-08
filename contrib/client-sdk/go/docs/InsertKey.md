# InsertKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Key** | **string** | Object key | 
**Lat** | **float64** | Latitude | 
**Lng** | **float64** | Longitude | 
**Ttl** | Pointer to **NullableInt64** | The time-to-live (TTL) for this key, in seconds | [optional] 

## Methods

### NewInsertKey

`func NewInsertKey(key string, lat float64, lng float64, ) *InsertKey`

NewInsertKey instantiates a new InsertKey object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInsertKeyWithDefaults

`func NewInsertKeyWithDefaults() *InsertKey`

NewInsertKeyWithDefaults instantiates a new InsertKey object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetKey

`func (o *InsertKey) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *InsertKey) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *InsertKey) SetKey(v string)`

SetKey sets Key field to given value.


### GetLat

`func (o *InsertKey) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *InsertKey) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *InsertKey) SetLat(v float64)`

SetLat sets Lat field to given value.


### GetLng

`func (o *InsertKey) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *InsertKey) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *InsertKey) SetLng(v float64)`

SetLng sets Lng field to given value.


### GetTtl

`func (o *InsertKey) GetTtl() int64`

GetTtl returns the Ttl field if non-nil, zero value otherwise.

### GetTtlOk

`func (o *InsertKey) GetTtlOk() (*int64, bool)`

GetTtlOk returns a tuple with the Ttl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTtl

`func (o *InsertKey) SetTtl(v int64)`

SetTtl sets Ttl field to given value.

### HasTtl

`func (o *InsertKey) HasTtl() bool`

HasTtl returns a boolean if a field has been set.

### SetTtlNil

`func (o *InsertKey) SetTtlNil(b bool)`

 SetTtlNil sets the value for Ttl to be an explicit nil

### UnsetTtl
`func (o *InsertKey) UnsetTtl()`

UnsetTtl ensures that no value is present for Ttl, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


