using System.ComponentModel;

namespace iChen.OpenProtocol
{
	public struct StateValues
	{
		[DefaultValue(OpModes.Unknown)]
		public OpModes OpMode { get; }

		[DefaultValue(JobModes.Unknown)]
		public JobModes JobMode { get; }

		public uint OperatorId { get; }
		public string JobCardId { get; }
		public string MoldId { get; }

		public StateValues (OpModes OpMode = OpModes.Unknown, JobModes JobMode = JobModes.Unknown, uint OperatorId = 0, string JobCardId = null, string MoldId = null)
		{
			this.OpMode = OpMode;
			this.JobMode = JobMode;
			this.OperatorId = OperatorId;
			this.JobCardId = !string.IsNullOrWhiteSpace(JobCardId) ? JobCardId : null;
			this.MoldId = !string.IsNullOrWhiteSpace(MoldId) ? MoldId : null;
		}
	}
}

