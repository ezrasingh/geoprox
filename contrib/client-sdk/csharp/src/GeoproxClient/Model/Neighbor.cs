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
    /// Nearby object
    /// </summary>
    [DataContract(Name = "Neighbor")]
    public partial class Neighbor : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="Neighbor" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected Neighbor() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="Neighbor" /> class.
        /// </summary>
        /// <param name="distance">Distance in kilometers (required).</param>
        /// <param name="key">Object key (required).</param>
        public Neighbor(double distance = default(double), string key = default(string))
        {
            this.Distance = distance;
            // to ensure "key" is required (not null)
            if (key == null)
            {
                throw new ArgumentNullException("key is a required property for Neighbor and cannot be null");
            }
            this.Key = key;
        }

        /// <summary>
        /// Distance in kilometers
        /// </summary>
        /// <value>Distance in kilometers</value>
        [DataMember(Name = "distance", IsRequired = true, EmitDefaultValue = true)]
        public double Distance { get; set; }

        /// <summary>
        /// Object key
        /// </summary>
        /// <value>Object key</value>
        [DataMember(Name = "key", IsRequired = true, EmitDefaultValue = true)]
        public string Key { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class Neighbor {\n");
            sb.Append("  Distance: ").Append(Distance).Append("\n");
            sb.Append("  Key: ").Append(Key).Append("\n");
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
