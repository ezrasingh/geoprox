/*
geoprox-server

Testing GeohashApiAPIService

*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech);

package geoprox_client

import (
	"context"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"testing"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID"
)

func Test_geoprox_client_GeohashApiAPIService(t *testing.T) {

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)

	t.Run("Test GeohashApiAPIService DecodeGeohash", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		var ghash string

		resp, httpRes, err := apiClient.GeohashApiAPI.DecodeGeohash(context.Background(), ghash).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test GeohashApiAPIService EncodeLatlng", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		resp, httpRes, err := apiClient.GeohashApiAPI.EncodeLatlng(context.Background()).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

	t.Run("Test GeohashApiAPIService GetNeighbors", func(t *testing.T) {

		t.Skip("skip test")  // remove to run test

		var ghash string

		resp, httpRes, err := apiClient.GeohashApiAPI.GetNeighbors(context.Background(), ghash).Execute()

		require.Nil(t, err)
		require.NotNil(t, resp)
		assert.Equal(t, 200, httpRes.StatusCode)

	})

}