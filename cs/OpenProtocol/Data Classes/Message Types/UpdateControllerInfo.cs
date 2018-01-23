using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.ComponentModel;

namespace iChen.OpenProtocol
{
	/// <remarks>This message is deprecated.</remarks>
	[Obsolete]
	public class UpdateControllerInfoMessage : Message
	{
		public uint ControllerId { get; }
		public string DisplayName { get; }

		public UpdateControllerInfoMessage (uint ControllerId, string DisplayName = null, int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (string.IsNullOrWhiteSpace(DisplayName)) DisplayName = ControllerId.ToString();

			this.ControllerId = ControllerId;
			this.DisplayName = DisplayName.Trim();
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal UpdateControllerInfoMessage (long Sequence, uint ControllerId, string DisplayName, int Priority) : base(Sequence, Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (string.IsNullOrWhiteSpace(DisplayName)) DisplayName = ControllerId.ToString();

			this.ControllerId = ControllerId;
			this.DisplayName = DisplayName.Trim();
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(ControllerId), ControllerId);
			yield return new KeyValuePair<string, object>(nameof(DisplayName), DisplayName);
			foreach (var field in base.GetFields()) yield return field;
		}
	}

	public class ControllerStatusMessage : Message
	{
		[JsonProperty("timestamp")]
		public DateTime TimeStamp { get; }

		public uint ControllerId { get; }
		public string DisplayName { get; }
		public bool IsDisconnected { get; }

		[DefaultValue(OpModes.Unknown)]
		public OpModes OpMode { get; } = OpModes.Unknown;

		[DefaultValue(JobModes.Unknown)]
		public JobModes JobMode { get; } = JobModes.Unknown;

		[DefaultValue("")]
		[JsonProperty(NullValueHandling = NullValueHandling.Include)]
		public string JobCardId { get; } = string.Empty;

		public KeyValuePair<string, bool> Alarm { get; }
		public KeyValuePair<string, double> Audit { get; }
		public uint? OperatorId { get; }

		[DefaultValue("")]
		[JsonProperty(NullValueHandling = NullValueHandling.Include)]
		public string MoldId { get; } = string.Empty;

		public Controller Controller { get; }

		public ControllerStatusMessage (Controller Controller, int Priority = 0) : base(Priority)
		{
			if (Controller == null) throw new ArgumentNullException(nameof(Controller));

			this.Controller = Controller;
			this.ControllerId = Controller.ControllerId;
			this.DisplayName = Controller.DisplayName;
			this.IsDisconnected = false;
			this.OpMode = Controller.OpMode;
			this.JobMode = Controller.JobMode;
			this.JobCardId = Controller.JobCardId ?? string.Empty;
			this.OperatorId = Controller.OperatorId;
			this.MoldId = Controller.MoldId ?? string.Empty;
		}

		public ControllerStatusMessage (uint ControllerId, OpModes OpMode, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));

