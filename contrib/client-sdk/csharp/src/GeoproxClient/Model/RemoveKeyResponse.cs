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
    /// Returns key and deletion status
    /// </summary>
    [DataContract(Name = "RemoveKeyResponse")]
    public partial class RemoveKeyResponse : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="RemoveKeyResponse" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected RemoveKeyResponse() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="RemoveKeyResponse" /> class.
        /// </summary>
        /// <param name="deleted">If true key was removed (required).</param>
        /// <param name="key">Object key (required).</param>
        public RemoveKeyResponse(bool deleted = default(bool), string key = default(string))
        {
            this.Deleted = deleted;
            // to ensure "key" is required (not null)
            if (key == null)
            {
                throw new ArgumentNullException("key is a required property for RemoveKeyResponse and cannot be null");
            }
            this.Key = key;
        }

        /// <summary>
        /// If true key was removed
        /// </summary>
        /// <value>If true key was removed</value>
        [DataMember(Name = "deleted", IsRequired = true, EmitDefaultValue = true)]
        public bool Deleted { get; set; }

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
            sb.Append("class RemoveKeyResponse {\n");
            sb.Append("  Deleted: ").Append(Deleted).Append("\n");
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
