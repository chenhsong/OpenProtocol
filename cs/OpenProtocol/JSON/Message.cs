namespace iChen.OpenProtocol
{
	public abstract partial class Message
	{
		/// <summary>
		/// Parse a JSON object into a message.
		/// </summary>
		/// <param name="json">JSON text.</param>
		/// <returns>An Open Protocol message object.</returns>
		public static Message ParseJSON (string json) => JSON.Parse(json);

		/// <summary>
		/// Parse a JSON object into a message.
		/// </summary>
		/// <param name="json">JSON text.</param>
		/// <param name="sequence">Override sequence number.</param>
		/// <returns>An Open Protocol message object.</returns>
		public static Message ParseJSON (string json, long sequence)
		{
			var msg = ParseJSON(json);
			msg.Sequence = sequence;
			return msg;
		}

		/// <summary>
		/// Parse a JSON object into a message.
		/// </summary>
		/// <param name="json">JSON text.</param>
		/// <param name="sequence">Override sequence number.</param>
		/// <param name="priority">Override priority.</param>
		/// <returns>An Open Protocol message object.</returns>
		public static Message ParseJSON (string json, long sequence, int priority)
		{
			var msg = ParseJSON(json, sequence);
			msg.Priority = priority;
			return msg;
		}

		/// <summary>
		/// Parse a JSON object into a message.
		/// </summary>
		/// <param name="json">JSON text.</param>
		/// <returns>An Open Protocol message object.</returns>
		public static T ParseJSON<T> (string json) where T : Message => (T) ParseJSON(json);

		/// <summary>
		/// Parse a JSON object into a message.
		/// </summary>
		/// <param name="json">JSON text.</param>
		/// <param name="sequence">Override sequence number.</param>
		/// <returns>An Open Protocol message object.</returns>
		public static T ParseJSON<T> (string json, long sequence) where T : Message => (T) ParseJSON(json, sequence);

		/// <summary>
		/// Parse a JSON object into a message.
		/// </summary>
		/// <param name="json">JSON text.</param>
		/// <param name="sequence">Override sequence number.</param>
		/// <param name="priority">Override priority.</param>
		/// <returns>An Open Protocol message object.</returns>
		public static T ParseJSON<T> (string json, long sequence, int priority) where T : Message => (T) ParseJSON(json, sequence, priority);

		/// <summary>
		/// Cache the JSON representation because the message object is immutable.
		/// </summary>
		private string _JsonText = null;

		/// <summary>
		/// Serialize a message into JSON.
		/// </summary>
		/// <returns>JSON representation of the message.</returns>
		public string ToJSON ()
		{
			if (_JsonText == null) _JsonText = JSON.Serialize(this);
			return _JsonText;
		}
	}
}