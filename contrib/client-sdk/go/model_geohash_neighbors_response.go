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

// checks if the GeohashNeighborsResponse type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &GeohashNeighborsResponse{}

// GeohashNeighborsResponse Neighboring geohash regions
type GeohashNeighborsResponse struct {
	// East
	E string `json:"e"`
	// North
	N string `json:"n"`
	// North East
	Ne string `json:"ne"`
	// North West
	Nw string `json:"nw"`
	// South
	S string `json:"s"`
	// South East
	Se string `json:"se"`
	// South West
	Sw string `json:"sw"`
	// West
	W string `json:"w"`
}

type _GeohashNeighborsResponse GeohashNeighborsResponse

// NewGeohashNeighborsResponse instantiates a new GeohashNeighborsResponse object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewGeohashNeighborsResponse(e string, n string, ne string, nw string, s string, se string, sw string, w string) *GeohashNeighborsResponse {
	this := GeohashNeighborsResponse{}
	this.E = e
	this.N = n
	this.Ne = ne
	this.Nw = nw
	this.S = s
	this.Se = se
	this.Sw = sw
	this.W = w
	return &this
}

// NewGeohashNeighborsResponseWithDefaults instantiates a new GeohashNeighborsResponse object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewGeohashNeighborsResponseWithDefaults() *GeohashNeighborsResponse {
	this := GeohashNeighborsResponse{}
	return &this
}

// GetE returns the E field value
func (o *GeohashNeighborsResponse) GetE() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.E
}

// GetEOk returns a tuple with the E field value
// and a boolean to check if the value has been set.
func (o *GeohashNeighborsResponse) GetEOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.E, true
}

// SetE sets field value
func (o *GeohashNeighborsResponse) SetE(v string) {
	o.E = v
}

// GetN returns the N field value
func (o *GeohashNeighborsResponse) GetN() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.N
}

// GetNOk returns a tuple with the N field value
// and a boolean to check if the value has been set.
func (o *GeohashNeighborsResponse) GetNOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.N, true
}

// SetN sets field value
func (o *GeohashNeighborsResponse) SetN(v string) {
	o.N = v
}

// GetNe returns the Ne field value
func (o *GeohashNeighborsResponse) GetNe() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Ne
}

// GetNeOk returns a tuple with the Ne field value
// and a boolean to check if the value has been set.
func (o *GeohashNeighborsResponse) GetNeOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Ne, true
}

// SetNe sets field value
func (o *GeohashNeighborsResponse) SetNe(v string) {
	o.Ne = v
}

// GetNw returns the Nw field value
func (o *GeohashNeighborsResponse) GetNw() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Nw
}

// GetNwOk returns a tuple with the Nw field value
// and a boolean to check if the value has been set.
func (o *GeohashNeighborsResponse) GetNwOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Nw, true
}

// SetNw sets field value
func (o *GeohashNeighborsResponse) SetNw(v string) {
	o.Nw = v
}

// GetS returns the S field value
func (o *GeohashNeighborsResponse) GetS() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.S
}

// GetSOk returns a tuple with the S field value
// and a boolean to check if the value has been set.
func (o *GeohashNeighborsResponse) GetSOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.S, true
}

// SetS sets field value
func (o *GeohashNeighborsResponse) SetS(v string) {
	o.S = v
}

// GetSe returns the Se field value
func (o *GeohashNeighborsResponse) GetSe() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Se
}

// GetSeOk returns a tuple with the Se field value
// and a boolean to check if the value has been set.
func (o *GeohashNeighborsResponse) GetSeOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Se, true
}

// SetSe sets field value
func (o *GeohashNeighborsResponse) SetSe(v string) {
	o.Se = v
}

// GetSw returns the Sw field value
func (o *GeohashNeighborsResponse) GetSw() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Sw
}

// GetSwOk returns a tuple with the Sw field value
// and a boolean to check if the value has been set.
func (o *GeohashNeighborsResponse) GetSwOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Sw, true
}

// SetSw sets field value
func (o *GeohashNeighborsResponse) SetSw(v string) {
	o.Sw = v
}

// GetW returns the W field value
func (o *GeohashNeighborsResponse) GetW() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.W
}

// GetWOk returns a tuple with the W field value
// and a boolean to check if the value has been set.
func (o *GeohashNeighborsResponse) GetWOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.W, true
}

// SetW sets field value
func (o *GeohashNeighborsResponse) SetW(v string) {
	o.W = v
}

func (o GeohashNeighborsResponse) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o GeohashNeighborsResponse) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	toSerialize["e"] = o.E
	toSerialize["n"] = o.N
	toSerialize["ne"] = o.Ne
	toSerialize["nw"] = o.Nw
	toSerialize["s"] = o.S
	toSerialize["se"] = o.Se
	toSerialize["sw"] = o.Sw
	toSerialize["w"] = o.W
	return toSerialize, nil
}

func (o *GeohashNeighborsResponse) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"e",
		"n",
		"ne",
		"nw",
		"s",
		"se",
		"sw",
		"w",
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

	varGeohashNeighborsResponse := _GeohashNeighborsResponse{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varGeohashNeighborsResponse)

	if err != nil {
		return err
	}

	*o = GeohashNeighborsResponse(varGeohashNeighborsResponse)

	return err
}

type NullableGeohashNeighborsResponse struct {
	value *GeohashNeighborsResponse
	isSet bool
}

func (v NullableGeohashNeighborsResponse) Get() *GeohashNeighborsResponse {
	return v.value
}

func (v *NullableGeohashNeighborsResponse) Set(val *GeohashNeighborsResponse) {
	v.value = val
	v.isSet = true
}

func (v NullableGeohashNeighborsResponse) IsSet() bool {
	return v.isSet
}

func (v *NullableGeohashNeighborsResponse) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableGeohashNeighborsResponse(val *GeohashNeighborsResponse) *NullableGeohashNeighborsResponse {
	return &NullableGeohashNeighborsResponse{value: val, isSet: true}
}

func (v NullableGeohashNeighborsResponse) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableGeohashNeighborsResponse) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


