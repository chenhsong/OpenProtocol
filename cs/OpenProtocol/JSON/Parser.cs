using System;
using Newtonsoft.Json;

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

			return (Message) JsonConvert.DeserializeObject(json, ParseSettings);
		}
	}
}