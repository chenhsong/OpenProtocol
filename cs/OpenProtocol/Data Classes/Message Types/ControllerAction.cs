using Newtonsoft.Json;
using System;
using System.Collections.Generic;

namespace iChen.OpenProtocol
{
	public class ControllerActionMessage : Message
	{
		[JsonProperty("timestamp")]
		public DateTime TimeStamp { get; }

		public uint ControllerId { get; }
		public uint ActionId { get; }

		public ControllerActionMessage (uint ControllerId, uint ActionId, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (ActionId <= 0) throw new ArgumentOutOfRangeException(nameof(ActionId));

			this.ControllerId = ControllerId;
			this.ActionId = ActionId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal ControllerActionMessage (long Sequence, uint ControllerId, uint ActionId, DateTime TimeStamp, int Priority) : base(Sequence, Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (ActionId <= 0) throw new ArgumentOutOfRangeException(nameof(ActionId));

			this.ControllerId = ControllerId;
			this.ActionId = ActionId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(ControllerId), ControllerId);
			yield return new KeyValuePair<string, object>(nameof(ActionId), ActionId);
			foreach (var field in base.GetFields()) yield return field;
		}
	}
}