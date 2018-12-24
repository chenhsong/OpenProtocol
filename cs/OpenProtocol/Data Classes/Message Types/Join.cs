using System;
using System.Collections.Generic;
using System.ComponentModel;
using Newtonsoft.Json;

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
			if (OrgId != null && string.IsNullOrWhiteSpace(OrgId)) throw new ArgumentNullException(nameof(OrgId));

			this.Language = (Language != Languages.Unknown) ? Language : throw new ArgumentOutOfRangeException(nameof(Language));
			this.Version = !string.IsNullOrWhiteSpace(Version) ? Version.Trim() : throw new ArgumentNullException(nameof(Version));
			this.OrgId = OrgId?.Trim();
			this.Password = !string.IsNullOrWhiteSpace(Password) ? Password.Trim() : throw new ArgumentNullException(nameof(Password));
			this.Filter = (Filter != Filters.None) ? Filter : throw new ArgumentNullException(nameof(Filters));
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal JoinMessage (string ID, long Sequence, string Version, string Password, int Priority, Languages Language = Languages.Unknown, string OrgId = null, Filters Filter = Filters.None) : base(ID, Sequence, Priority)
		{
			if (OrgId != null && string.IsNullOrWhiteSpace(OrgId)) throw new ArgumentNullException(nameof(OrgId));

			this.Language = (Language != Languages.Unknown) ? Language : throw new ArgumentOutOfRangeException(nameof(Language));
			this.Version = !string.IsNullOrWhiteSpace(Version) ? Version.Trim() : throw new ArgumentNullException(nameof(Version));
			this.OrgId = OrgId?.Trim();
			this.Password = !string.IsNullOrWhiteSpace(Password) ? Password.Trim() : throw new ArgumentNullException(nameof(Password));
			this.Filter = (Filter != Filters.None) ? Filter : throw new ArgumentNullException(nameof(Filters));
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
		internal JoinResponseMessage (string ID, long Sequence, uint Result, uint Level, string Message, int Priority) : base(ID, Sequence, Priority)
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