using Newtonsoft.Json;
using System;
using System.Collections.Generic;

namespace iChen.OpenProtocol
{
	public class UpdateLanguageMessage : Message
	{
		public Languages Language { get; }

		public UpdateLanguageMessage (Languages Language, int Priority = 0) : base(Priority)
		{
			if (Language == Languages.Unknown) throw new ArgumentOutOfRangeException(nameof(Language));
			this.Language = Language;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal UpdateLanguageMessage (long Sequence, Languages Language, int Priority) : base(Sequence, Priority)
		{
			if (Language == Languages.Unknown) throw new ArgumentOutOfRangeException(nameof(Language));
			this.Language = Language;
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(Language), Language);
			foreach (var field in base.GetFields()) yield return field;
		}
	}
}