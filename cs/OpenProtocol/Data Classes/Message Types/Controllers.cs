using System.Collections.Generic;
using System.Linq;
using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	public class RequestControllersListMessage : Message
	{
		public uint ControllerId { get; } = 0;

		public RequestControllersListMessage (uint ControllerId, int Priority = 0) : this(Priority)
		{
			this.ControllerId = ControllerId;
		}

		public RequestControllersListMessage (int Priority = 0) : base(Priority)
		{
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal RequestControllersListMessage (uint ControllerId, string ID, long Sequence, int Priority) : base(ID, Sequence, Priority)
		{
			this.ControllerId = ControllerId;
		}
	}

	[JsonObject]
	public class ControllersListMessage : DataDictionaryMessage<uint, Controller>
	{
		public ControllersListMessage (IReadOnlyCollection<Controller> Data, int Priority = 0) :
			base(Data?.ToDictionary(c => c.ControllerId), Priority)
		{
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal ControllersListMessage (string ID, long Sequence, IReadOnlyDictionary<uint, Controller> Data, int Priority)
			: base(ID, Sequence, Data, Priority)
		{
		}
	}
}