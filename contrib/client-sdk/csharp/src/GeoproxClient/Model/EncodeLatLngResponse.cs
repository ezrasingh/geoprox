/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.5.0
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
    /// Returns geohash encoded latitude/longitude
    /// </summary>
    [DataContract(Name = "EncodeLatLngResponse")]
    public partial class EncodeLatLngResponse : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="EncodeLatLngResponse" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected EncodeLatLngResponse() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="EncodeLatLngResponse" /> class.
        /// </summary>
        /// <param name="geohash">geohash (required).</param>
        public EncodeLatLngResponse(string geohash = default(string))
        {
            // to ensure "geohash" is required (not null)
            if (geohash == null)
            {
                throw new ArgumentNullException("geohash is a required property for EncodeLatLngResponse and cannot be null");
            }
            this.Geohash = geohash;
        }

        /// <summary>
        /// Gets or Sets Geohash
        /// </summary>
        [DataMember(Name = "geohash", IsRequired = true, EmitDefaultValue = true)]
        public string Geohash { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class EncodeLatLngResponse {\n");
            sb.Append("  Geohash: ").Append(Geohash).Append("\n");
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
            yield break;
        }
    }

}
