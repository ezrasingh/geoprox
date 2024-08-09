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

// checks if the InsertKey type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &InsertKey{}

// InsertKey Arguments for inserting a key
type InsertKey struct {
	// Object key
	Key string `json:"key"`
	// Latitude
	Lat float64 `json:"lat"`
	// Longitude
	Lng float64 `json:"lng"`
}

type _InsertKey InsertKey

// NewInsertKey instantiates a new InsertKey object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewInsertKey(key string, lat float64, lng float64) *InsertKey {
	this := InsertKey{}
	this.Key = key
	this.Lat = lat
	this.Lng = lng
	return &this
}

// NewInsertKeyWithDefaults instantiates a new InsertKey object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewInsertKeyWithDefaults() *InsertKey {
	this := InsertKey{}
	return &this
}

// GetKey returns the Key field value
func (o *InsertKey) GetKey() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Key
}

// GetKeyOk returns a tuple with the Key field value
// and a boolean to check if the value has been set.
func (o *InsertKey) GetKeyOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Key, true
}

// SetKey sets field value
func (o *InsertKey) SetKey(v string) {
	o.Key = v
}

// GetLat returns the Lat field value
func (o *InsertKey) GetLat() float64 {
	if o == nil {
		var ret float64
		return ret
	}

	return o.Lat
}

// GetLatOk returns a tuple with the Lat field value
// and a boolean to check if the value has been set.
func (o *InsertKey) GetLatOk() (*float64, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Lat, true
}

// SetLat sets field value
func (o *InsertKey) SetLat(v float64) {
	o.Lat = v
}

// GetLng returns the Lng field value
func (o *InsertKey) GetLng() float64 {
	if o == nil {
		var ret float64
		return ret
	}

	return o.Lng
}

// GetLngOk returns a tuple with the Lng field value
// and a boolean to check if the value has been set.
func (o *InsertKey) GetLngOk() (*float64, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Lng, true
}

// SetLng sets field value
func (o *InsertKey) SetLng(v float64) {
	o.Lng = v
}

func (o InsertKey) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o InsertKey) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	toSerialize["key"] = o.Key
	toSerialize["lat"] = o.Lat
	toSerialize["lng"] = o.Lng
	return toSerialize, nil
}

func (o *InsertKey) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"key",
		"lat",
		"lng",
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

	varInsertKey := _InsertKey{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varInsertKey)

	if err != nil {
		return err
	}

	*o = InsertKey(varInsertKey)

	return err
}

type NullableInsertKey struct {
	value *InsertKey
	isSet bool
}

func (v NullableInsertKey) Get() *InsertKey {
	return v.value
}

func (v *NullableInsertKey) Set(val *InsertKey) {
	v.value = val
	v.isSet = true
}

func (v NullableInsertKey) IsSet() bool {
	return v.isSet
}

func (v *NullableInsertKey) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableInsertKey(val *InsertKey) *NullableInsertKey {
	return &NullableInsertKey{value: val, isSet: true}
}

func (v NullableInsertKey) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableInsertKey) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


