/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.0
 * Contact: singhezra@gmail.com
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */


using Xunit;

using System;
using System.Linq;
using System.IO;
using System.Collections.Generic;
using GeoproxClient.Model;
using GeoproxClient.Client;
using System.Reflection;
using Newtonsoft.Json;

namespace GeoproxClient.Test.Model
{
    /// <summary>
    ///  Class for testing DecodeGeohashResponse
    /// </summary>
    /// <remarks>
    /// This file is automatically generated by OpenAPI Generator (https://openapi-generator.tech).
    /// Please update the test case below to test the model.
    /// </remarks>
    public class DecodeGeohashResponseTests : IDisposable
    {
        // TODO uncomment below to declare an instance variable for DecodeGeohashResponse
        //private DecodeGeohashResponse instance;

        public DecodeGeohashResponseTests()
        {
            // TODO uncomment below to create an instance of DecodeGeohashResponse
            //instance = new DecodeGeohashResponse();
        }

        public void Dispose()
        {
            // Cleanup when everything is done.
        }

        /// <summary>
        /// Test an instance of DecodeGeohashResponse
        /// </summary>
        [Fact]
        public void DecodeGeohashResponseInstanceTest()
        {
            // TODO uncomment below to test "IsType" DecodeGeohashResponse
            //Assert.IsType<DecodeGeohashResponse>(instance);
        }

        /// <summary>
        /// Test the property 'Lat'
        /// </summary>
        [Fact]
        public void LatTest()
        {
            // TODO unit test for the property 'Lat'
        }

        /// <summary>
        /// Test the property 'LatError'
        /// </summary>
        [Fact]
        public void LatErrorTest()
        {
            // TODO unit test for the property 'LatError'
        }

        /// <summary>
        /// Test the property 'Lng'
        /// </summary>
        [Fact]
        public void LngTest()
        {
            // TODO unit test for the property 'Lng'
        }

        /// <summary>
        /// Test the property 'LngError'
        /// </summary>
        [Fact]
        public void LngErrorTest()
        {
            // TODO unit test for the property 'LngError'
        }
    }
}
