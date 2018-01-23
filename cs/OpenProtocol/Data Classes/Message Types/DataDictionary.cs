using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.Linq;

namespace iChen.OpenProtocol
{
	public abstract class DataDictionaryMessage<K, T> : Message/*, IReadOnlyDictionary<K, T>*/
	{
		protected IReadOnlyDictionary<K, T> m_DataStore = null;

		public virtual IReadOnlyDictionary<K, T> Data { get { return m_DataStore; } }

		public DataDictionaryMessage (IReadOnlyDictionary<K, T> Data, int Priority = 0, IEqualityComparer<K> comparer = null) : base(Priority)
		{
			if (Data == null) throw new ArgumentNullException(nameof(Data));
			this.m_DataStore = Data.ToDictionary(kv => kv.Key, kv => kv.Value, comparer);
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal DataDictionaryMessage (long Sequence, IReadOnlyDictionary<K, T> Data, int Priority, IEqualityComparer<K> comparer = null) : base(Sequence, Priority)
		{
			if (Data == null) throw new ArgumentNullException(nameof(Data));
			this.m_DataStore = Data.ToDictionary(kv => kv.Key, kv => kv.Value, comparer);
		}

		public override IEnumerable<KeyValuePair<string, object>> GetFields ()
		{
			yield return new KeyValuePair<string, object>(nameof(Data), Data.Count);
			foreach (var field in base.GetFields()) yield return field;
		}

		#region IReadOnlyDictionary

		public bool ContainsKey (K key)
		{
			return Data.ContainsKey(key);
		}

		public bool TryGetValue (K key, out T value)
		{
			return Data.TryGetValue(key, out value);
		}

		public IEnumerator<KeyValuePair<K, T>> GetEnumerator ()
		{
			return Data.GetEnumerator();
		}

		//IEnumerator IEnumerable.GetEnumerator ()
		//{
		//	return Data.GetEnumerator();
		//}

		[JsonIgnore]
		public int Count { get { return Data.Count; } }

		[JsonIgnore]
		public IEnumerable<K> Keys { get { return Data.Keys; } }

		[JsonIgnore]
		public IEnumerable<T> Values { get { return Data.Values; } }

		public T this[K key] { get { return Data[key]; } }

		#endregion IReadOnlyDictionary
	}

	public abstract class ObjectDictionaryMessage : DataDictionaryMessage<string, object>
	{
		[JsonConverter(typeof(ObjectDictionaryJsonConverter))]
		public override IReadOnlyDictionary<string, object> Data { get { return m_DataStore; } }

		public ObjectDictionaryMessage (IReadOnlyDictionary<string, object> Data, int Priority = 0, IEqualityComparer<string> comparer = null) : base(Data, Priority, comparer)
		{
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal ObjectDictionaryMessage (long Sequence, IReadOnlyDictionary<string, object> Data, int Priority, IEqualityComparer<string> comparer = null) : base(Sequence, Data, Priority, comparer)
		{
		}
	}

	public abstract class ControllerDictionaryMessage : DataDictionaryMessage<string, double>
	{
		public uint ControllerId { get; }

		public ControllerDictionaryMessage (uint ControllerId, IReadOnlyDictionary<string, double> Data, int Priority = 0, IEqualityComparer<string> comparer = null) : base(Data, Priority, comparer)
		{
			if (ControllerId <= 0) throw new ArgumentOutOfRangeException(nameof(ControllerId));
			this.ControllerId = ControllerId;
		}

		/// <remarks>This constructor is internal and only used for deserialization.</remarks>
		[JsonConstructor]
		internal ControllerDictionaryMessage (long Sequence, uint ControllerId, IReadOnlyDictionary<string, double> Data, int Priority, IEqualityComparer<string> comparer = null) : base(Sequence, Data, Priority, comparer)
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