/*
geoprox-server

Geoprox server implementation providing a HTTP API for geospatial queries and position tracking

API version: 0.5.0
Contact: singhezra@gmail.com
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package geoprox_client

import (
	"encoding/json"
	"bytes"
	"fmt"
)

// checks if the QueryRangeMany type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &QueryRangeMany{}

// QueryRangeMany Arguments for group range query
type QueryRangeMany struct {
	// Maximum number of neighbors that can be returned (default 100)
	Count NullableInt32 `json:"count,omitempty"`
	// List of indices to search
	Indices []string `json:"indices"`
	// Latitude
	Lat float64 `json:"lat"`
	// Longitude
	Lng float64 `json:"lng"`
	// Search radius in kilometers
	Range int32 `json:"range"`
	// If enabled neighbors will be sorted by distance, nearest to furthest (default false)
	Sorted NullableBool `json:"sorted,omitempty"`
}

type _QueryRangeMany QueryRangeMany

// NewQueryRangeMany instantiates a new QueryRangeMany object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewQueryRangeMany(indices []string, lat float64, lng float64, range_ int32) *QueryRangeMany {
	this := QueryRangeMany{}
	this.Indices = indices
	this.Lat = lat
	this.Lng = lng
	this.Range = range_
	return &this
}

// NewQueryRangeManyWithDefaults instantiates a new QueryRangeMany object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewQueryRangeManyWithDefaults() *QueryRangeMany {
	this := QueryRangeMany{}
	return &this
}

// GetCount returns the Count field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *QueryRangeMany) GetCount() int32 {
	if o == nil || IsNil(o.Count.Get()) {
		var ret int32
		return ret
	}
	return *o.Count.Get()
}

// GetCountOk returns a tuple with the Count field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *QueryRangeMany) GetCountOk() (*int32, bool) {
	if o == nil {
		return nil, false
	}
	return o.Count.Get(), o.Count.IsSet()
}

// HasCount returns a boolean if a field has been set.
func (o *QueryRangeMany) HasCount() bool {
	if o != nil && o.Count.IsSet() {
		return true
	}

	return false
}

// SetCount gets a reference to the given NullableInt32 and assigns it to the Count field.
func (o *QueryRangeMany) SetCount(v int32) {
	o.Count.Set(&v)
}
// SetCountNil sets the value for Count to be an explicit nil
func (o *QueryRangeMany) SetCountNil() {
	o.Count.Set(nil)
}

// UnsetCount ensures that no value is present for Count, not even an explicit nil
func (o *QueryRangeMany) UnsetCount() {
	o.Count.Unset()
}

// GetIndices returns the Indices field value
func (o *QueryRangeMany) GetIndices() []string {
	if o == nil {
		var ret []string
		return ret
	}

	return o.Indices
}

// GetIndicesOk returns a tuple with the Indices field value
// and a boolean to check if the value has been set.
func (o *QueryRangeMany) GetIndicesOk() ([]string, bool) {
	if o == nil {
		return nil, false
	}
	return o.Indices, true
}

// SetIndices sets field value
func (o *QueryRangeMany) SetIndices(v []string) {
	o.Indices = v
}

// GetLat returns the Lat field value
func (o *QueryRangeMany) GetLat() float64 {
	if o == nil {
		var ret float64
		return ret
	}

	return o.Lat
}

// GetLatOk returns a tuple with the Lat field value
// and a boolean to check if the value has been set.
func (o *QueryRangeMany) GetLatOk() (*float64, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Lat, true
}

// SetLat sets field value
func (o *QueryRangeMany) SetLat(v float64) {
	o.Lat = v
}

// GetLng returns the Lng field value
func (o *QueryRangeMany) GetLng() float64 {
	if o == nil {
		var ret float64
		return ret
	}

	return o.Lng
}

// GetLngOk returns a tuple with the Lng field value
// and a boolean to check if the value has been set.
func (o *QueryRangeMany) GetLngOk() (*float64, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Lng, true
}

// SetLng sets field value
func (o *QueryRangeMany) SetLng(v float64) {
	o.Lng = v
}

// GetRange returns the Range field value
func (o *QueryRangeMany) GetRange() int32 {
	if o == nil {
		var ret int32
		return ret
	}

	return o.Range
}

// GetRangeOk returns a tuple with the Range field value
// and a boolean to check if the value has been set.
func (o *QueryRangeMany) GetRangeOk() (*int32, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Range, true
}

// SetRange sets field value
func (o *QueryRangeMany) SetRange(v int32) {
	o.Range = v
}

// GetSorted returns the Sorted field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *QueryRangeMany) GetSorted() bool {
	if o == nil || IsNil(o.Sorted.Get()) {
		var ret bool
		return ret
	}
	return *o.Sorted.Get()
}

// GetSortedOk returns a tuple with the Sorted field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *QueryRangeMany) GetSortedOk() (*bool, bool) {
	if o == nil {
		return nil, false
	}
	return o.Sorted.Get(), o.Sorted.IsSet()
}

// HasSorted returns a boolean if a field has been set.
func (o *QueryRangeMany) HasSorted() bool {
	if o != nil && o.Sorted.IsSet() {
		return true
	}

	return false
}

// SetSorted gets a reference to the given NullableBool and assigns it to the Sorted field.
func (o *QueryRangeMany) SetSorted(v bool) {
	o.Sorted.Set(&v)
}
// SetSortedNil sets the value for Sorted to be an explicit nil
func (o *QueryRangeMany) SetSortedNil() {
	o.Sorted.Set(nil)
}

// UnsetSorted ensures that no value is present for Sorted, not even an explicit nil
func (o *QueryRangeMany) UnsetSorted() {
	o.Sorted.Unset()
}

func (o QueryRangeMany) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o QueryRangeMany) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if o.Count.IsSet() {
		toSerialize["count"] = o.Count.Get()
	}
	toSerialize["indices"] = o.Indices
	toSerialize["lat"] = o.Lat
	toSerialize["lng"] = o.Lng
	toSerialize["range"] = o.Range
	if o.Sorted.IsSet() {
		toSerialize["sorted"] = o.Sorted.Get()
	}
	return toSerialize, nil
}

func (o *QueryRangeMany) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"indices",
		"lat",
		"lng",
		"range",
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

	varQueryRangeMany := _QueryRangeMany{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varQueryRangeMany)

	if err != nil {
		return err
	}

	*o = QueryRangeMany(varQueryRangeMany)

	return err
}

type NullableQueryRangeMany struct {
	value *QueryRangeMany
	isSet bool
}

func (v NullableQueryRangeMany) Get() *QueryRangeMany {
	return v.value
}

func (v *NullableQueryRangeMany) Set(val *QueryRangeMany) {
	v.value = val
	v.isSet = true
}

func (v NullableQueryRangeMany) IsSet() bool {
	return v.isSet
}

func (v *NullableQueryRangeMany) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableQueryRangeMany(val *QueryRangeMany) *NullableQueryRangeMany {
	return &NullableQueryRangeMany{value: val, isSet: true}
}

func (v NullableQueryRangeMany) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableQueryRangeMany) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


