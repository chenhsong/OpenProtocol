using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Linq;
using System.Net;
using System.Text.RegularExpressions;

namespace iChen.OpenProtocol
{
	public class Controller
	{
		public uint ControllerId { get; }
		public string DisplayName { get; }

		[DefaultValue(ControllerTypes.Unknown)]
		public ControllerTypes ControllerType { get; } = ControllerTypes.Unknown;

		public string Version { get; }
		public string Model { get; }
		public string IP { get; }

		[DefaultValue(OpModes.Unknown)]
		public OpModes OpMode { get; } = OpModes.Unknown;

		[DefaultValue(JobModes.Unknown)]
		public JobModes JobMode { get; } = JobModes.Unknown;

		public string JobCardId { get; }

		public IReadOnlyDictionary<string, double> LastCycleData { get; }
		public DateTime LastConnectionTime { get; }
		public uint OperatorId { get; }
		public string MoldId { get; }

		private static readonly Regex IPRegex = new Regex(@"^(?<ip>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})(\:(?<port>\d{1,5}))?$", RegexOptions.Compiled | RegexOptions.CultureInvariant | RegexOptions.ExplicitCapture | RegexOptions.IgnoreCase | RegexOptions.Singleline);
		private static readonly Regex SerialPortRegex = new Regex(@"COM(?<port>\d+)", RegexOptions.Singleline | RegexOptions.Compiled | RegexOptions.CultureInvariant | RegexOptions.ExplicitCapture | RegexOptions.IgnoreCase);

		public Controller (uint ControllerId, ControllerTypes ControllerType, string Version, string Model, string IP, OpModes OpMode, JobModes JobMode, string JobCardId, string DisplayName, IReadOnlyDictionary<string, double> LastCycleData = null, DateTime LastConnectionTime = default(DateTime), uint OperatorId = 0, string MoldId = null)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			if (ControllerType < 0) throw new ArgumentOutOfRangeException(nameof(ControllerType));
			if (string.IsNullOrWhiteSpace(Version)) throw new ArgumentNullException(nameof(Version));
			if (string.IsNullOrWhiteSpace(Model)) throw new ArgumentNullException(nameof(Model));
			if (string.IsNullOrWhiteSpace(IP)) throw new ArgumentNullException(nameof(IP));
			if (string.IsNullOrWhiteSpace(DisplayName)) throw new ArgumentNullException(nameof(DisplayName));
			if (OpMode == OpModes.Unknown) throw new ArgumentOutOfRangeException(nameof(OpMode));
			if (JobMode == JobModes.Unknown) throw new ArgumentOutOfRangeException(nameof(JobMode));
			if (JobCardId != null && string.IsNullOrWhiteSpace(JobCardId)) throw new ArgumentNullException(nameof(JobCardId));
			if (MoldId != null && string.IsNullOrWhiteSpace(MoldId)) throw new ArgumentNullException(nameof(MoldId));

			string strIP = null;
			var match = IPRegex.Match(IP.Trim());
			if (match.Success) {
				IPAddress addr;
				if (!IPAddress.TryParse(match.Groups["ip"].Value, out addr)) throw new ArgumentOutOfRangeException(nameof(IP));
				strIP = addr.ToString() + (match.Groups["port"].Success ? ":" + match.Groups["port"].Value : null);
			} else {
				match = SerialPortRegex.Match(IP.Trim());
				if (match.Success) {
					strIP = "COM" + match.Groups["port"].Value;
				} else {
					throw new ArgumentOutOfRangeException(nameof(IP));
				}
			}

			this.ControllerId = ControllerId;
			this.ControllerType = ControllerType;
			this.Version = Version.Trim();
			this.Model = Model.Trim();
			this.IP = strIP;
			this.OpMode = OpMode;
			this.JobMode = JobMode;
			this.DisplayName = DisplayName.Trim();
			this.JobCardId = JobCardId?.Trim();
			this.LastCycleData = LastCycleData?.ToDictionary(kv => kv.Key, kv => kv.Value, StringComparer.OrdinalIgnoreCase);
			this.LastConnectionTime = LastConnectionTime;
			this.OperatorId = OperatorId;
			this.MoldId = MoldId?.Trim();
		}
	}
}