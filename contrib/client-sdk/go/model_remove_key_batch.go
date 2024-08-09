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

// checks if the RemoveKeyBatch type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &RemoveKeyBatch{}

// RemoveKeyBatch Arguments for removing multiple keys
type RemoveKeyBatch struct {
	// Object key
	Keys []string `json:"keys"`
}

type _RemoveKeyBatch RemoveKeyBatch

// NewRemoveKeyBatch instantiates a new RemoveKeyBatch object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewRemoveKeyBatch(keys []string) *RemoveKeyBatch {
	this := RemoveKeyBatch{}
	this.Keys = keys
	return &this
}

// NewRemoveKeyBatchWithDefaults instantiates a new RemoveKeyBatch object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewRemoveKeyBatchWithDefaults() *RemoveKeyBatch {
	this := RemoveKeyBatch{}
	return &this
}

// GetKeys returns the Keys field value
func (o *RemoveKeyBatch) GetKeys() []string {
	if o == nil {
		var ret []string
		return ret
	}

	return o.Keys
}

// GetKeysOk returns a tuple with the Keys field value
// and a boolean to check if the value has been set.
func (o *RemoveKeyBatch) GetKeysOk() ([]string, bool) {
	if o == nil {
		return nil, false
	}
	return o.Keys, true
}

// SetKeys sets field value
func (o *RemoveKeyBatch) SetKeys(v []string) {
	o.Keys = v
}

func (o RemoveKeyBatch) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o RemoveKeyBatch) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	toSerialize["keys"] = o.Keys
	return toSerialize, nil
}

func (o *RemoveKeyBatch) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"keys",
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

	varRemoveKeyBatch := _RemoveKeyBatch{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varRemoveKeyBatch)

	if err != nil {
		return err
	}

	*o = RemoveKeyBatch(varRemoveKeyBatch)

	return err
}

type NullableRemoveKeyBatch struct {
	value *RemoveKeyBatch
	isSet bool
}

func (v NullableRemoveKeyBatch) Get() *RemoveKeyBatch {
	return v.value
}

func (v *NullableRemoveKeyBatch) Set(val *RemoveKeyBatch) {
	v.value = val
	v.isSet = true
}

func (v NullableRemoveKeyBatch) IsSet() bool {
	return v.isSet
}

func (v *NullableRemoveKeyBatch) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableRemoveKeyBatch(val *RemoveKeyBatch) *NullableRemoveKeyBatch {
	return &NullableRemoveKeyBatch{value: val, isSet: true}
}

func (v NullableRemoveKeyBatch) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableRemoveKeyBatch) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


