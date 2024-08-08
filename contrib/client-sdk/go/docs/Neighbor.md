# Neighbor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Distance** | **float64** | Distance in kilometers | 
**Key** | **string** | Object key | 

## Methods

### NewNeighbor

`func NewNeighbor(distance float64, key string, ) *Neighbor`

NewNeighbor instantiates a new Neighbor object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewNeighborWithDefaults

`func NewNeighborWithDefaults() *Neighbor`

NewNeighborWithDefaults instantiates a new Neighbor object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDistance

`func (o *Neighbor) GetDistance() float64`

GetDistance returns the Distance field if non-nil, zero value otherwise.

### GetDistanceOk

`func (o *Neighbor) GetDistanceOk() (*float64, bool)`

GetDistanceOk returns a tuple with the Distance field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDistance

`func (o *Neighbor) SetDistance(v float64)`

SetDistance sets Distance field to given value.


### GetKey

`func (o *Neighbor) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *Neighbor) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *Neighbor) SetKey(v string)`

SetKey sets Key field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


