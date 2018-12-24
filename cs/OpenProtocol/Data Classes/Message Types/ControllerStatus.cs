using System;
using System.Collections.Generic;
using System.ComponentModel;
using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
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

		public KeyValuePair<string, bool> Alarm { get; }
		public KeyValuePair<string, double> Audit { get; }
		public KeyValuePair<string, double> Variable { get; }

		public uint? OperatorId { get; }

		// This property is string.Empty if no change, null if set to blank
		[DefaultValue("")]
		[JsonProperty(NullValueHandling = NullValueHandling.Include)]
		[JsonConverter(typeof(NullToEmptyStringConverter))]
		public string OperatorName { get; } = string.Empty;

		// This property is string.Empty if no change, null if set to blank
		[DefaultValue("")]
		[JsonProperty(NullValueHandling = NullValueHandling.Include)]
		[JsonConverter(typeof(NullToEmptyStringConverter))]
		public string JobCardId { get; } = string.Empty;

		// This property is string.Empty if no change, null if set to blank
		[DefaultValue("")]
		[JsonProperty(NullValueHandling = NullValueHandling.Include)]
		[JsonConverter(typeof(NullToEmptyStringConverter))]
		public string MoldId { get; } = string.Empty;

		public StateValues State { get; }

		public Controller Controller { get; }

		public ControllerStatusMessage (Controller Controller, int Priority = 0) : base(Priority)
		{
			this.Controller = Controller ?? throw new ArgumentNullException(nameof(Controller));

			this.ControllerId = Controller.ControllerId;
			this.DisplayName = Controller.DisplayName;
			this.IsDisconnected = false;
			this.OpMode = Controller.OpMode;
			this.JobMode = Controller.JobMode;
			this.JobCardId = Controller.JobCardId ?? string.Empty;
			this.OperatorId = Controller.OperatorId;
			this.OperatorName = Controller.OperatorName;
			this.MoldId = Controller.MoldId ?? string.Empty;

			this.State = new StateValues(Controller.OpMode, Controller.JobMode, Controller.OperatorId, Controller.JobCardId, Controller.MoldId);
		}

		public ControllerStatusMessage (uint ControllerId, OpModes OpMode, StateValues State = default(StateValues), DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.OpMode = OpMode;

			this.State = new StateValues(OpMode, State.JobMode, State.OperatorId, State.JobCardId, State.MoldId);
		}

		public ControllerStatusMessage (uint ControllerId, JobModes JobMode, StateValues State = default(StateValues), DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.JobMode = JobMode;

			this.State = new StateValues(State.OpMode, JobMode, State.OperatorId, State.JobCardId, State.MoldId);
		}

		public ControllerStatusMessage (uint ControllerId, bool IsDisconnected, StateValues State = default(StateValues), DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.IsDisconnected = IsDisconnected ? true : throw new ArgumentOutOfRangeException(nameof(IsDisconnected));
			this.OpMode = OpModes.Offline;
			this.JobMode = JobModes.Offline;

			this.State = new StateValues(OpMode, JobMode, State.OperatorId, State.JobCardId, State.MoldId);
		}

		public ControllerStatusMessage (uint ControllerId, string DisplayName, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.DisplayName = !string.IsNullOrWhiteSpace(DisplayName) ? DisplayName : throw new ArgumentNullException(nameof(DisplayName));
		}

		public ControllerStatusMessage (uint ControllerId, string JobCardId, string MoldId, StateValues State = default(StateValues), DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (JobCardId != null && string.IsNullOrWhiteSpace(JobCardId)) JobCardId = string.Empty;
			if (MoldId != null && string.IsNullOrWhiteSpace(MoldId)) MoldId = string.Empty;

			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.JobCardId = JobCardId;
			this.MoldId = MoldId;

			this.State = new StateValues(State.OpMode, State.JobMode, State.OperatorId, JobCardId, MoldId);
		}

		public ControllerStatusMessage (uint ControllerId, string AlarmName, bool AlarmValue, StateValues State = default(StateValues), DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (string.IsNullOrWhiteSpace(AlarmName)) throw new ArgumentNullException(nameof(AlarmName));

			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.Alarm = new KeyValuePair<string, bool>(AlarmName, AlarmValue);

			this.State = State;
		}

		public ControllerStatusMessage (uint ControllerId, string VariableName, double VariableValue, uint OperatorId, StateValues State = default(StateValues), bool IsAudit = true, DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (string.IsNullOrWhiteSpace(VariableName)) throw new ArgumentNullException(nameof(VariableName));

			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.OperatorId = OperatorId;

			if (IsAudit)
				this.Audit = new KeyValuePair<string, double>(VariableName, VariableValue);
			else
				this.Variable = new KeyValuePair<string, double>(VariableName, VariableValue);

			this.State = State;
		}

		public ControllerStatusMessage (uint ControllerId, uint OperatorId, string OperatorName, StateValues State = default(StateValues), DateTime TimeStamp = default(DateTime), int Priority = 0) : base(Priority)
		{
			if (OperatorName != null && string.IsNullOrWhiteSpace(OperatorName)) OperatorName = null;

			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.OperatorId = OperatorId;
			this.OperatorName = OperatorName;

			this.State = new StateValues(State.OpMode, State.JobMode, OperatorId, JobCardId, State.MoldId);
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal ControllerStatusMessage (string ID, long Sequence, uint ControllerId, DateTime TimeStamp, Controller Controller, string DisplayName, bool IsDisconnected, OpModes OpMode, JobModes JobMode, uint? OperatorId, string OperatorName, string JobCardId, string MoldId, KeyValuePair<string, bool> Alarm, KeyValuePair<string, double> Audit, KeyValuePair<string, double> Variable, StateValues State, int Priority) : base(ID, Sequence, Priority)
		{
			if (DisplayName != null && string.IsNullOrWhiteSpace(DisplayName)) throw new ArgumentNullException(nameof(DisplayName));

			// The OperatorName, JobCardId and MoldId properties...
			//   when missing, should be treated as string.Empty, but enter here as null (default value without running NullToEmptyStringConverter)
			//   when actually null, should be treated as null, but enter here as string.Empty (via NullValueToEmptyStringConverter)
			// Therefore, we need to swap the null and string.Empty values.

			if (OperatorName == null) OperatorName = string.Empty;
			else if (string.IsNullOrWhiteSpace(OperatorName)) OperatorName = null;

			if (JobCardId == null) JobCardId = string.Empty;
			else if (string.IsNullOrWhiteSpace(JobCardId)) JobCardId = null;

			if (MoldId == null) MoldId = string.Empty;
			else if (string.IsNullOrWhiteSpace(MoldId)) MoldId = null;

			if (!OperatorId.HasValue)
				OperatorName = string.Empty;
			else if (OperatorId.Value == 0)
				OperatorName = null;

			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.TimeStamp = (TimeStamp == default(DateTime)) ? DateTime.Now : TimeStamp;
			this.DisplayName = DisplayName;
			this.IsDisconnected = IsDisconnected;
			this.OpMode = OpMode;
			this.JobMode = JobMode;
			this.OperatorId = OperatorId;
			this.OperatorName = OperatorName;
			this.JobCardId = JobCardId;
			this.MoldId = MoldId;
			this.Alarm = new KeyValuePair<string, bool>(Alarm.Key, Alarm.Value);
			this.Audit = new KeyValuePair<string, double>(Audit.Key, Audit.Value);
			this.Variable = new KeyValuePair<string, double>(Variable.Key, Variable.Value);
			this.Controller = Controller;

			this.State = State;
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			if (TimeStamp != default(DateTime)) yield return new KeyValuePair<string, object>(nameof(TimeStamp), TimeStamp);
			yield return new KeyValuePair<string, object>(nameof(ControllerId), ControllerId);
			if (!string.IsNullOrWhiteSpace(DisplayName)) yield return new KeyValuePair<string, object>(nameof(DisplayName), DisplayName);
			yield return new KeyValuePair<string, object>(nameof(IsDisconnected), IsDisconnected);
			if (OpMode != OpModes.Unknown) yield return new KeyValuePair<string, object>(nameof(OpMode), OpMode);
			if (JobMode != JobModes.Unknown) yield return new KeyValuePair<string, object>(nameof(JobMode), JobMode);
			if (OperatorId.HasValue) yield return new KeyValuePair<string, object>(nameof(OperatorId), OperatorId.Value);
			if (OperatorName != string.Empty) yield return new KeyValuePair<string, object>(nameof(OperatorName), OperatorName);
			if (JobCardId != string.Empty) yield return new KeyValuePair<string, object>(nameof(JobCardId), JobCardId);
			if (MoldId != string.Empty) yield return new KeyValuePair<string, object>(nameof(MoldId), MoldId);
			foreach (var field in base.GetFields()) yield return field;
			if (Alarm.Key != null) {
				yield return new KeyValuePair<string, object>(nameof(Alarm) + "." + nameof(Alarm.Key), Alarm.Key);
				yield return new KeyValuePair<string, object>(nameof(Alarm) + "." + nameof(Alarm.Value), Alarm.Value);
			}
			if (Audit.Key != null) {
				yield return new KeyValuePair<string, object>(nameof(Audit) + "." + nameof(Audit.Key), Audit.Key);
				yield return new KeyValuePair<string, object>(nameof(Audit) + "." + nameof(Audit.Value), Audit.Value);
			}
			if (Variable.Key != null) {
				yield return new KeyValuePair<string, object>(nameof(Variable) + "." + nameof(Variable.Key), Variable.Key);
				yield return new KeyValuePair<string, object>(nameof(Variable) + "." + nameof(Variable.Value), Variable.Value);
			}
		}
	}
}