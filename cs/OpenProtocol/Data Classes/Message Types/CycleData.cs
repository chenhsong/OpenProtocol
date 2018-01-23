using Newtonsoft.Json;
using System;
using System.Collections.Generic;

namespace iChen.OpenProtocol
{
	public class CycleDataMessage : ControllerDictionaryMessage
	{
		public DateTime TimeStamp { get; }
		public string JobCardId { get; }
		public string MoldId { get; }
		public uint OperatorId { get; }

		public CycleDataMessage (uint ControllerId, string JobCardId, string MoldId, uint OperatorId, IReadOnlyDictionary<string, double> Data, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(ControllerId, Data, Priority)
		{
			if (JobCardId != null && string.IsNullOrWhiteSpace(JobCardId)) throw new ArgumentNullException(nameof(JobCardId));
			if (MoldId != null && string.IsNullOrWhiteSpace(MoldId)) throw new ArgumentNullException(nameof(MoldId));

			this.JobCardId = JobCardId?.Trim();
			this.MoldId = MoldId?.Trim();
			this.OperatorId = OperatorId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal CycleDataMessage (long Sequence, uint ControllerId, string JobCardId, string MoldId, uint OperatorId, IReadOnlyDictionary<string, double> Data, DateTime TimeStamp, int Priority) : base(Sequence, ControllerId, Data, Priority)
		{
			if (JobCardId != null && string.IsNullOrWhiteSpace(JobCardId)) throw new ArgumentNullException(nameof(JobCardId));
			if (MoldId != null && string.IsNullOrWhiteSpace(MoldId)) throw new ArgumentNullException(nameof(MoldId));

			this.JobCardId = JobCardId?.Trim();
			this.MoldId = MoldId?.Trim();
			this.OperatorId = OperatorId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(TimeStamp), TimeStamp);
			yield return new KeyValuePair<string, object>(nameof(JobCardId), JobCardId);
			yield return new KeyValuePair<string, object>(nameof(MoldId), MoldId);
			yield return new KeyValuePair<string, object>(nameof(OperatorId), OperatorId);
			foreach (var field in base.GetFields()) yield return field;
		}
	}
}