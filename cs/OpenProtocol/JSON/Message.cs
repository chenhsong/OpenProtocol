namespace iChen.OpenProtocol
{
	public abstract partial class Message
	{
		/// <summary>
		/// Parse a JSON object into a message.
		/// </summary>
		/// <param name="json">JSON text.</param>
		/// <returns>An Open Protocol message object.</returns>
		public static Message ParseJSON (string json)
		{
			return JSON.Parse(json);
		}

		/// <summary>
		/// Parse a JSON object into a message.
		/// </summary>
		/// <param name="json">JSON text.</param>
		/// <returns>An Open Protocol message object.</returns>
		public static T ParseJSON<T> (string json) where T : Message
		{
			return (T) ParseJSON(json);
		}

		public string ToJSON ()
		{
			return JSON.Serialize(this);
		}
	}
}