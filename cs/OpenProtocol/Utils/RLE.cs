using System.Collections.Generic;
using System.Linq;

namespace iChen.Persistence.Cloud
{
	internal static class RunLengthEncoder
	{
		/// <remarks>
		/// Encodes a ushort array using RLE, packing the length of the runs (max short value) and the ushort value
		/// into a 32-bit int value.
		///
		/// The length of a run is encoded as one less than the actual length (i.e., 1 --> encoded as zero) because
		/// of a few reasons:
		///   1) Run length can never be zero, so a short value can potentially hold runs of one longer,
		///   2) For single values, the encoded 32-bit value is simply the value itself
		///      (because run length is encoded zero). This makes it easier to read.
		/// </remarks>
		public static IList<int> Encode (IList<ushort> data)
		{
			var list = new List<int>();
			var x = 0;
			var start = -1;
			ushort ch = 0;

			while (x < data.Count) {
				if (start < 0 || data[x] != ch) {
					// Encode the old run
					if (start >= 0) list.Add((x - start - 1) * 0x00010000 + ch);

					// Start a new run
					start = x;
					ch = data[x];
				}

				x++;
			}

			// Encode the last run
			if (start >= 0) list.Add((x - start - 1) * 0x00010000 + ch);

			return list;
		}

		public static IList<ushort> Decode (IList<int> data)
		{
			var list = new List<ushort>();

			foreach (var val in data) {
				var num = val / 0x00010000;
				var ch = (ushort) (val % 0x00010000);

				list.AddRange(Enumerable.Repeat(ch, num + 1));
			}

			return list;
		}
	}
}