using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.ComponentModel;

namespace iChen.OpenProtocol
{
	public class JoinMessage : Message
	{
		public Languages Language { get; } = Languages.Unknown;
		public string OrgId { get; }
		public string Version { get; }
		public string Password { get; }

		[DefaultValue(Filters.All)]
		public Filters Filter { get; } = Filters.All;

		public JoinMessage (Languages Language, string Version, string Password, string OrgId = null, Filters Filter = Filters.All, int Priority = 0) : base(Priority)
		{
			if (Language == Languages.Unknown) throw new ArgumentOutOfRangeException(nameof(Language));
			if (string.IsNullOrWhiteSpace(Version)) throw new ArgumentNullException(nameof(Version));
			if (OrgId != null && string.IsNullOrWhiteSpace(OrgId)) throw new ArgumentNullException(nameof(OrgId));
			if (string.IsNullOrWhiteSpace(Password)) throw new ArgumentNullException(nameof(Password));
			if (Filter == Filters.None) throw new ArgumentNullException(nameof(Filters));

			this.Language = Language;
			this.Version = Version.Trim();
			this.OrgId = OrgId?.Trim();
			this.Password = Password.Trim();
			this.Filter = Filter;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal JoinMessage (long Sequence, string Version, string Password, int Priority, Languages Language = Languages.Unknown, string OrgId = null, Filters Filter = Filters.None) : base(Sequence, Priority)
		{
			if (Language == Languages.Unknown) throw new ArgumentOutOfRangeException(nameof(Language));
			if (string.IsNullOrWhiteSpace(Version)) throw new ArgumentNullException(nameof(Version));
			if (OrgId != null && string.IsNullOrWhiteSpace(OrgId)) throw new ArgumentNullException(nameof(OrgId));
			if (string.IsNullOrWhiteSpace(Password)) throw new ArgumentNullException(nameof(Password));
			if (Filter == Filters.None) throw new ArgumentNullException(nameof(Filters));

			this.Language = Language;
			this.Version = Version.Trim();
			this.OrgId = OrgId?.Trim();
			this.Password = Password.Trim();
			this.Filter = Filter;
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			foreach (var field in base.GetFields()) yield return field;
			yield return new KeyValuePair<string, object>(nameof(Language), Language);
			yield return new KeyValuePair<string, object>(nameof(Version), Version);
			yield return new KeyValuePair<string, object>(nameof(Password), Password);
		}
	}

	public class JoinResponseMessage : Message
	{
		public uint Result { get; } = 100;
		public uint Level { get; } = 0;
		public string Message { get; }

		public JoinResponseMessage (uint Result, uint Level, string Message = null, int Priority = 0) : base(Priority)
		{
			this.Result = Result;
			this.Level = Level;
			this.Message = Message;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal JoinResponseMessage (long Sequence, uint Result, uint Level, string Message, int Priority) : base(Sequence, Priority)
		{
			this.Result = Result;
			this.Level = Level;
			this.Message = Message;
		}

		[JsonIgnore]
		public bool Succeeded { get { return Result >= 100; } }

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(Result), Result);

			foreach (var field in base.GetFields()) yield return field;

			yield return new KeyValuePair<string, object>(nameof(Level), Level);

			if (!string.IsNullOrWhiteSpace(Message)) {
				yield return new KeyValuePair<string, object>(nameof(Message), Message);
			}
		}
	}
}