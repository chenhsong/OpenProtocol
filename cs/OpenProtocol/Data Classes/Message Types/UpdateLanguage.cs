using System;
using System.Collections.Generic;
using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	public class UpdateLanguageMessage : Message
	{
		public Languages Language { get; }

		public UpdateLanguageMessage (Languages Language, int Priority = 0) : base(Priority)
		{
			this.Language = (Language != Languages.Unknown) ? Language : throw new ArgumentOutOfRangeException(nameof(Language));
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal UpdateLanguageMessage (string ID, long Sequence, Languages Language, int Priority) : base(ID, Sequence, Priority)
		{
			this.Language = (Language != Languages.Unknown) ? Language : throw new ArgumentOutOfRangeException(nameof(Language));
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(Language), Language);
			foreach (var field in base.GetFields()) yield return field;
		}
	}
}