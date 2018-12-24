using System;
using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	/// <remarks>This code is from user3473830 at https://stackoverflow.com/questions/30300740/how-to-configure-json-net-deserializer-to-track-missing-properties </remarks>
	internal class NullToEmptyStringConverter : JsonConverter
	{
		public override bool CanWrite => false;

		public override bool CanConvert (Type objectType) => objectType == typeof(string);

		public override object ReadJson (JsonReader reader, Type objectType, object existingValue, JsonSerializer serializer)
		{
			if (objectType != typeof(string)) return serializer.Deserialize(reader);

			switch (reader.TokenType) {
				case JsonToken.String: return reader.Value;

				case JsonToken.Null: return string.Empty;

				default: throw new JsonSerializationException($"Invalid token type for string: {reader.TokenType}.");
			}
		}

		public override void WriteJson (JsonWriter writer, object value, JsonSerializer serializer)
			=> throw new NotImplementedException("Unnecessary because CanWrite is false. The type will skip the converter.");
	}
}