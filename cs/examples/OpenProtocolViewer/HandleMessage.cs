using System;
using System.Collections.Generic;
using System.Linq;

namespace iChen.OpenProtocol.Example
{
	internal static partial class Program
	{
		/// <summary>Mock users database mapping user password --> access level (0-10)</summary>
		private static readonly Dictionary<string, byte> UsersDatabase = new Dictionary<string, byte>(StringComparer.InvariantCultureIgnoreCase)
		{
			{ "123456", 10 }, { "000000", 0 }, { "111111", 1 }, { "222222", 2 }, { "333333", 3 },
			{ "444444", 4 }, { "555555", 5 }, { "666666", 6 }, { "777777", 7 }, { "888888", 8 },
			{ "999999", 9 }
		};

		/// <summary>Mock job scheduling system</summary>
		private static readonly JobCard[] Jobs = new JobCard[]
		{
			new JobCard("JOB_CARD_1", "ABC-123", 0, 8000),
			new JobCard("JOB_CARD_2", "M002", 2000, 10000),
			new JobCard("JOB_CARD_3", "MOULD_003", 888, 3333),
			new JobCard("JOB_CARD_4", "MOULD_004", 123, 45678)
		};

		private static void DisplayBuiltIn ()
		{
			Console.WriteLine("=================================================");
			Console.WriteLine("Built-in Users for Testing:");
			foreach (var kv in UsersDatabase) {
				Console.WriteLine($"> Name=MIS_User{kv.Value}, Password={kv.Key}, Level={kv.Value}");
			}
			Console.WriteLine("=================================================");
			Console.WriteLine("Built-in Job Cards for Testing:");
			foreach (var job in Jobs) {
				Console.WriteLine($"> Name={job.JobCardId}, Mold={job.MoldId}, Quantity={job.Progress}/{job.Total}");
			}
			Console.WriteLine("=================================================");
		}

		/// <summary>Handle a JSON-formatted message</summary>
		/// <param name="json">JSON-formatted message</param>
		/// <returns>Reply message, if any</returns>
		private static Message HandleMessage (string json)
		{
			Message message;

			// Parse message

			try {
				message = Message.ParseJSON(json);
				Console.WriteLine($"Parsed message = {message}");
			} catch (Exception ex) {
				Console.WriteLine($"Error parsing message: {ex}");
				return null;
			}

			// Check message type

			if (message is AliveMessage) {
				// Send an ALIVE when received one
				return new AliveMessage();
			} else if (message is JoinResponseMessage) {
				// Send a REQ_CNTRLER_LIST message
				return new RequestControllersListMessage();
			} else if (message is LoginOperatorMessage) {
				var operator_message = message as LoginOperatorMessage;

				// MIS integration - return access level
				if (!UsersDatabase.ContainsKey(operator_message.Password)) {
					Console.WriteLine($"No user found with password: {operator_message.Password}.");
					return new OperatorInfoMessage(operator_message.ControllerId, 0, "Not allowed", operator_message.Password, 0);
				} else {
					var access_level = UsersDatabase[operator_message.Password];
					Console.WriteLine($"User found: password={operator_message.Password}, access level={access_level}.");
					return new OperatorInfoMessage(operator_message.ControllerId, uint.Parse(operator_message.Password), "MIS_User" + access_level, operator_message.Password, access_level);
				}
			} else if (message is RequestJobCardsListMessage) {
				var jobcard_message = message as RequestJobCardsListMessage;

				// MIS integration - return job cards list
				return new JobCardsListMessage(jobcard_message.ControllerId, Jobs.ToDictionary(job => job.JobCardId));
			}

			return null;
		}
	}
}