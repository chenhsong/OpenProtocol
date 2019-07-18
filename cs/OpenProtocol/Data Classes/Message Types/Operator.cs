using System;
using System.Collections.Generic;
using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	public class LoginOperatorMessage : Message
	{
		public uint ControllerId { get; }
		public string Password { get; }

		public LoginOperatorMessage (uint ControllerId, string Password, int Priority = 0) : base(Priority)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.Password = !string.IsNullOrWhiteSpace(Password) ? Password.Trim() : throw new ArgumentNullException(nameof(Password));
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal LoginOperatorMessage (string ID, long Sequence, uint ControllerId, string Password, int Priority) : base(ID, Sequence, Priority)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.Password = !string.IsNullOrWhiteSpace(Password) ? Password.Trim() : throw new ArgumentNullException(nameof(Password));
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
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.OperatorId = OperatorId;
			this.Password = !string.IsNullOrWhiteSpace(Password) ? Password.Trim() : throw new ArgumentNullException(nameof(Password));
			this.Name = !string.IsNullOrWhiteSpace(Name) ? Name.Trim() : throw new ArgumentNullException(nameof(Name));
			this.Level = Level;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal OperatorInfoMessage (string ID, long Sequence, uint ControllerId, uint OperatorId, string Name, string Password, byte Level, int Priority) : base(ID, Sequence, Priority)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.OperatorId = OperatorId;
			this.Password = !string.IsNullOrWhiteSpace(Password) ? Password.Trim() : throw new ArgumentNullException(nameof(Password));
			this.Name = !string.IsNullOrWhiteSpace(Name) ? Name.Trim() : throw new ArgumentNullException(nameof(Name));
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