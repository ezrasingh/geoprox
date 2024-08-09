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

// checks if the QueryRangeManyResponse type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &QueryRangeManyResponse{}

// QueryRangeManyResponse Returns indices and object keys found with their distance
type QueryRangeManyResponse struct {
	// Contains information about any errors occured during batch search.
	Errors map[string]string `json:"errors"`
	// Object keys found within range
	Results map[string][]Neighbor `json:"results"`
}

type _QueryRangeManyResponse QueryRangeManyResponse

// NewQueryRangeManyResponse instantiates a new QueryRangeManyResponse object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewQueryRangeManyResponse(errors map[string]string, results map[string][]Neighbor) *QueryRangeManyResponse {
	this := QueryRangeManyResponse{}
	this.Errors = errors
	this.Results = results
	return &this
}

// NewQueryRangeManyResponseWithDefaults instantiates a new QueryRangeManyResponse object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewQueryRangeManyResponseWithDefaults() *QueryRangeManyResponse {
	this := QueryRangeManyResponse{}
	return &this
}

// GetErrors returns the Errors field value
func (o *QueryRangeManyResponse) GetErrors() map[string]string {
	if o == nil {
		var ret map[string]string
		return ret
	}

	return o.Errors
}

// GetErrorsOk returns a tuple with the Errors field value
// and a boolean to check if the value has been set.
func (o *QueryRangeManyResponse) GetErrorsOk() (*map[string]string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Errors, true
}

// SetErrors sets field value
func (o *QueryRangeManyResponse) SetErrors(v map[string]string) {
	o.Errors = v
}

// GetResults returns the Results field value
func (o *QueryRangeManyResponse) GetResults() map[string][]Neighbor {
	if o == nil {
		var ret map[string][]Neighbor
		return ret
	}

	return o.Results
}

// GetResultsOk returns a tuple with the Results field value
// and a boolean to check if the value has been set.
func (o *QueryRangeManyResponse) GetResultsOk() (*map[string][]Neighbor, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Results, true
}

// SetResults sets field value
func (o *QueryRangeManyResponse) SetResults(v map[string][]Neighbor) {
	o.Results = v
}

func (o QueryRangeManyResponse) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o QueryRangeManyResponse) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	toSerialize["errors"] = o.Errors
	toSerialize["results"] = o.Results
	return toSerialize, nil
}

func (o *QueryRangeManyResponse) UnmarshalJSON(data []byte) (err error) {
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

	varQueryRangeManyResponse := _QueryRangeManyResponse{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varQueryRangeManyResponse)

	if err != nil {
		return err
	}

	*o = QueryRangeManyResponse(varQueryRangeManyResponse)

	return err
}

type NullableQueryRangeManyResponse struct {
	value *QueryRangeManyResponse
	isSet bool
}

func (v NullableQueryRangeManyResponse) Get() *QueryRangeManyResponse {
	return v.value
}

func (v *NullableQueryRangeManyResponse) Set(val *QueryRangeManyResponse) {
	v.value = val
	v.isSet = true
}

func (v NullableQueryRangeManyResponse) IsSet() bool {
	return v.isSet
}

func (v *NullableQueryRangeManyResponse) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableQueryRangeManyResponse(val *QueryRangeManyResponse) *NullableQueryRangeManyResponse {
	return &NullableQueryRangeManyResponse{value: val, isSet: true}
}

func (v NullableQueryRangeManyResponse) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableQueryRangeManyResponse) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


