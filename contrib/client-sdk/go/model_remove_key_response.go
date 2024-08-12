/*
geoprox-server

Geoprox server implementation providing a HTTP API for geospatial queries and position tracking

API version: 0.4.2
Contact: singhezra@gmail.com
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package geoprox_client

import (
	"encoding/json"
	"bytes"
	"fmt"
)

// checks if the RemoveKeyResponse type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &RemoveKeyResponse{}

// RemoveKeyResponse Returns key and deletion status
type RemoveKeyResponse struct {
	// If true key was removed
	Deleted bool `json:"deleted"`
	// Object key
	Key string `json:"key"`
}

type _RemoveKeyResponse RemoveKeyResponse

// NewRemoveKeyResponse instantiates a new RemoveKeyResponse object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewRemoveKeyResponse(deleted bool, key string) *RemoveKeyResponse {
	this := RemoveKeyResponse{}
	this.Deleted = deleted
	this.Key = key
	return &this
}

// NewRemoveKeyResponseWithDefaults instantiates a new RemoveKeyResponse object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewRemoveKeyResponseWithDefaults() *RemoveKeyResponse {
	this := RemoveKeyResponse{}
	return &this
}

// GetDeleted returns the Deleted field value
func (o *RemoveKeyResponse) GetDeleted() bool {
	if o == nil {
		var ret bool
		return ret
	}

	return o.Deleted
}

// GetDeletedOk returns a tuple with the Deleted field value
// and a boolean to check if the value has been set.
func (o *RemoveKeyResponse) GetDeletedOk() (*bool, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Deleted, true
}

// SetDeleted sets field value
func (o *RemoveKeyResponse) SetDeleted(v bool) {
	o.Deleted = v
}

// GetKey returns the Key field value
func (o *RemoveKeyResponse) GetKey() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Key
}

// GetKeyOk returns a tuple with the Key field value
// and a boolean to check if the value has been set.
func (o *RemoveKeyResponse) GetKeyOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Key, true
}

// SetKey sets field value
func (o *RemoveKeyResponse) SetKey(v string) {
	o.Key = v
}

func (o RemoveKeyResponse) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o RemoveKeyResponse) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	toSerialize["deleted"] = o.Deleted
	toSerialize["key"] = o.Key
	return toSerialize, nil
}

func (o *RemoveKeyResponse) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"deleted",
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

	varRemoveKeyResponse := _RemoveKeyResponse{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varRemoveKeyResponse)

	if err != nil {
		return err
	}

	*o = RemoveKeyResponse(varRemoveKeyResponse)

	return err
}

type NullableRemoveKeyResponse struct {
	value *RemoveKeyResponse
	isSet bool
}

func (v NullableRemoveKeyResponse) Get() *RemoveKeyResponse {
	return v.value
}

func (v *NullableRemoveKeyResponse) Set(val *RemoveKeyResponse) {
	v.value = val
	v.isSet = true
}

func (v NullableRemoveKeyResponse) IsSet() bool {
	return v.isSet
}

func (v *NullableRemoveKeyResponse) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableRemoveKeyResponse(val *RemoveKeyResponse) *NullableRemoveKeyResponse {
	return &NullableRemoveKeyResponse{value: val, isSet: true}
}

func (v NullableRemoveKeyResponse) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableRemoveKeyResponse) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


