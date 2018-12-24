using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	public class AliveMessage : Message
	{
		public AliveMessage () : base(0)
		{
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal AliveMessage (string ID, long Sequence, int Priority) : base(ID, Sequence, Priority)
		{
		}
	}
}