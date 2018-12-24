using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	/// <summary>
	/// This class acts as the base class for all OpenProtocol messages.
	/// </summary>
	public abstract partial class Message
	{
		public const string MessageTypePostfix = "Message";

		private static long m_Seq = 0;

		public static Func<long> GetNextSequenceNumber = () => Interlocked.Increment(ref m_Seq);

		private string _typeName = null;

		[JsonProperty("$type", Order = -999)]
		public string TypeName
		{
			get {
				if (_typeName == null) {
					_typeName = GetType().Name;
					if (_typeName.EndsWith(MessageTypePostfix)) _typeName = _typeName.Substring(0, _typeName.Length - MessageTypePostfix.Length);
				}

				return _typeName;
			}
		}

		public string ID { get; private set; } = null;
		public long Sequence { get; internal set; }
		public int Priority { get; internal set; }

		/// <param name="Priority">Message priority</param>
		/// <remarks>The message sequence number is auto-incremented.</remarks>
		public Message (int Priority)
		{
			this.Priority = Priority;
			this.Sequence = GetNextSequenceNumber();
		}

		/// <param name="ID">Unique ID</param>
		/// <param name="Sequence">Message sequence number</param>
		/// <param name="Priority">Message priority</param>
		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal Message (string ID, long Sequence, int Priority)
		{
			if (ID != null && string.IsNullOrWhiteSpace(ID)) throw new ArgumentOutOfRangeException(nameof(ID));

			this.ID = ID?.Trim();
			this.Sequence = Sequence;
			this.Priority = Priority;
		}

		/// <summary>
		/// Overrides the ID field with a randomly-generated unique key string.
		/// </summary>
		/// <remarks>This is the only muting method; otherwise the class is immutable.</remarks>
		public void CreateUniqueID ()
		{
			ID = Guid.NewGuid().ToString().Replace("-", "").ToLowerInvariant();
			_JsonText = null;	// The class is supposed to be immutable (other than this method), so clear the cached JSON text
		}
		
		/// <summary>
		/// Get all the fields in a message.
		/// </summary>
		/// <returns>IEnumerable of field-name/field-value pairs.</returns>
		public virtual IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			if (!string.IsNullOrWhiteSpace(ID)) yield return new KeyValuePair<string, object>(nameof(ID), ID);
			yield return new KeyValuePair<string, object>(nameof(Sequence), Sequence);
			yield return new KeyValuePair<string, object>(nameof(Priority), Priority);
		}

		/// <summary>
		/// Comma-separated string of all fields in this message and their values.
		/// </summary>
		/// <remarks>Text strings are enclosed by square brackets.</remarks>
		internal virtual string FieldsText
		{
			get => string.Join(", ", GetFields().Select(kv => {
				switch (kv.Value) {
					case null: return $"{kv.Key}=null";
					case string str: return $"{kv.Key}=[{str}]";
					case DateTime date: return $"{kv.Key}=#{date.ToString("yyyy-MM-ddTHH:mm:sszzz")}#";
					default: return $"{kv.Key}={kv.Value}";
				}
			}));
		}

		public override string ToString () => $"{{{TypeName} {FieldsText}}}";
	}
}