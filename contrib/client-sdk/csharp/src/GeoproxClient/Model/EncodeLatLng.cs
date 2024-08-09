/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.1
 * Contact: singhezra@gmail.com
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */


using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.IO;
using System.Runtime.Serialization;
using System.Text;
using System.Text.RegularExpressions;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using System.ComponentModel.DataAnnotations;
using OpenAPIDateConverter = GeoproxClient.Client.OpenAPIDateConverter;

namespace GeoproxClient.Model
{
    /// <summary>
    /// Arguments for encoding latitude/longitude as geohash
    /// </summary>
    [DataContract(Name = "EncodeLatLng")]
    public partial class EncodeLatLng : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="EncodeLatLng" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected EncodeLatLng() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="EncodeLatLng" /> class.
        /// </summary>
        /// <param name="depth">Determines geohash length (required).</param>
        /// <param name="lat">Latitude (required).</param>
        /// <param name="lng">Longitude (required).</param>
        public EncodeLatLng(int depth = default(int), double lat = default(double), double lng = default(double))
        {
            this.Depth = depth;
            this.Lat = lat;
            this.Lng = lng;
        }

        /// <summary>
        /// Determines geohash length
        /// </summary>
        /// <value>Determines geohash length</value>
        [DataMember(Name = "depth", IsRequired = true, EmitDefaultValue = true)]
        public int Depth { get; set; }

        /// <summary>
        /// Latitude
        /// </summary>
        /// <value>Latitude</value>
        [DataMember(Name = "lat", IsRequired = true, EmitDefaultValue = true)]
        public double Lat { get; set; }

        /// <summary>
        /// Longitude
        /// </summary>
        /// <value>Longitude</value>
        [DataMember(Name = "lng", IsRequired = true, EmitDefaultValue = true)]
        public double Lng { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class EncodeLatLng {\n");
            sb.Append("  Depth: ").Append(Depth).Append("\n");
            sb.Append("  Lat: ").Append(Lat).Append("\n");
            sb.Append("  Lng: ").Append(Lng).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public virtual string ToJson()
        {
            return Newtonsoft.Json.JsonConvert.SerializeObject(this, Newtonsoft.Json.Formatting.Indented);
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {
            // Depth (int) maximum
            if (this.Depth > (int)10)
            {
                yield return new ValidationResult("Invalid value for Depth, must be a value less than or equal to 10.", new [] { "Depth" });
            }

            // Depth (int) minimum
            if (this.Depth < (int)1)
            {
                yield return new ValidationResult("Invalid value for Depth, must be a value greater than or equal to 1.", new [] { "Depth" });
            }

            yield break;
        }
    }

}
