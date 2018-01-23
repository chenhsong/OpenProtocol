using Newtonsoft.Json;
using Newtonsoft.Json.Serialization;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;

namespace iChen.OpenProtocol
{
	/// <summary>This class implements an IContractResolver for JSON.NET to serialize objects to JSON</summary>
	/// <remarks>Based on the camelCase resolver which turns property names into camelCase.</remarks>

	internal class JsonSerializationContractResolver : CamelCasePropertyNamesContractResolver
	{
		private static readonly IReadOnlyDictionary<string, string> Mappings = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase)
		{
			{ "ID",  "id" }, { "TimeStamp", "timestamp" }, { "IP", "IP" }, { "IPAddress", "IPAddress" }, { "GUID", "guid" }
		};

		protected override JsonDictionaryContract CreateDictionaryContract (Type objectType)
		{
			var contract = base.CreateDictionaryContract(objectType);
			contract.DictionaryKeyResolver = propertyName => propertyName;
			return contract;
		}

		protected override JsonProperty CreateProperty (MemberInfo member, MemberSerialization memberSerialization)
		{
			var property = base.CreateProperty(member, memberSerialization);

			// If the property name ends with "ID" (and not just "ID"), make it "Id" instead to be more JavaScript-friendly
			if (property.PropertyName.EndsWith("ID") && property.PropertyName.Length > 2) {
				property.PropertyName = property.PropertyName.Substring(0, property.PropertyName.Length - 2) + "Id";
			}
			// If the property name is all-caps and more than one character, do not make it camelCase
			if (property.UnderlyingName.All(ch => char.IsUpper(ch)) && property.UnderlyingName.Length > 1) {
				property.PropertyName = property.UnderlyingName;
			}
			// Maps some common JavaScript properties
			if (Mappings.ContainsKey(property.PropertyName)) property.PropertyName = Mappings[property.PropertyName];

			if (property.PropertyType == typeof(DateTime)) {
				// Do not serialize DateTime.MinValue
				var propinfo = property.DeclaringType.GetTypeInfo().GetProperty(property.UnderlyingName);
				var fieldinfo = property.DeclaringType.GetTypeInfo().GetField(property.UnderlyingName);
				property.ShouldSerialize = obj => (DateTime) (propinfo?.GetValue(obj) ?? fieldinfo.GetValue(obj)) != DateTime.MinValue;
			} else if (property.PropertyType == typeof(DateTimeOffset)) {
				// Do not serialize DateTimeOffset.MinValue
				var propinfo = property.DeclaringType.GetTypeInfo().GetProperty(property.UnderlyingName);
				var fieldinfo = property.DeclaringType.GetTypeInfo().GetField(property.UnderlyingName);
				property.ShouldSerialize = obj => (DateTimeOffset) (propinfo?.GetValue(obj) ?? fieldinfo.GetValue(obj)) != DateTimeOffset.MinValue;
			} else if (property.PropertyType.IsAssignableToGenericType(typeof(ICollection<>))) {
				// Do not serialize empty collections
				var propinfo = property.DeclaringType.GetTypeInfo().GetProperty(property.UnderlyingName);
				var fieldinfo = property.DeclaringType.GetTypeInfo().GetField(property.UnderlyingName);

				if ((propinfo != null && propinfo.CanRead) || fieldinfo != null) {
					property.ShouldSerialize = obj => {
						try {
							if (obj == null) return false;
							var val = propinfo?.GetValue(obj) ?? fieldinfo?.GetValue(obj) ?? null;

							switch (val) {
								case null: return false;
								case Array ar: return ar.Length > 0;
								default: {
										var r = (int) val.GetType().GetTypeInfo().GetProperty("Count").GetValue(val);
										return r > 0;
									}
							}
						} catch (Exception ex) {
							throw new JsonSerializationException($"Error while serializing property [{propinfo.Name}] (type=[{propinfo?.PropertyType.Name ?? fieldinfo?.FieldType.Name ?? "???"}]) of [{obj.GetType().FullName}].", ex);
						}
					};
				}
			}

			return property;
		}
	}
}