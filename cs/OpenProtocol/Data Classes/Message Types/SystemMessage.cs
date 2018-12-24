using System;
using System.Collections.Generic;
using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	public class SystemMessageMessage : Message
	{
		public string Message { get; }

		public SystemMessageMessage (string Message, int Priority = 0) : base(Priority)
		{
			this.Message = !string.IsNullOrWhiteSpace(Message) ? Message : throw new ArgumentNullException(nameof(Message));
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal SystemMessageMessage (string ID, long Sequence, string Message, int Priority) : base(ID, Sequence, Priority)
		{
			this.Message = !string.IsNullOrWhiteSpace(Message) ? Message : throw new ArgumentNullException(nameof(Message));
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(Message), Message);
			foreach (var field in base.GetFields()) yield return field;
		}
	}
}