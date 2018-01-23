using Newtonsoft.Json;
using System;
using System.Collections.Generic;

namespace iChen.OpenProtocol
{
	public class LoginOperatorMessage : Message
	{
		public uint ControllerId { get; }
		public string Password { get; }

		public LoginOperatorMessage (uint ControllerId, string Password, int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (string.IsNullOrWhiteSpace(Password)) throw new ArgumentNullException(nameof(Password));

			this.ControllerId = ControllerId;
			this.Password = Password.Trim();
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal LoginOperatorMessage (long Sequence, uint ControllerId, string Password, int Priority) : base(Sequence, Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (string.IsNullOrWhiteSpace(Password)) throw new ArgumentNullException(nameof(Password));

			this.ControllerId = ControllerId;
			this.Password = Password.Trim();
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(ControllerId), ControllerId);
			yield return new KeyValuePair<string, object>(nameof(Password), Password);
			foreach (var field in base.GetFields()) yield return field;
		}
	}

	public class OperatorInfoMessage : Message
	{
		public uint ControllerId { get; }
		public uint OperatorId { get; }
		public string Name { get; }
		public string Password { get; }

		[JsonProperty(DefaultValueHandling = DefaultValueHandling.Include)]
		public byte Level { get; }

		public OperatorInfoMessage (uint ControllerId, uint OperatorId, string Name, string Password, byte Level, int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (OperatorId <= 0) throw new ArgumentOutOfRangeException(nameof(OperatorId));
			if (string.IsNullOrWhiteSpace(Password)) throw new ArgumentNullException(nameof(Password));
			if (string.IsNullOrWhiteSpace(Name)) throw new ArgumentNullException(nameof(Name));

			this.ControllerId = ControllerId;
			this.OperatorId = OperatorId;
			this.Password = Password.Trim();
			this.Name = Name.Trim();
			this.Level = Level;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal OperatorInfoMessage (long Sequence, uint ControllerId, uint OperatorId, string Name, string Password, byte Level, int Priority) : base(Sequence, Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (OperatorId <= 0) throw new ArgumentOutOfRangeException(nameof(OperatorId));
			if (string.IsNullOrWhiteSpace(Password)) throw new ArgumentNullException(nameof(Password));
			if (string.IsNullOrWhiteSpace(Name)) throw new ArgumentNullException(nameof(Name));

			this.ControllerId = ControllerId;
			this.OperatorId = OperatorId;
			this.Password = Password.Trim();
			this.Name = Name.Trim();
			this.Level = Level;
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(ControllerId), ControllerId);
			yield return new KeyValuePair<string, object>(nameof(OperatorId), OperatorId);
			yield return new KeyValuePair<string, object>(nameof(Name), Name);
			yield return new KeyValuePair<string, object>(nameof(Level), Level);

			foreach (var field in base.GetFields()) yield return field;
		}
	}
}