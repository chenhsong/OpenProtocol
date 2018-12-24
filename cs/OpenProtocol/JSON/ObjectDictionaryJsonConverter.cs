using System;
using System.Collections.Generic;
using System.Linq;
using System.Reflection;
using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	/// <remarks>This code is from enzi at http://stackoverflow.com/questions/8297541/how-do-i-change-the-default-type-for-numeric-deserialization </remarks>
	internal class ObjectDictionaryJsonConverter : JsonConverter
	{
		private const string Base64Prefix = "base64:";

		public override bool CanConvert (Type objectType)
			=> objectType.GetTypeInfo().IsAssignableFrom(typeof(Dictionary<string, object>).GetTypeInfo());

		public override object ReadJson (JsonReader reader, Type objectType, object existingValue, JsonSerializer serializer)
		{
			if (!objectType.GetTypeInfo().IsAssignableFrom(typeof(Dictionary<string, object>).GetTypeInfo())) return serializer.Deserialize(reader);

			// again, very concrete
			var result = new Dictionary<string, object>();
			reader.Read();

			while (reader.TokenType == JsonToken.PropertyName) {
				var propertyName = reader.Value as string;
				reader.Read();

				object value;
				switch (reader.TokenType) {
					case JsonToken.Integer: {
							value = Convert.ToInt32(reader.Value);      // convert to Int32 instead of Int64
							break;
						}
					case JsonToken.String: {
							value = serializer.Deserialize(reader);
							if (value is string strval && strval.StartsWith(Base64Prefix)) {
								// convert base64-encoded strings to byte arrays
								value = Convert.FromBase64String(strval.Substring(Base64Prefix.Length));
							}
							break;
						}
					default: {
							value = serializer.Deserialize(reader);     // let the serializer handle all other cases
							break;
						}
				}
				result.Add(propertyName, value);
				reader.Read();
			}

			return result;
		}

		public override void WriteJson (JsonWriter writer, object value, JsonSerializer serializer)
		{
			switch (value) {
				case IReadOnlyDictionary<string, object> dict when dict.Values.Any(val => val != null && val is byte[]): {
						// Has byte-array, convert byte-arrays into base64-encoded strings
						var copy = dict.Select(kv => (kv.Value != null && kv.Value is byte[])
															? new KeyValuePair<string, object>(kv.Key, Base64Prefix + Convert.ToBase64String(kv.Value as byte[]))
															: kv).ToDictionary(kv => kv.Key, kv => kv.Value);

						serializer.Serialize(writer, copy);
						break;
					}

				case IReadOnlyDictionary<string, object> dict: {
						// No byte array
						serializer.Serialize(writer, value);
						break;
					}

				default: serializer.Serialize(writer, value); return;
			}
		}
	}
}
