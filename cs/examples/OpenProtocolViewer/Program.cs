using System;
using System.Net.WebSockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace iChen.OpenProtocol.Example
{
	internal static partial class Program
	{
		private static void Main ()
		{
			Console.WriteLine("iChen 4.1 Open Protocol Viewer");
			Console.WriteLine();

			// Read parameters
			Console.Write("WebSocket URL (example: ws://x.x.x.x:port): ");
			var url = Console.ReadLine().Trim();

			Console.Write("Password: ");
			var pwd = Console.ReadLine().Trim();

			// Prepare a cancellation token for the WebSocket connection
			using (var tokensource = new CancellationTokenSource()) {
				var cancel_token = tokensource.Token;

				// Make connection to the server
				using (var task = ConnectToiChenServerAsync(url, pwd, cancel_token)) {
					Console.ReadLine();

					// Terminate program by triggering the cancellation token
					tokensource.Cancel();
					task.Wait();
				}
			}
		}

		private static async Task SendMessageAsync (ClientWebSocket websock, Message msg, CancellationToken ct)
		{
			var json_text = msg.ToJSON();
			var message_data = Encoding.UTF8.GetBytes(json_text);

			await websock.SendAsync(new ArraySegment<byte>(message_data), WebSocketMessageType.Text, true, ct);
			Console.WriteLine($"Message sent: {json_text}");
		}

		private static async Task ConnectToiChenServerAsync (string url, string password, CancellationToken ct)
		{
			Console.WriteLine($"Connecting to iChen Server at {url}...");

			// Create a WebSocket connection
			using (var websock = new ClientWebSocket()) {
				// Connect to the URL
				try {
					await websock.ConnectAsync(new Uri(url), ct);
				} catch (Exception ex) {
					Console.WriteLine($"Cannot connect to iChen Server at {url}: {ex.Message}");
					return;
				}

				Console.WriteLine("Connection to iChen Server established.");

				// Send a JOIN message
				Console.WriteLine("Sending JOIN message...");

				await SendMessageAsync(websock, new JoinMessage(Languages.EN, "4.0", password, null, Filters.All | Filters.JobCards | Filters.Operators), ct);

				// Listen to messages
				Console.WriteLine("Listening to messages from the server...");

				DisplayBuiltIn();
				Console.WriteLine("Press ENTER to quit...");

				var buffer = new byte[10240];
				var segment = new ArraySegment<byte>(buffer);
				var sb = new StringBuilder();

				// Loop until canceled
				for (; !ct.IsCancellationRequested;) {
					try {
						// Receive WebSocket message
						var response = await websock.ReceiveAsync(segment, ct);
						var response_length = response.Count;
						var text = Encoding.UTF8.GetString(segment.Array, 0, response_length);
						Console.WriteLine($"{response_length} byte(s) received: {text}");

						string json_text;

						if (!response.EndOfMessage) {
							sb.Append(text);
							continue;
						} else if (sb.Length == 0) {
							json_text = text;
						} else {
							sb.Append(text);
							json_text = sb.ToString();
							sb.Clear();
						}

						// Handle the message
						var reply_message = HandleMessage(json_text);

						// Send reply message (if any)
						if (reply_message != null) await SendMessageAsync(websock, reply_message, ct);
					} catch (OperationCanceledException) {
						Console.WriteLine("Quitting...");
						break;
					}
				}
			}
		}
	}
}