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
    /// Returns index creation status
    /// </summary>
    [DataContract(Name = "CreateIndexResponse")]
    public partial class CreateIndexResponse : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="CreateIndexResponse" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected CreateIndexResponse() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="CreateIndexResponse" /> class.
        /// </summary>
        /// <param name="created">If true index was created (required).</param>
        /// <param name="existed">If true index alredy exist (required).</param>
        public CreateIndexResponse(bool created = default(bool), bool existed = default(bool))
        {
            this.Created = created;
            this.Existed = existed;
        }

        /// <summary>
        /// If true index was created
        /// </summary>
        /// <value>If true index was created</value>
        [DataMember(Name = "created", IsRequired = true, EmitDefaultValue = true)]
        public bool Created { get; set; }

        /// <summary>
        /// If true index alredy exist
        /// </summary>
        /// <value>If true index alredy exist</value>
        [DataMember(Name = "existed", IsRequired = true, EmitDefaultValue = true)]
        public bool Existed { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class CreateIndexResponse {\n");
            sb.Append("  Created: ").Append(Created).Append("\n");
            sb.Append("  Existed: ").Append(Existed).Append("\n");
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
