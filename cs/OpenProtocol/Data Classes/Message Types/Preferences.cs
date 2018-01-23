using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Linq;

namespace iChen.OpenProtocol
{
	/// <remarks>This message is deprecated.</remarks>
	[Obsolete]
	public class PreferencesMessage : DataDictionaryMessage<string, object>
	{
		public PreferencesMessage (IReadOnlyDictionary<string, object> Data, int Priority = 0) :
			base(Data, Priority, StringComparer.OrdinalIgnoreCase)
		{
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal PreferencesMessage (long Sequence, IReadOnlyDictionary<string, object> Data, int Priority) :
			base(Sequence, Data, Priority, StringComparer.OrdinalIgnoreCase)
		{
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			foreach (var field in base.GetFields()) yield return field;
			yield return new KeyValuePair<string, object>(nameof(Data), Data.Count);
		}

		public override string ToString ()
		{
			var prefs = string.Join("\n\t", Data.Select(p => $"{p.Key}={p.Value}"));
			return $"{{{GetType().Name} {FieldsText}\nPreferences:\n\t{prefs}\n}}";
		}
	}
}