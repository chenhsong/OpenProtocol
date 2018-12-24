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

		public string ControllerType { get; } = "Unknown";

		public string Version { get; }
		public string Model { get; }
		public string IP { get; }

		public double? GeoLatitude { get; }
		public double? GeoLongitude { get; }

		[DefaultValue(OpModes.Unknown)]
		public OpModes OpMode { get; } = OpModes.Unknown;

		[DefaultValue(JobModes.Unknown)]
		public JobModes JobMode { get; } = JobModes.Unknown;

		public string JobCardId { get; }

		public IReadOnlyDictionary<string, double> LastCycleData { get; }
		public IReadOnlyDictionary<string, double> Variables { get; }
		public DateTime LastConnectionTime { get; }
		public uint OperatorId { get; }
		public string OperatorName { get; }
		public string MoldId { get; }

		private static readonly Regex IPRegex = new Regex(@"^(?<ip>\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})(\:(?<port>\d{1,5}))?$", RegexOptions.Compiled | RegexOptions.CultureInvariant | RegexOptions.ExplicitCapture | RegexOptions.IgnoreCase | RegexOptions.Singleline);
		private static readonly Regex TtyRegex = new Regex(@"tty\w+", RegexOptions.Singleline | RegexOptions.Compiled | RegexOptions.CultureInvariant | RegexOptions.ExplicitCapture | RegexOptions.IgnoreCase);
		private static readonly Regex SerialPortRegex = new Regex(@"COM(?<port>\d+)", RegexOptions.Singleline | RegexOptions.Compiled | RegexOptions.CultureInvariant | RegexOptions.ExplicitCapture | RegexOptions.IgnoreCase);

		public Controller (uint ControllerId, string ControllerType, string Version, string Model, string IP, OpModes OpMode, JobModes JobMode, string JobCardId, string DisplayName, double? GeoLatitude = null, double? GeoLongitude = null, IReadOnlyDictionary<string, double> LastCycleData = null, IReadOnlyDictionary<string, double> Variables = null, DateTime LastConnectionTime = default(DateTime), uint OperatorId = 0, string OperatorName = null, string MoldId = null)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.ControllerType = !string.IsNullOrWhiteSpace(ControllerType) ? ControllerType.Trim() : throw new ArgumentNullException(nameof(ControllerType));
			this.Version = !string.IsNullOrWhiteSpace(Version) ? Version.Trim() : throw new ArgumentNullException(nameof(Version));
			this.Model = !string.IsNullOrWhiteSpace(Model) ? Model.Trim() : throw new ArgumentNullException(nameof(Model));

			if (string.IsNullOrWhiteSpace(IP)) throw new ArgumentNullException(nameof(IP));

			string strIP = IP.Trim();
			var match = IPRegex.Match(strIP);

			if (match.Success) {
				if (!IPAddress.TryParse(match.Groups["ip"].Value, out IPAddress addr)) throw new ArgumentOutOfRangeException(nameof(IP));
				strIP = addr.ToString() + (match.Groups["port"].Success ? ":" + match.Groups["port"].Value : null);
			} else {
				match = SerialPortRegex.Match(strIP);
				if (match.Success) {
					strIP = "COM" + match.Groups["port"].Value;
				} else {
					match = TtyRegex.Match(strIP);
					if (!match.Success) throw new ArgumentOutOfRangeException(nameof(IP));
				}
			}

			this.IP = strIP;

			this.OpMode = (OpMode != OpModes.Unknown) ? OpMode : throw new ArgumentOutOfRangeException(nameof(OpMode));
			this.JobMode = (JobMode != JobModes.Unknown) ? JobMode : throw new ArgumentOutOfRangeException(nameof(JobMode));
			this.DisplayName = !string.IsNullOrWhiteSpace(DisplayName) ? DisplayName.Trim() : throw new ArgumentNullException(nameof(DisplayName));
			this.JobCardId = (JobCardId == null || !string.IsNullOrWhiteSpace(JobCardId)) ? JobCardId?.Trim() : throw new ArgumentNullException(nameof(JobCardId));
			this.LastCycleData = LastCycleData?.ToDictionary(kv => kv.Key, kv => kv.Value, StringComparer.OrdinalIgnoreCase);
			this.Variables = Variables?.ToDictionary(kv => kv.Key, kv => kv.Value, StringComparer.OrdinalIgnoreCase);
			this.LastConnectionTime = LastConnectionTime;
			this.OperatorId = OperatorId;
			this.OperatorName = (OperatorName == null || !string.IsNullOrWhiteSpace(OperatorName)) ? OperatorName?.Trim() : throw new ArgumentNullException(nameof(OperatorName));
			this.MoldId = (MoldId == null || !string.IsNullOrWhiteSpace(MoldId)) ? MoldId?.Trim() : throw new ArgumentNullException(nameof(MoldId));

			this.GeoLatitude = GeoLatitude;
			this.GeoLongitude = GeoLongitude;
		}
	}
}