# InsertKeyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Geohash** | **string** | Geohash encoded latitude/longitude | 
**Key** | **string** | Object key | 

## Methods

### NewInsertKeyResponse

`func NewInsertKeyResponse(geohash string, key string, ) *InsertKeyResponse`

NewInsertKeyResponse instantiates a new InsertKeyResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInsertKeyResponseWithDefaults

`func NewInsertKeyResponseWithDefaults() *InsertKeyResponse`

NewInsertKeyResponseWithDefaults instantiates a new InsertKeyResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetGeohash

`func (o *InsertKeyResponse) GetGeohash() string`

GetGeohash returns the Geohash field if non-nil, zero value otherwise.

### GetGeohashOk

`func (o *InsertKeyResponse) GetGeohashOk() (*string, bool)`

GetGeohashOk returns a tuple with the Geohash field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGeohash

`func (o *InsertKeyResponse) SetGeohash(v string)`

SetGeohash sets Geohash field to given value.


### GetKey

`func (o *InsertKeyResponse) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *InsertKeyResponse) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *InsertKeyResponse) SetKey(v string)`

SetKey sets Key field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


