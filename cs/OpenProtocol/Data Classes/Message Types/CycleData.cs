using System;
using System.Collections.Generic;
using System.ComponentModel;
using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	public class CycleDataMessage : ControllerDictionaryMessage
	{
		public DateTime TimeStamp { get; }
		public string JobCardId { get; }
		public string MoldId { get; }
		public uint OperatorId { get; }

		[DefaultValue(OpModes.Unknown)]
		public OpModes OpMode { get; } = OpModes.Unknown;

		[DefaultValue(JobModes.Unknown)]
		public JobModes JobMode { get; } = JobModes.Unknown;

		public CycleDataMessage (uint ControllerId, string JobCardId, string MoldId, uint OperatorId, OpModes OpMode, JobModes JobMode, IReadOnlyDictionary<string, double> Data, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(ControllerId, Data, Priority)
		{
			if (JobCardId != null && string.IsNullOrWhiteSpace(JobCardId)) throw new ArgumentNullException(nameof(JobCardId));
			if (MoldId != null && string.IsNullOrWhiteSpace(MoldId)) throw new ArgumentNullException(nameof(MoldId));

			this.JobCardId = JobCardId?.Trim();
			this.MoldId = MoldId?.Trim();
			this.OperatorId = OperatorId;
			this.OpMode = OpMode;
			this.JobMode = JobMode;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal CycleDataMessage (string ID, long Sequence, uint ControllerId, string JobCardId, string MoldId, uint OperatorId, OpModes OpMode, JobModes JobMode, IReadOnlyDictionary<string, double> Data, DateTime TimeStamp, int Priority) : base(ID, Sequence, ControllerId, Data, Priority)
		{
			if (JobCardId != null && string.IsNullOrWhiteSpace(JobCardId)) throw new ArgumentNullException(nameof(JobCardId));
			if (MoldId != null && string.IsNullOrWhiteSpace(MoldId)) throw new ArgumentNullException(nameof(MoldId));

			this.JobCardId = JobCardId?.Trim();
			this.MoldId = MoldId?.Trim();
			this.OperatorId = OperatorId;
			this.OpMode = OpMode;
			this.JobMode = JobMode;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(TimeStamp), TimeStamp);
			yield return new KeyValuePair<string, object>(nameof(JobCardId), JobCardId);
			yield return new KeyValuePair<string, object>(nameof(MoldId), MoldId);
			yield return new KeyValuePair<string, object>(nameof(OperatorId), OperatorId);
			yield return new KeyValuePair<string, object>(nameof(OpMode), OpMode);
			yield return new KeyValuePair<string, object>(nameof(JobMode), JobMode);
			foreach (var field in base.GetFields()) yield return field;
		}
	}
}