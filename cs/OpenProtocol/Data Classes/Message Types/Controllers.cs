using Newtonsoft.Json;
using System.Collections.Generic;
using System.Linq;

namespace iChen.OpenProtocol
{
	public class RequestControllersListMessage : Message
	{
		public RequestControllersListMessage (int Priority = 0) : base(Priority)
		{
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal RequestControllersListMessage (long Sequence, int Priority) : base(Sequence, Priority)
		{
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
		internal ControllersListMessage (long Sequence, IReadOnlyDictionary<uint, Controller> Data, int Priority) :
			base(Sequence, Data, Priority)
		{
		}
	}
}