			this.ControllerId = ControllerId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.OpMode = OpMode;
		}

		public ControllerStatusMessage (uint ControllerId, JobModes JobMode, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));

			this.ControllerId = ControllerId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.JobMode = JobMode;
		}

		public ControllerStatusMessage (uint ControllerId, bool IsDisconnected, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (!IsDisconnected) throw new ArgumentOutOfRangeException(nameof(IsDisconnected));

			this.ControllerId = ControllerId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.IsDisconnected = true;
		}

		//public ControllerStatusMessage (uint ControllerId, string DisplayName, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		//{
		//	if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
		//	if (string.IsNullOrWhiteSpace(DisplayName)) throw new ArgumentNullException(nameof(DisplayName));

		//	this.ControllerId = ControllerId;
		//	this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
		//	this.DisplayName = DisplayName;
		//}

		public ControllerStatusMessage (uint ControllerId, string JobCardId, string MoldId, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (JobCardId != null && string.IsNullOrWhiteSpace(JobCardId)) JobCardId = string.Empty;
			if (MoldId != null && string.IsNullOrWhiteSpace(MoldId)) MoldId = string.Empty;

			this.ControllerId = ControllerId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.JobCardId = JobCardId;
			this.MoldId = MoldId;
		}

		public ControllerStatusMessage (uint ControllerId, string AlarmName, bool AlarmValue, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (AlarmName == null) throw new ArgumentNullException(nameof(AlarmName));

			this.ControllerId = ControllerId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.Alarm = new KeyValuePair<string, bool>(AlarmName, AlarmValue);
		}

		public ControllerStatusMessage (uint ControllerId, string AuditName, double AuditValue, uint OperatorId, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (AuditName == null) throw new ArgumentNullException(nameof(AuditName));

			this.ControllerId = ControllerId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.OperatorId = OperatorId;
			this.Audit = new KeyValuePair<string, double>(AuditName, AuditValue);
		}

		public ControllerStatusMessage (uint ControllerId, uint? OperatorId, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));

			this.ControllerId = ControllerId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.OperatorId = OperatorId;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal ControllerStatusMessage (long Sequence, uint ControllerId, DateTime TimeStamp, Controller Controller, string DisplayName, bool IsDisconnected, uint? OperatorId, string MoldId, KeyValuePair<string, bool> Alarm, KeyValuePair<string, double> Audit, int Priority, OpModes OpMode = OpModes.Unknown, JobModes JobMode = JobModes.Offline) : base(Sequence, Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (string.IsNullOrWhiteSpace(DisplayName)) DisplayName = null;
			if (MoldId != null && string.IsNullOrWhiteSpace(MoldId)) MoldId = string.Empty;

			this.ControllerId = ControllerId;
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.DisplayName = DisplayName?.Trim();
			this.IsDisconnected = IsDisconnected;
			this.OpMode = OpMode;
			this.JobMode = JobMode;
			this.OperatorId = OperatorId;
			this.MoldId = MoldId;
			this.Alarm = new KeyValuePair<string, bool>(Alarm.Key, Alarm.Value);
			this.Audit = new KeyValuePair<string, double>(Audit.Key, Audit.Value);
			this.Controller = Controller;
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			if (TimeStamp != default(DateTime)) yield return new KeyValuePair<string, object>(nameof(TimeStamp), TimeStamp);
			yield return new KeyValuePair<string, object>(nameof(ControllerId), ControllerId);
			if (!string.IsNullOrWhiteSpace(DisplayName)) yield return new KeyValuePair<string, object>(nameof(DisplayName), DisplayName);
			if (OpMode != OpModes.Unknown) yield return new KeyValuePair<string, object>(nameof(OpMode), OpMode);
			if (JobMode != JobModes.Unknown) yield return new KeyValuePair<string, object>(nameof(JobMode), JobMode);
			if (JobCardId != null) yield return new KeyValuePair<string, object>(nameof(JobCardId), JobCardId);
			if (OperatorId.HasValue) yield return new KeyValuePair<string, object>(nameof(OperatorId), OperatorId.Value);
			if (MoldId != null) yield return new KeyValuePair<string, object>(nameof(MoldId), MoldId);
			foreach (var field in base.GetFields()) yield return field;
			yield return new KeyValuePair<string, object>(nameof(IsDisconnected), IsDisconnected);
			if (Alarm.Key != null) {
				yield return new KeyValuePair<string, object>(nameof(Alarm) + "." + nameof(Alarm.Key), Alarm.Key);
				yield return new KeyValuePair<string, object>(nameof(Alarm) + "." + nameof(Alarm.Value), Alarm.Value);
			}
			if (Audit.Key != null) {
				yield return new KeyValuePair<string, object>(nameof(Audit) + "." + nameof(Audit.Key), Audit.Key);
				yield return new KeyValuePair<string, object>(nameof(Audit) + "." + nameof(Audit.Value), Audit.Value);
			}
		}
	}
}