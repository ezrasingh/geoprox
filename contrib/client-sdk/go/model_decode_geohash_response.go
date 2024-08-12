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

// checks if the DecodeGeohashResponse type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &DecodeGeohashResponse{}

// DecodeGeohashResponse Returns geohash decoded as latitude/longitude with precision errors
type DecodeGeohashResponse struct {
	// Latitude
	Lat float64 `json:"lat"`
	// Latitude error
	LatError float64 `json:"lat_error"`
	// Longitude
	Lng float64 `json:"lng"`
	// Longitude error
	LngError float64 `json:"lng_error"`
}

type _DecodeGeohashResponse DecodeGeohashResponse

// NewDecodeGeohashResponse instantiates a new DecodeGeohashResponse object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewDecodeGeohashResponse(lat float64, latError float64, lng float64, lngError float64) *DecodeGeohashResponse {
	this := DecodeGeohashResponse{}
	this.Lat = lat
	this.LatError = latError
	this.Lng = lng
	this.LngError = lngError
	return &this
}

// NewDecodeGeohashResponseWithDefaults instantiates a new DecodeGeohashResponse object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewDecodeGeohashResponseWithDefaults() *DecodeGeohashResponse {
	this := DecodeGeohashResponse{}
	return &this
}

// GetLat returns the Lat field value
func (o *DecodeGeohashResponse) GetLat() float64 {
	if o == nil {
		var ret float64
		return ret
	}

	return o.Lat
}

// GetLatOk returns a tuple with the Lat field value
// and a boolean to check if the value has been set.
func (o *DecodeGeohashResponse) GetLatOk() (*float64, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Lat, true
}

// SetLat sets field value
func (o *DecodeGeohashResponse) SetLat(v float64) {
	o.Lat = v
}

// GetLatError returns the LatError field value
func (o *DecodeGeohashResponse) GetLatError() float64 {
	if o == nil {
		var ret float64
		return ret
	}

	return o.LatError
}

// GetLatErrorOk returns a tuple with the LatError field value
// and a boolean to check if the value has been set.
func (o *DecodeGeohashResponse) GetLatErrorOk() (*float64, bool) {
	if o == nil {
		return nil, false
	}
	return &o.LatError, true
}

// SetLatError sets field value
func (o *DecodeGeohashResponse) SetLatError(v float64) {
	o.LatError = v
}

// GetLng returns the Lng field value
func (o *DecodeGeohashResponse) GetLng() float64 {
	if o == nil {
		var ret float64
		return ret
	}

	return o.Lng
}

// GetLngOk returns a tuple with the Lng field value
// and a boolean to check if the value has been set.
func (o *DecodeGeohashResponse) GetLngOk() (*float64, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Lng, true
}

// SetLng sets field value
func (o *DecodeGeohashResponse) SetLng(v float64) {
	o.Lng = v
}

// GetLngError returns the LngError field value
func (o *DecodeGeohashResponse) GetLngError() float64 {
	if o == nil {
		var ret float64
		return ret
	}

	return o.LngError
}

// GetLngErrorOk returns a tuple with the LngError field value
// and a boolean to check if the value has been set.
func (o *DecodeGeohashResponse) GetLngErrorOk() (*float64, bool) {
	if o == nil {
		return nil, false
	}
	return &o.LngError, true
}

// SetLngError sets field value
func (o *DecodeGeohashResponse) SetLngError(v float64) {
	o.LngError = v
}

func (o DecodeGeohashResponse) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o DecodeGeohashResponse) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	toSerialize["lat"] = o.Lat
	toSerialize["lat_error"] = o.LatError
	toSerialize["lng"] = o.Lng
	toSerialize["lng_error"] = o.LngError
	return toSerialize, nil
}

func (o *DecodeGeohashResponse) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"lat",
		"lat_error",
		"lng",
		"lng_error",
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

	varDecodeGeohashResponse := _DecodeGeohashResponse{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varDecodeGeohashResponse)

	if err != nil {
		return err
	}

	*o = DecodeGeohashResponse(varDecodeGeohashResponse)

	return err
}

type NullableDecodeGeohashResponse struct {
	value *DecodeGeohashResponse
	isSet bool
}

func (v NullableDecodeGeohashResponse) Get() *DecodeGeohashResponse {
	return v.value
}

func (v *NullableDecodeGeohashResponse) Set(val *DecodeGeohashResponse) {
	v.value = val
	v.isSet = true
}

func (v NullableDecodeGeohashResponse) IsSet() bool {
	return v.isSet
}

func (v *NullableDecodeGeohashResponse) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableDecodeGeohashResponse(val *DecodeGeohashResponse) *NullableDecodeGeohashResponse {
	return &NullableDecodeGeohashResponse{value: val, isSet: true}
}

func (v NullableDecodeGeohashResponse) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableDecodeGeohashResponse) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


