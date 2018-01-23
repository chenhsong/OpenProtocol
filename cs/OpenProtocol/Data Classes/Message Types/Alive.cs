using Newtonsoft.Json;

namespace iChen.OpenProtocol
{
	public class AliveMessage : Message
	{
		public AliveMessage () : base()
		{
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal AliveMessage (long Sequence, int Priority) : base(Sequence, Priority)
		{
		}
	}
}