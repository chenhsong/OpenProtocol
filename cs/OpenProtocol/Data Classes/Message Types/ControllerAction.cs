using System;
using System.Collections.Generic;
using Newtonsoft.Json;

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
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.ActionId = (ActionId > 0) ? ActionId : throw new ArgumentOutOfRangeException(nameof(ActionId));
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal ControllerActionMessage (string ID, long Sequence, uint ControllerId, uint ActionId, DateTime TimeStamp, int Priority) : base(ID, Sequence, Priority)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.ActionId = (ActionId > 0) ? ActionId : throw new ArgumentOutOfRangeException(nameof(ActionId));
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