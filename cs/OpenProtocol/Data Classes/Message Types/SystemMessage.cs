using Newtonsoft.Json;
using System;
using System.Collections.Generic;

namespace iChen.OpenProtocol
{
	public class SystemMessageMessage : Message
	{
		public string Message { get; }

		public SystemMessageMessage (string Message, int Priority = 0) : base(Priority)
		{
			if (string.IsNullOrWhiteSpace(Message)) throw new ArgumentNullException(nameof(Message));
			this.Message = Message;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal SystemMessageMessage (long Sequence, string Message, int Priority) : base(Sequence, Priority)
		{
			if (string.IsNullOrWhiteSpace(Message)) throw new ArgumentNullException(nameof(Message));
			this.Message = Message;
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(Message), Message);
			foreach (var field in base.GetFields()) yield return field;
		}
	}
}