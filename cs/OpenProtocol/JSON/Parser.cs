using Newtonsoft.Json;
using System;

namespace iChen.OpenProtocol
{
	internal static partial class JSON
	{
		private readonly static JsonSerializerSettings ParseSettings = new JsonSerializerSettings()
		{
			DefaultValueHandling = DefaultValueHandling.Ignore,
			ContractResolver = new JsonSerializationContractResolver(),
			ReferenceLoopHandling = ReferenceLoopHandling.Ignore,
			NullValueHandling = NullValueHandling.Include,
			DateFormatHandling = DateFormatHandling.IsoDateFormat,
			DateTimeZoneHandling = DateTimeZoneHandling.Local,
			Formatting = Formatting.None, ObjectCreationHandling = ObjectCreationHandling.Replace,
			MissingMemberHandling = MissingMemberHandling.Ignore,
			TypeNameHandling = TypeNameHandling.Auto,
			SerializationBinder = new SimpleTypeNameSerializationBinder()
		};

		public static Message Parse (string json)
		{
			if (json == null) throw new ArgumentNullException(nameof(json));

			lock (typeof(JsonConvert)) {
				var test = @"{""CLAMP1"":987.65,""CLAMP2"":42,""FAST"":true}";
				var dict = JsonConvert.DeserializeObject(test, ParseSettings);

				return (Message) JsonConvert.DeserializeObject(json, ParseSettings);
			}
		}
	}
}