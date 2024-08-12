/*
 * geoprox-server
 *
 * Geoprox server implementation providing a HTTP API for geospatial queries and position tracking
 *
 * The version of the OpenAPI document: 0.4.2
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
    /// Arguments for range query
    /// </summary>
    [DataContract(Name = "QueryRange")]
    public partial class QueryRange : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="QueryRange" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected QueryRange() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="QueryRange" /> class.
        /// </summary>
        /// <param name="count">Maximum number of neighbors that can be returned (default 100).</param>
        /// <param name="lat">Latitude (required).</param>
        /// <param name="lng">Longitude (required).</param>
        /// <param name="range">Search radius in kilometers (required).</param>
        /// <param name="sorted">If enabled neighbors will be sorted by distance, nearest to furthest (default false).</param>
        public QueryRange(int? count = default(int?), double lat = default(double), double lng = default(double), int range = default(int), bool? sorted = default(bool?))
        {
            this.Lat = lat;
            this.Lng = lng;
            this.Range = range;
            this.Count = count;
            this.Sorted = sorted;
        }

        /// <summary>
        /// Maximum number of neighbors that can be returned (default 100)
        /// </summary>
        /// <value>Maximum number of neighbors that can be returned (default 100)</value>
        [DataMember(Name = "count", EmitDefaultValue = true)]
        public int? Count { get; set; }

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
        /// Search radius in kilometers
        /// </summary>
        /// <value>Search radius in kilometers</value>
        [DataMember(Name = "range", IsRequired = true, EmitDefaultValue = true)]
        public int Range { get; set; }

        /// <summary>
        /// If enabled neighbors will be sorted by distance, nearest to furthest (default false)
        /// </summary>
        /// <value>If enabled neighbors will be sorted by distance, nearest to furthest (default false)</value>
        [DataMember(Name = "sorted", EmitDefaultValue = true)]
        public bool? Sorted { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class QueryRange {\n");
            sb.Append("  Count: ").Append(Count).Append("\n");
            sb.Append("  Lat: ").Append(Lat).Append("\n");
            sb.Append("  Lng: ").Append(Lng).Append("\n");
            sb.Append("  Range: ").Append(Range).Append("\n");
            sb.Append("  Sorted: ").Append(Sorted).Append("\n");
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
            // Count (int?) maximum
            if (this.Count > (int?)65535)
            {
                yield return new ValidationResult("Invalid value for Count, must be a value less than or equal to 65535.", new [] { "Count" });
            }

            // Count (int?) minimum
            if (this.Count < (int?)1)
            {
                yield return new ValidationResult("Invalid value for Count, must be a value greater than or equal to 1.", new [] { "Count" });
            }

            // Range (int) maximum
            if (this.Range > (int)65535)
            {
                yield return new ValidationResult("Invalid value for Range, must be a value less than or equal to 65535.", new [] { "Range" });
            }

            // Range (int) minimum
            if (this.Range < (int)0)
            {
                yield return new ValidationResult("Invalid value for Range, must be a value greater than or equal to 0.", new [] { "Range" });
            }

            yield break;
        }
    }

}
