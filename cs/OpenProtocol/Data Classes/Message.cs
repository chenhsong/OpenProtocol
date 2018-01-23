using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Linq;

namespace iChen.OpenProtocol
{
	/// <summary>
	/// This class acts as the base class for all OpenProtocol messages.
	/// </summary>
	public abstract partial class Message
	{
		public const string MessageTypePostfix = "Message";

		private static long m_Seq = 0;

		public static Func<long> GetSequence = () => ++m_Seq;

		[JsonProperty("$type", Order = -999)]
		public string TypeName
		{
			get {
				var name = GetType().Name;
				if (name.EndsWith(MessageTypePostfix)) name = name.Substring(0, name.Length - MessageTypePostfix.Length);
				return name;
			}
		}

		public long Sequence { get; }
		public int Priority { get; }

		/// <param name="Priority">Message priority</param>
		/// <remarks>The message sequence number is auto-incremented.</remarks>
		public Message (int Priority = 0)
		{
			this.Priority = Priority;
			lock (typeof(Message)) { this.Sequence = GetSequence(); }
		}

		/// <param name="Sequence">Message sequence number</param>
		/// <param name="Priority">Message priority</param>
		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal Message (long Sequence, int Priority)
		{
			this.Sequence = Sequence;
			this.Priority = Priority;
		}

		/// <summary>
		/// Get all the fields in a message.
		/// </summary>
		/// <returns>IEnumerable of field-name/field-value pairs.</returns>
		public virtual IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(Sequence), Sequence);
			yield return new KeyValuePair<string, object>(nameof(Priority), Priority);
		}

		/// <summary>
		/// Comma-separated string of all fields in this message and their values.
		/// </summary>
		/// <remarks>Text strings are enclosed by square brackets.</remarks>
		internal virtual string FieldsText
		{
			get {
				return string.Join(", ",
					 GetFields().Select(kv => $"{kv.Key}={(kv.Value is string ? "[" + kv.Value + "]" : (kv.Value is DateTime ? "[" + ((DateTime) kv.Value).ToString("yyyy-MM-ddTHH:mm:sszzz") + "]" : kv.Value))}")
				);
			}
		}

		public override string ToString () => $"{{{TypeName} {FieldsText}}}";
	}
}