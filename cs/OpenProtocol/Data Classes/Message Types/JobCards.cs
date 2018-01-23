using Newtonsoft.Json;
using System;
using System.Collections.Generic;

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
			if (string.IsNullOrWhiteSpace(JobCardId)) throw new ArgumentNullException(nameof(JobCardId));
			if (string.IsNullOrWhiteSpace(MoldId)) throw new ArgumentNullException(nameof(MoldId));

			this.JobCardId = JobCardId;
			this.MoldId = MoldId;
			this.Progress = Progress;
			this.Total = Total;
		}
	}

	public class RequestJobCardsListMessage : Message
	{
		public uint ControllerId { get; }

		public RequestJobCardsListMessage (uint ControllerId, int Priority = 0) : base(Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));

			this.ControllerId = ControllerId;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal RequestJobCardsListMessage (long Sequence, uint ControllerId, int Priority) : base(Sequence, Priority)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));

			this.ControllerId = ControllerId;
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
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.ControllerId = ControllerId;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal JobCardsListMessage (long Sequence, uint ControllerId, IReadOnlyDictionary<string, JobCard> Data, int Priority) :
			base(Sequence, Data, Priority, StringComparer.OrdinalIgnoreCase)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.ControllerId = ControllerId;
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(ControllerId), ControllerId);
			foreach (var field in base.GetFields()) yield return field;
		}
	}
}