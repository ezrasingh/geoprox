# QueryRangeManyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Errors** | **map[string]string** | Contains information about any errors occured during batch search. | 
**Results** | [**map[string][]Neighbor**](array.md) | Object keys found within range | 

## Methods

### NewQueryRangeManyResponse

`func NewQueryRangeManyResponse(errors map[string]string, results map[string][]Neighbor, ) *QueryRangeManyResponse`

NewQueryRangeManyResponse instantiates a new QueryRangeManyResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewQueryRangeManyResponseWithDefaults

`func NewQueryRangeManyResponseWithDefaults() *QueryRangeManyResponse`

NewQueryRangeManyResponseWithDefaults instantiates a new QueryRangeManyResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetErrors

`func (o *QueryRangeManyResponse) GetErrors() map[string]string`

GetErrors returns the Errors field if non-nil, zero value otherwise.

### GetErrorsOk

`func (o *QueryRangeManyResponse) GetErrorsOk() (*map[string]string, bool)`

GetErrorsOk returns a tuple with the Errors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrors

`func (o *QueryRangeManyResponse) SetErrors(v map[string]string)`

SetErrors sets Errors field to given value.


### GetResults

`func (o *QueryRangeManyResponse) GetResults() map[string][]Neighbor`

GetResults returns the Results field if non-nil, zero value otherwise.

### GetResultsOk

`func (o *QueryRangeManyResponse) GetResultsOk() (*map[string][]Neighbor, bool)`

GetResultsOk returns a tuple with the Results field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResults

`func (o *QueryRangeManyResponse) SetResults(v map[string][]Neighbor)`

SetResults sets Results field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


