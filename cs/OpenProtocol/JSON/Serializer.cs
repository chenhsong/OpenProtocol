using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using System;
using System.Reflection;

namespace iChen.OpenProtocol
{
	internal static partial class JSON
	{
		/// <summary>Check if a certain type is assignable to a generic type</summary>
		/// <remarks>
		/// This code is from James Fraumeni.
		/// http://stackoverflow.com/questions/74616/how-to-detect-if-type-is-another-generic-type/1075059#1075059
		/// </remarks>

		public static bool IsAssignableToGenericType (this Type givenType, Type genericType)
		{
			var interfaceTypes = givenType.GetTypeInfo().GetInterfaces();

			foreach (var it in interfaceTypes) {
				if (it.GetTypeInfo().IsGenericType && it.GetGenericTypeDefinition() == genericType) return true;
			}

			if (givenType.GetTypeInfo().IsGenericType && givenType.GetGenericTypeDefinition() == genericType) return true;

			var baseType = givenType.GetTypeInfo().BaseType;
			if (baseType == null) return false;

			return IsAssignableToGenericType(baseType, genericType);
		}

		private readonly static JsonSerializerSettings SerializationSettings = new JsonSerializerSettings()
		{
			DefaultValueHandling = DefaultValueHandling.Ignore,
			ContractResolver = new JsonSerializationContractResolver(),
			ReferenceLoopHandling = ReferenceLoopHandling.Ignore,
			NullValueHandling = NullValueHandling.Ignore,
			DateFormatHandling = DateFormatHandling.IsoDateFormat,
			DateTimeZoneHandling = DateTimeZoneHandling.Local,
			Formatting = Formatting.None,
			TypeNameHandling = TypeNameHandling.None,
			Converters = new JsonConverter[] { new StringEnumConverter(), new DecimalJsonConverter() }
		};

		public static string Serialize (Message message)
		{
			if (message == null) throw new ArgumentNullException(nameof(message));

			lock (typeof(JsonConvert)) {
				return JsonConvert.SerializeObject(message, Formatting.None, SerializationSettings);
			}
		}
	}
}