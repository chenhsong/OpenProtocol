using System;
using Newtonsoft.Json.Serialization;

namespace iChen.OpenProtocol
{
	/// <summary>
	/// This type name binder serializes an Open Protocol message type into the simple name of the type
	/// without the "Message" postfix.
	/// For example: iChen.OpenProtocol.FoobarMessage --> "Foobar"
	/// </summary>
	internal class SimpleTypeNameSerializationBinder : ISerializationBinder
	{
		public void BindToName (Type serializedType, out string assemblyName, out string typeName)
		{
			assemblyName = null;
			var name = serializedType.Name;
			if (name.EndsWith(Message.MessageTypePostfix)) name = name.Substring(0, name.Length - Message.MessageTypePostfix.Length);
			typeName = name;
		}

		public Type BindToType (string assemblyName, string typeName)
		{
			var resolvedTypeName = $"{typeof(Message).Namespace}.{typeName}{Message.MessageTypePostfix}";
			return Type.GetType(resolvedTypeName, true);
		}
	}
}