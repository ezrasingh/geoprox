/*
geoprox-server

Geoprox server implementation providing a HTTP API for geospatial queries and position tracking

API version: 0.4.1
Contact: singhezra@gmail.com
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package geoprox_client

import (
	"encoding/json"
	"bytes"
	"fmt"
)

// checks if the Neighbor type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &Neighbor{}

// Neighbor Nearby object
type Neighbor struct {
	// Distance in kilometers
	Distance float64 `json:"distance"`
	// Object key
	Key string `json:"key"`
}

type _Neighbor Neighbor

// NewNeighbor instantiates a new Neighbor object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewNeighbor(distance float64, key string) *Neighbor {
	this := Neighbor{}
	this.Distance = distance
	this.Key = key
	return &this
}

// NewNeighborWithDefaults instantiates a new Neighbor object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewNeighborWithDefaults() *Neighbor {
	this := Neighbor{}
	return &this
}

// GetDistance returns the Distance field value
func (o *Neighbor) GetDistance() float64 {
	if o == nil {
		var ret float64
		return ret
	}

	return o.Distance
}

// GetDistanceOk returns a tuple with the Distance field value
// and a boolean to check if the value has been set.
func (o *Neighbor) GetDistanceOk() (*float64, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Distance, true
}

// SetDistance sets field value
func (o *Neighbor) SetDistance(v float64) {
	o.Distance = v
}

// GetKey returns the Key field value
func (o *Neighbor) GetKey() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Key
}

// GetKeyOk returns a tuple with the Key field value
// and a boolean to check if the value has been set.
func (o *Neighbor) GetKeyOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Key, true
}

// SetKey sets field value
func (o *Neighbor) SetKey(v string) {
	o.Key = v
}

func (o Neighbor) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o Neighbor) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	toSerialize["distance"] = o.Distance
	toSerialize["key"] = o.Key
	return toSerialize, nil
}

func (o *Neighbor) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"distance",
		"key",
	}

	allProperties := make(map[string]interface{})

	err = json.Unmarshal(data, &allProperties)

	if err != nil {
		return err;
	}

	for _, requiredProperty := range(requiredProperties) {
		if _, exists := allProperties[requiredProperty]; !exists {
			return fmt.Errorf("no value given for required property %v", requiredProperty)
		}
	}

	varNeighbor := _Neighbor{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varNeighbor)

	if err != nil {
		return err
	}

	*o = Neighbor(varNeighbor)

	return err
}

type NullableNeighbor struct {
	value *Neighbor
	isSet bool
}

func (v NullableNeighbor) Get() *Neighbor {
	return v.value
}

func (v *NullableNeighbor) Set(val *Neighbor) {
	v.value = val
	v.isSet = true
}

func (v NullableNeighbor) IsSet() bool {
	return v.isSet
}

func (v *NullableNeighbor) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableNeighbor(val *Neighbor) *NullableNeighbor {
	return &NullableNeighbor{value: val, isSet: true}
}

func (v NullableNeighbor) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableNeighbor) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


