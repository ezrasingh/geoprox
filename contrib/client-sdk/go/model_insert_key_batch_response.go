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

// checks if the InsertKeyBatchResponse type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &InsertKeyBatchResponse{}

// InsertKeyBatchResponse Returns results and errors of batch key insert
type InsertKeyBatchResponse struct {
	// Contains information about which keys failed to be inserted and the associated error details.
	Errors map[string]string `json:"errors"`
	// Object keys that have been inserted in the index and their geohash.
	Results map[string]string `json:"results"`
}

type _InsertKeyBatchResponse InsertKeyBatchResponse

// NewInsertKeyBatchResponse instantiates a new InsertKeyBatchResponse object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewInsertKeyBatchResponse(errors map[string]string, results map[string]string) *InsertKeyBatchResponse {
	this := InsertKeyBatchResponse{}
	this.Errors = errors
	this.Results = results
	return &this
}

// NewInsertKeyBatchResponseWithDefaults instantiates a new InsertKeyBatchResponse object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewInsertKeyBatchResponseWithDefaults() *InsertKeyBatchResponse {
	this := InsertKeyBatchResponse{}
	return &this
}

// GetErrors returns the Errors field value
func (o *InsertKeyBatchResponse) GetErrors() map[string]string {
	if o == nil {
		var ret map[string]string
		return ret
	}

	return o.Errors
}

// GetErrorsOk returns a tuple with the Errors field value
// and a boolean to check if the value has been set.
func (o *InsertKeyBatchResponse) GetErrorsOk() (*map[string]string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Errors, true
}

// SetErrors sets field value
func (o *InsertKeyBatchResponse) SetErrors(v map[string]string) {
	o.Errors = v
}

// GetResults returns the Results field value
func (o *InsertKeyBatchResponse) GetResults() map[string]string {
	if o == nil {
		var ret map[string]string
		return ret
	}

	return o.Results
}

// GetResultsOk returns a tuple with the Results field value
// and a boolean to check if the value has been set.
func (o *InsertKeyBatchResponse) GetResultsOk() (*map[string]string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Results, true
}

// SetResults sets field value
func (o *InsertKeyBatchResponse) SetResults(v map[string]string) {
	o.Results = v
}

func (o InsertKeyBatchResponse) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o InsertKeyBatchResponse) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	toSerialize["errors"] = o.Errors
	toSerialize["results"] = o.Results
	return toSerialize, nil
}

func (o *InsertKeyBatchResponse) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"errors",
		"results",
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

	varInsertKeyBatchResponse := _InsertKeyBatchResponse{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varInsertKeyBatchResponse)

	if err != nil {
		return err
	}

	*o = InsertKeyBatchResponse(varInsertKeyBatchResponse)

	return err
}

type NullableInsertKeyBatchResponse struct {
	value *InsertKeyBatchResponse
	isSet bool
}

func (v NullableInsertKeyBatchResponse) Get() *InsertKeyBatchResponse {
	return v.value
}

func (v *NullableInsertKeyBatchResponse) Set(val *InsertKeyBatchResponse) {
	v.value = val
	v.isSet = true
}

func (v NullableInsertKeyBatchResponse) IsSet() bool {
	return v.isSet
}

func (v *NullableInsertKeyBatchResponse) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableInsertKeyBatchResponse(val *InsertKeyBatchResponse) *NullableInsertKeyBatchResponse {
	return &NullableInsertKeyBatchResponse{value: val, isSet: true}
}

func (v NullableInsertKeyBatchResponse) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableInsertKeyBatchResponse) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


