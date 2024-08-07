/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.0
 * Contact: singhezra@gmail.com
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */

using System;
using System.IO;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Reflection;
using RestSharp;
using Xunit;

using GeoproxClient.Client;
using GeoproxClient.Api;
// uncomment below to import models
//using GeoproxClient.Model;

namespace GeoproxClient.Test.Api
{
    /// <summary>
    ///  Class for testing GeohashApiApi
    /// </summary>
    /// <remarks>
    /// This file is automatically generated by OpenAPI Generator (https://openapi-generator.tech).
    /// Please update the test case below to test the API endpoint.
    /// </remarks>
    public class GeohashApiApiTests : IDisposable
    {
        private GeohashApiApi instance;

        public GeohashApiApiTests()
        {
            instance = new GeohashApiApi();
        }

        public void Dispose()
        {
            // Cleanup when everything is done.
        }

        /// <summary>
        /// Test an instance of GeohashApiApi
        /// </summary>
        [Fact]
        public void InstanceTest()
        {
            // TODO uncomment below to test 'IsType' GeohashApiApi
            //Assert.IsType<GeohashApiApi>(instance);
        }

        /// <summary>
        /// Test DecodeGeohash
        /// </summary>
        [Fact]
        public void DecodeGeohashTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //string ghash = null;
            //var response = instance.DecodeGeohash(ghash);
            //Assert.IsType<DecodeGeohashResponse>(response);
        }

        /// <summary>
        /// Test EncodeLatlng
        /// </summary>
        [Fact]
        public void EncodeLatlngTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //double lat = null;
            //double lng = null;
            //int depth = null;
            //var response = instance.EncodeLatlng(lat, lng, depth);
            //Assert.IsType<EncodeLatLngResponse>(response);
        }

        /// <summary>
        /// Test GetNeighbors
        /// </summary>
        [Fact]
        public void GetNeighborsTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //string ghash = null;
            //var response = instance.GetNeighbors(ghash);
            //Assert.IsType<GeohashNeighborsResponse>(response);
        }
    }
}
