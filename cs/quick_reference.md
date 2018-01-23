% iChen 4.1 Open Protocol .NET Library Quick Reference
% Chen Hsong
% 2016

iChen^&reg;^ 4.1 Open Protocol&trade; .NET Library Quick Reference
====================================================================

Copyright &copy; Chen Hsong Holdings Ltd.  All rights reserved.  
.NET Framework Required: .NET Standard 1.6  
Document Version: 4.1  
Last Edited: 2018-01-23 


Open Protocol&trade; Messages
-------------------------------

The iChen System^&reg;^ (version 4.1 or above) communicates with third-party 
outside systems via a standardized **WebSocket** interface 
(IETF&nbsp;RFC&nbsp;6455) with messages transmitted in plain text encoded in 
**JSON** format. 

The property `$type` in each JSON-formatted message contains the type of that 
message. 

All message classes in this library inherit from `iChen.OpenProtocol.Message` 
(abstract base class). All message classes are *immutable*, meaning that once 
created, they cannot be altered. 


Creating Messages
-----------------

To create a message, just `new` it:

~~~~~~~~~~~~csharp
JoinMessage join = new JoinMessage (Languages.B5, "2.2", "password", 100);
~~~~~~~~~~~~


Serializing Messages to JSON
----------------------------

To serialize it into JSON, call `ToJSON` on the message class:

~~~~~~~~~~~~csharp
string json = msg.ToJSON();
~~~~~~~~~~~~


Parsing JSON into Messages
--------------------------

To deserialize a piece of JSON into a message, call `Message.ParseJSON`: 

~~~~~~~~~~~~csharp
string text = ...  /* Get JSON text for message */
JoinMessage join = Message.ParseJSON<JoinMessage>(text);
~~~~~~~~~~~~

or

~~~~~~~~~~~~csharp
Message msg = Message.ParseJSON(text);                      // msg is generic Message
JoinMessage join = Message.ParseJSON(text) as JoinMessage;
~~~~~~~~~~~~


Messages that Contain Data Dictionaries
---------------------------------------

Some message types that contain a data dictionary (e.g. `MoldDataMessage`, 
`CycleDataMessage` etc.) in their `Data` property can be used just like a 
`Dictionary`, e.g.: 

~~~~~~~~~~~~csharp
MoldDataMessage msg = new MoldDataMessage(...);   // Dictionary is in msg.Data
object p = msg["Param"];                          // Same as msg.Data["Param"];
int n = msg.Count;                                // Same as msg.Data.Count;
bool x = msg.ContainsKey("Param");                // Same as msg.Data.ContainsKey("Param");
~~~~~~~~~~~~


Creating Data-Dictionary Messages
---------------------------------

Some message types' constructors (e.g. `MoldDataMessage`, `CycleDataMessage` 
etc.) require an `IReadOnlyDictionary` which can be obtained automatically 
from a normal `Dictionary` object. Simply passing in the `Dictionary` will 
work because `Dictionary` implements `IReadOnlyDictionary`: 

~~~~~~~~~~~~csharp
Dictionary<string, object> dict = new Dictionary<string, object>();
dict.Add("Hello", 123);
dict.Add("World", true);
dict.Add("iChen", "Awesome");

CycleDataMessage msg = new CycleDataMessage(99, dict);
~~~~~~~~~~~~

or

~~~~~~~~~~~~csharp
CycleDataMessage msg = new CycleDataMessage(99, new Dictionary<string, object>() {
    { "Hello", 123 }, { "World", true }, { "iChen", "Awesome" }
});
~~~~~~~~~~~~
