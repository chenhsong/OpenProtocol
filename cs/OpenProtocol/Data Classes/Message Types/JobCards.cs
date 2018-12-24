using System;
using System.Collections.Generic;
using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	public class JobCard
	{
		public string JobCardId { get; }
		public string MoldId { get; }

		[JsonProperty(DefaultValueHandling = DefaultValueHandling.Include)]
		public uint Progress { get; }

		[JsonProperty(DefaultValueHandling = DefaultValueHandling.Include)]
		public uint Total { get; }

		public JobCard (string JobCardId, string MoldId, uint Progress, uint Total)
		{
			this.JobCardId = !string.IsNullOrWhiteSpace(JobCardId) ? JobCardId : throw new ArgumentNullException(nameof(JobCardId));
			this.MoldId = !string.IsNullOrWhiteSpace(MoldId) ? MoldId : throw new ArgumentNullException(nameof(MoldId));
			this.Progress = Progress;
			this.Total = Total;
		}
	}

	public class RequestJobCardsListMessage : Message
	{
		public uint ControllerId { get; }

		public RequestJobCardsListMessage (uint ControllerId, int Priority = 0) : base(Priority)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal RequestJobCardsListMessage (string ID, long Sequence, uint ControllerId, int Priority) : base(ID, Sequence, Priority)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(ControllerId), ControllerId);
			foreach (var field in base.GetFields()) yield return field;
		}
	}

	public class JobCardsListMessage : DataDictionaryMessage<string, JobCard>
	{
		public uint ControllerId { get; }

		public JobCardsListMessage (uint ControllerId, IReadOnlyDictionary<string, JobCard> Data, int Priority = 0) :
			base(Data, Priority, StringComparer.OrdinalIgnoreCase)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal JobCardsListMessage (string ID, long Sequence, uint ControllerId, IReadOnlyDictionary<string, JobCard> Data, int Priority) :
			base(ID, Sequence, Data, Priority, StringComparer.OrdinalIgnoreCase)
		{
			this.ControllerId = (ControllerId > 0) ? ControllerId : throw new ArgumentOutOfRangeException(nameof(ControllerId));
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(ControllerId), ControllerId);
			foreach (var field in base.GetFields()) yield return field;
		}
	}
}