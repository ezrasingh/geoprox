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
    /// Neighboring geohash regions
    /// </summary>
    [DataContract(Name = "GeohashNeighborsResponse")]
    public partial class GeohashNeighborsResponse : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="GeohashNeighborsResponse" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected GeohashNeighborsResponse() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="GeohashNeighborsResponse" /> class.
        /// </summary>
        /// <param name="e">East (required).</param>
        /// <param name="n">North (required).</param>
        /// <param name="ne">North East (required).</param>
        /// <param name="nw">North West (required).</param>
        /// <param name="s">South (required).</param>
        /// <param name="se">South East (required).</param>
        /// <param name="sw">South West (required).</param>
        /// <param name="w">West (required).</param>
        public GeohashNeighborsResponse(string e = default(string), string n = default(string), string ne = default(string), string nw = default(string), string s = default(string), string se = default(string), string sw = default(string), string w = default(string))
        {
            // to ensure "e" is required (not null)
            if (e == null)
            {
                throw new ArgumentNullException("e is a required property for GeohashNeighborsResponse and cannot be null");
            }
            this.E = e;
            // to ensure "n" is required (not null)
            if (n == null)
            {
                throw new ArgumentNullException("n is a required property for GeohashNeighborsResponse and cannot be null");
            }
            this.N = n;
            // to ensure "ne" is required (not null)
            if (ne == null)
            {
                throw new ArgumentNullException("ne is a required property for GeohashNeighborsResponse and cannot be null");
            }
            this.Ne = ne;
            // to ensure "nw" is required (not null)
            if (nw == null)
            {
                throw new ArgumentNullException("nw is a required property for GeohashNeighborsResponse and cannot be null");
            }
            this.Nw = nw;
            // to ensure "s" is required (not null)
            if (s == null)
            {
                throw new ArgumentNullException("s is a required property for GeohashNeighborsResponse and cannot be null");
            }
            this.S = s;
            // to ensure "se" is required (not null)
            if (se == null)
            {
                throw new ArgumentNullException("se is a required property for GeohashNeighborsResponse and cannot be null");
            }
            this.Se = se;
            // to ensure "sw" is required (not null)
            if (sw == null)
            {
                throw new ArgumentNullException("sw is a required property for GeohashNeighborsResponse and cannot be null");
            }
            this.Sw = sw;
            // to ensure "w" is required (not null)
            if (w == null)
            {
                throw new ArgumentNullException("w is a required property for GeohashNeighborsResponse and cannot be null");
            }
            this.W = w;
        }

        /// <summary>
        /// East
        /// </summary>
        /// <value>East</value>
        [DataMember(Name = "e", IsRequired = true, EmitDefaultValue = true)]
        public string E { get; set; }

        /// <summary>
        /// North
        /// </summary>
        /// <value>North</value>
        [DataMember(Name = "n", IsRequired = true, EmitDefaultValue = true)]
        public string N { get; set; }

        /// <summary>
        /// North East
        /// </summary>
        /// <value>North East</value>
        [DataMember(Name = "ne", IsRequired = true, EmitDefaultValue = true)]
        public string Ne { get; set; }

        /// <summary>
        /// North West
        /// </summary>
        /// <value>North West</value>
        [DataMember(Name = "nw", IsRequired = true, EmitDefaultValue = true)]
        public string Nw { get; set; }

        /// <summary>
        /// South
        /// </summary>
        /// <value>South</value>
        [DataMember(Name = "s", IsRequired = true, EmitDefaultValue = true)]
        public string S { get; set; }

        /// <summary>
        /// South East
        /// </summary>
        /// <value>South East</value>
        [DataMember(Name = "se", IsRequired = true, EmitDefaultValue = true)]
        public string Se { get; set; }

        /// <summary>
        /// South West
        /// </summary>
        /// <value>South West</value>
        [DataMember(Name = "sw", IsRequired = true, EmitDefaultValue = true)]
        public string Sw { get; set; }

        /// <summary>
        /// West
        /// </summary>
        /// <value>West</value>
        [DataMember(Name = "w", IsRequired = true, EmitDefaultValue = true)]
        public string W { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class GeohashNeighborsResponse {\n");
            sb.Append("  E: ").Append(E).Append("\n");
            sb.Append("  N: ").Append(N).Append("\n");
            sb.Append("  Ne: ").Append(Ne).Append("\n");
            sb.Append("  Nw: ").Append(Nw).Append("\n");
            sb.Append("  S: ").Append(S).Append("\n");
            sb.Append("  Se: ").Append(Se).Append("\n");
            sb.Append("  Sw: ").Append(Sw).Append("\n");
            sb.Append("  W: ").Append(W).Append("\n");
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
