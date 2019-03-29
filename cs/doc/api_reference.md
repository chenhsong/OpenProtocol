iChen® 4 Open Protocol™ .NET Library API Reference
==================================================

Copyright © Chen Hsong Holdings Ltd.  All rights reserved.  
`iChen.OpenProtocol.dll` version: 4.1 and up  
Document Version: 4.1  
Last Edited: 2018-01-23


Library DLL
-----------

Assembly name: `iChen.OpenProtocol.dll`  
Main namespace: `iChen.OpenProtocol`  
Dependencies: `Json.NET` (`Newtonsoft.Json` via NuGet)  

The .NET Framework required for this assembly is .NET Standard 1.6 or above.


WebSocket Communications
------------------------

All communications with the iChen® Server is performed through an
industry-standard WebSocket interface (IETF RFC 6455).

The default port (configurable) of the WebSocket interface is 5788.

Secured WebSocket connections with TLS/SSL encryption (via protocol `wss://`)
are also supported.

Use your favorite WebSocket client library to connect to the server via
WebSocket. For example (assuming the iChen® server resides at the
URL `ichen.example.com`), a C# client may connect to the server like the
following:

~~~~~~~~~~~~csharp
using System.Net.WebSockets;    // Note: This namespace only works for Windows 8 and up

          :
          :

// Create a WebSocket client

using (ClientWebSocket websock = new ClientWebSocket())
{
    // Create a cancellation token if necessary
    // Note: This using block can be omitted if not using cancellation tokens

    using (CancellationTokenSource cts = new CancellationTokenSource())
    {
        CancellationToken ct = cts.Token;

        // Connect to the WebSocket server
        // Note: A WebSocket url always starts with the ws:// protocol

        await websock.ConnectAsync("ws://ichen.example.com:5788", ct);
                     :
                     :
    }
}
~~~~~~~~~~~~

Beware that `System.Net.WebSockets` is available only for the .NET Framework
starting from version 4.5 and for Windows 8 and up only. For Windows 7 or
below, use a third-party WebSocket client library such as `SuperWebSocket`.


How the iChen® Server Processes Messages
----------------------------------------

The iChen® server is a *massively parallel* execution engine, which
means that messages are not guaranteed to be processed *in order*. In
addition, messages of higher priority are always processed before messages of
lower priority, as much as possible.

Therefore, **no assumption** should be made regarding to the order of message
processing. If message sequence order is significant, the client should send
messages one by one and only after receiving confirmations/replies on
previous messages.


iChen.OpenProtocol.Message
--------------------------

### `static Message ParseJSON(String json)`

#### Usage

Call this static method to parse a JSON-encoded message into a particular
message class object. The message class is automatically inferred by the
information in the JSON message.

#### Parameters

| Parameter | Description                    |
|-----------|--------------------------------|
| `json`    | Plain-text JSON-format message |

#### Return Value

A `Message`-based class representing the iChen® message.

#### Example (C#)

~~~~~~~~~~~~csharp
// Create a buffer large enough to hold the message (say 10MB)
byte[] buffer = new byte[10240];
// Receive the WebSocket message into the bubber
ArraySegment<byte> segment = new ArraySegment<byte>(buffer);
WebSocketReceiveResult result = await websock.ReceiveAsync(segment, ct);
// Decode the message based on UTF-8
string json = System.Encoding.UTF8.GetString(segment.Array, 0, result.Count);
// Parse the JSON message into a message class
Message message = Message.ParseJSON(json);
// Assuming here that the JSON message encodes a CycleData message...
CycleDataMessage cycle = (CycleDataMessage) message;    // Will throw if message is not CycleDataMessage
~~~~~~~~~~~~


### `static Message ParseJSON<T>(String json)`

#### Usage

Call this static method to parse a JSON-encoded message into a particular
message class object. The type parameter `T` should be the message type class.

#### Parameters

| Parameter | Description                                |
|-----------|--------------------------------------------|
| `T`       | A message type that must inherit `Message` |
| `json`    | Plain-text JSON-format message             |

#### Return Value

A `Message`-based class of type `T` representing the iChen® message.

#### Example (C#)

~~~~~~~~~~~~csharp
// Create a buffer large enough to hold the message (say 10MB)
byte[] buffer = new byte[10240];
// Receive the WebSocket message into the buffer
ArraySegment<byte> segment = new ArraySegment<byte>(buffer);
WebSocketReceiveResult result = await websock.ReceiveAsync(segment, ct);
// Decode the message based on UTF-8
string json = System.Encoding.UTF8.GetString(segment.Array, 0, result.Count);
// Parse the JSON message into a CycleDataMessage
// Note: This call will throw if the JSON message is not a CycleData message
CycleDataMessage cycle = Message.ParseJSON<CycleDataMessage>(json);
~~~~~~~~~~~~


### `String ToJSON()`

#### Usage

Call this method on a message class to encode it into JSON.

#### Parameters

None.

#### Return Value

The message in plain-text JSON format.

#### Example (C#)

~~~~~~~~~~~~csharp
// Create a message class object, say, JoinMessage
JoinMessage message = new JoinMessage(Languages.B5, "4.1", "password", Filters.All);
// Encode the message into JSON
string json = message.ToJSON();
// Encode the JSON text as UTF-8
byte[] data = System.Encoding.UTF8.GetBytes(json);
// Send the UTF-8 byte stream to the WebSocket
ArraySegment<byte> segment = new ArraySegment<byte>(data);
await websock.SendAsync(segment, WebSocketMessageType.Text, true, ct);
~~~~~~~~~~~~


Helper Method to Send Messages
------------------------------

~~~~~~~~~~~~csharp
static async Task SendMessageAsync (ClientWebSocket websock, Message msg, CancellationToken ct = null)
{
    // Check parameters
    if (websock == null) throw new ArgumentNullException(nameof(websock));
    if (msg == null) throw new ArgumentNullException(nameof(msg));

    // Encode the message into JSON
    var json_text = msg.ToJSON();

    // Encode the JSON message as UTF-8
    var message_data = System.Encoding.UTF8.GetBytes(json_text);

    // Send the UTF-8 byte stream to the WebSocket connection
    await websock.SendAsync(new ArraySegment<byte>(message_data), WebSocketMessageType.Text, true, ct);
}
~~~~~~~~~~~~
