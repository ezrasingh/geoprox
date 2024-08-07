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
    ///  Class for testing GeoshardApiApi
    /// </summary>
    /// <remarks>
    /// This file is automatically generated by OpenAPI Generator (https://openapi-generator.tech).
    /// Please update the test case below to test the API endpoint.
    /// </remarks>
    public class GeoshardApiApiTests : IDisposable
    {
        private GeoshardApiApi instance;

        public GeoshardApiApiTests()
        {
            instance = new GeoshardApiApi();
        }

        public void Dispose()
        {
            // Cleanup when everything is done.
        }

        /// <summary>
        /// Test an instance of GeoshardApiApi
        /// </summary>
        [Fact]
        public void InstanceTest()
        {
            // TODO uncomment below to test 'IsType' GeoshardApiApi
            //Assert.IsType<GeoshardApiApi>(instance);
        }

        /// <summary>
        /// Test CreateIndex
        /// </summary>
        [Fact]
        public void CreateIndexTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //string index = null;
            //var response = instance.CreateIndex(index);
            //Assert.IsType<CreateIndexResponse>(response);
        }

        /// <summary>
        /// Test DropIndex
        /// </summary>
        [Fact]
        public void DropIndexTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //string index = null;
            //var response = instance.DropIndex(index);
            //Assert.IsType<DropIndexResponse>(response);
        }

        /// <summary>
        /// Test InsertKey
        /// </summary>
        [Fact]
        public void InsertKeyTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //string index = null;
            //InsertKey insertKey = null;
            //var response = instance.InsertKey(index, insertKey);
            //Assert.IsType<InsertKeyResponse>(response);
        }

        /// <summary>
        /// Test InsertKeyBatch
        /// </summary>
        [Fact]
        public void InsertKeyBatchTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //string index = null;
            //InsertKeyBatch insertKeyBatch = null;
            //var response = instance.InsertKeyBatch(index, insertKeyBatch);
            //Assert.IsType<InsertKeyBatchResponse>(response);
        }

        /// <summary>
        /// Test QueryRange
        /// </summary>
        [Fact]
        public void QueryRangeTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //string index = null;
            //double lat = null;
            //double lng = null;
            //int range = null;
            //int? count = null;
            //bool? sorted = null;
            //var response = instance.QueryRange(index, lat, lng, range, count, sorted);
            //Assert.IsType<QueryRangeResponse>(response);
        }

        /// <summary>
        /// Test QueryRangeMany
        /// </summary>
        [Fact]
        public void QueryRangeManyTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //List<string> indices = null;
            //double lat = null;
            //double lng = null;
            //int range = null;
            //int? count = null;
            //bool? sorted = null;
            //var response = instance.QueryRangeMany(indices, lat, lng, range, count, sorted);
            //Assert.IsType<QueryRangeManyResponse>(response);
        }

        /// <summary>
        /// Test RemoveKey
        /// </summary>
        [Fact]
        public void RemoveKeyTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //string index = null;
            //RemoveKey removeKey = null;
            //var response = instance.RemoveKey(index, removeKey);
            //Assert.IsType<RemoveKeyResponse>(response);
        }

        /// <summary>
        /// Test RemoveKeyBatch
        /// </summary>
        [Fact]
        public void RemoveKeyBatchTest()
        {
            // TODO uncomment below to test the method and replace null with proper value
            //string index = null;
            //RemoveKeyBatch removeKeyBatch = null;
            //var response = instance.RemoveKeyBatch(index, removeKeyBatch);
            //Assert.IsType<RemoveKeyBatchResponse>(response);
        }
    }
}
