iChen&reg; 4.1 Open Protocol&trade; .NET Library Messages Reference
=======================================================================

Copyright &copy; Chen Hsong Holdings Ltd.  All rights reserved.  
.NET Framework Required: .NET Standard 1.6  
For `iChen.OpenProtocol.dll` version: 4.1.1 and up  
Document Version: 4.1.1  
Last Edited: 2018-06-30


Introduction
------------

The iChen&reg; System 4.1 publishes an open communications protocol for 
third-party connectivity. An external system communicates with the 
iChen&reg; System via industry-standard WebSocket (IETF&nbsp;RFC&nbsp;6455) 
connections with text-based payloads. All messages passed in the protocol are 
serialized to plain-text in **JSON** format. 

To assist in connectivity, an access library is provided for the Microsoft 
.NET Framework. The library contains types, interfaces and classes useful for 
constructing, serializing and parsing **JSON**-formatted messages. 

![](comms_chart.png)


Enum Types
----------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  

| Type Name |Description                   |Flag? |
|:----------|:-----------------------------|:----:|
|[`Languages`](../../doc/enums.md#languages)|Language encoding codes       |No    |
|[`ControllerTypes`](../../doc/enums.md#controllertypes)|Type(s) of controllers  |No    |
|[`OpModes`](../../doc/enums.md#opmodes)|Operation modes               |No    |
|[`JobModes`](../../doc/enums.md#jobmodes)|Job modes                     |No    |
|[`Filters`](../../doc/enums.md#filters)|Type(s) of messages to receive|Yes   |


Controller
----------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  

### Description

An *immutable* class containing information on a controller (i.e. machine). 

### Properties

|Property Name       |.NET Type |JSON Property   |JSON Type|           Description               |
|:-------------------|:--------:|:--------------:|:-------:|:------------------------------------|
|`ControllerId`      |`UInt32`  |`controllerId`  |`number` |Unique numeric ID for this controller|
|`DisplayName`       |`String`  |`displayName`   |`string` |Human-friendly name for display      |
|`ControllerType`|[`ControllerTypes`](../../doc/enums.md#controllertypes) enum|`controllerType`|`string`|Type of controller            |
|`Version`           |`String`  |`version`       |`string` |The version of the controller        |
|`Model`             |`String`  |`model`         |`string` |The machine model                    |
|`IP`                |`String`  |`IP`            |`string` |IP address of the controller, in the format "`x.x.x.x`"|
|`OpMode`        |[`OpModes`](../../doc/enums.md#opmodes) enum|`opMode`        |`string` |Current operation mode of the controller|
|`JobMode`      |[`JobModes`](../../doc/enums.md#jobmodes) enum|`jobMode`       |`string` |Current job mode of the controller   |
|`JobCardId`         |`String`  |`jobCardId`     |`string` |Unique ID of the current job card loaded (if any)|
|`LastCycleData`     |`IReadOnlyDictionary` `<String,Double>`|`lastCycleData`|`object`|A data dictionary (if any) containing the last set of cycle data on the controller|
|`Variables`         |`IReadOnlyDictionary` `<String,Double>`|`variables`|`object`|A data dictionary (if any) containing the latest values of all pollable variables on the controller|
|`LastConnectionTime`|`DateTime`|`lastConnectionTime`|`string`|Last time of connection for the controller (in ISO-8601 format)|
|`OperatorId`        |`UInt32`  |`operatorId`    |`number` |Unique ID of the current logged-on operator, or zero if no operator is logged on|
|`OperatorName`      |`String`  |`operatorName`  |`string` |Name of the current logged-on operator, or `null` if no operator is logged on or if the name is not available|
|`MoldId`            |`String`  |`moldId`        |`string` |Unique ID of the current mold data set loaded (if any)|

### JSON Format Example

~~~~~~~~~~~~json
{
  "controllerId":123,
  "displayName":"M1",
  "controllerType":"Ai02",
  "version":"Ai-02",
  "model":"JM138-Ai",
  "IP":"192.168.1.123",
  "opMode":"Automatic",
  "jobMode":"ID08",
  "jobCardId":"XYZ",
  "lastCycleData": {
    "INJEND":401.28,
    "CYCTIME":21.54
  },
  "variables": {
    "RT_TempZ1":231.5,
    "RT_Pump":1.0
  },
  "lastConnectionTime":"2016-01-01T12:23:34+08:00",
  "operatorId":99,
  "operatorName":"Johnny",
  "moldId":"ABC123"
}
~~~~~~~~~~~~


JSON Conversion Behavior
------------------------

* Data dictionaries (i.e. `Dictionary<string, object>`) are serialized as 
  normal JSON object hashes with property names being the keys of entries in 
  the data dictionaries. 

* Supported primitive data types are 32-bit integers (`int`), 64-bit double 
  precision floating point numbers (`double`), boolean values (`bool`), 
  UTF8-encoded straight text (`string`), date/time values (`DateTime`), as 
  well as binary objects in form of byte arrays (`byte[]`). 

* All whole numbers in JSON object hashes that maps to corresponding 
  `Dictionary<string, object>` properties are parsed into 32-bit integers. 
  Fractional numbers are parsed into `double`'s. 

* Date/time values are serialized in ISO-8601 standard format.

* Byte arrays in data dictionaries are serialized into Base64-encoded strings 
  prefixed with "`base64:`". 

### Example

JSON message:

~~~~~~~~~~~~json
{
  "$type":"CycleData",
  "controllerId":234,
  "sequence":123,
  "priority":10,
  "data":{
    "TIME": "2016-02-26T01:12:23+08:00",
    "INJEND":401.28,
    "CYTIME":12.0,
    "COUNT": 42,
    "GOODPRT": true,
    "MESSAGE": "This part is perfect.",
    "PICTURE": "base64:XYZABCDEF1234567890"
  }
}
~~~~~~~~~~~~

C# Code:

~~~~~~~~~~~~csharp
CycleDataMessage msg = Message.ParseJSON<CycleDataMessage>(json);
DateTime time = (DateTIme) msg["TIME"];    // TIME is DateTime
double injend = (double) msg["INJEND"];    // INJEND is double
double cytime = (double) msg["CYTIME"];    // CYTIME is still double
int count = (int) msg["COUNT"];            // COUNT is int
bool good = (bool) msg["GOODPRT"];         // GOODPRT is bool
string mesg = (string) msg["MESSAGE"];     // MESSAGE is string
byte[] pic = (byte[]) msg["PICTURE"];      // PICTURE is byte array
~~~~~~~~~~~~


Message (abstract base class)
-----------------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  

### Description

This class is the abstract base class for all iChen&reg; 4.1 Open 
Protocol&trade; message type classes. 

All message classes are *immutable*. 

### Properties

|Property Name       |.NET Type|JSON Property|JSON Type|           Description                       |
|:-------------------|:-------:|:-----------:|:-------:|:--------------------------------------------|
|`Sequence`          |`Int64`  |`sequence`   |`number` |A increasing sequence number for each message|
|`Priority`          |`Int32`  |`priority`   |`number` |A number representing the priority of each message (default=0); higher priority messages are sent before lower priority messages|

### Instance Methods

|Property Name       |Returns |           Description         |
|:-------------------|:------:|:------------------------------|
|Constructor         |        |Constructor                    |
|`ToJSON()`          |`String`|Serialize the message into JSON plain-text format|

### Static Methods

|Property Name              |Returns         |           Description                   |
|:--------------------------|:--------------:|:----------------------------------------|
|`ParseJSON(String json)`   |`Message`       |Parse a message in JSON plain-text format|
|`ParseJSON<T>(String json)`|`T` is `Message`|Parse a message in JSON plain-text format into the specified message type|

### JSON Format Example

~~~~~~~~~~~~json
{ "$type":"Message", "sequence":123, "priority":10 }
~~~~~~~~~~~~


JoinMessage
-----------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.Message`

### Description

This class implements the `JOIN` request message, which must be sent to the 
iChen&reg; 4.1 Server immediately after establishing a connection. 

If this message is not sent, or if the server fails to authenticate the 
connection, then the server may or may not terminate the connection and/or 
refuse to send further messages. 

If this request is successful, the server responds with a [`JoinResponseMessage`](#joinresponsemessage) 
message. 

### Properties

|Property Name       |.NET Type|JSON Property|JSON Type|           Description         |
|:-------------------|:-------:|:-----------:|:-------:|:------------------------------|
|`Sequence`          |`Int64`  |`sequence`   |`number` |*Inherited from [`Message`](#message)*     |
|`Priority`          |`Int32`  |`priority`   |`number` |*Inherited from [`Message`](#message)*     |
|`Language`   |[`Languages`](../../doc/enums.md#languages) enum|`language`   |`string` |Language encoding              |
|`Version`           |`String` |`version`    |`string` |The maximum protocol version supported, in the format "`4.1.2.3`"|
|`OrgId`             |`String` |`orgId`      |`string` |Organization ID (if any)       |
|`Password`          |`String` |`password`   |`string` |Password to join the server    |
|`Filter`      |[`Filters`](../../doc/enums.md#filters) enum |`filter`     |`string` |A filter (if any) containing what type(s) of messages to receive|

### Filter

The `Filter` property is serialized to JSON as a *comma-separated* string 
containing the type(s) of messages interested in receiving: 

|Filter      |Message Type                       |Message Class(es) Affected|
|:----------:|:----------------------------------|:-----------------------:|
|`None`      |Nothing                            |*N/A*                    |
|`Status`    |Controller status and variables    |[`ControllerStatusMessage`](#controllerstatusmessage)|
|`Cycle`     |Cycle data                         |[`CycleDataMessage`](#cycledatamessage)|
|`Mold`      |Mold settings                      |[`MoldDataMessage`](#molddatamessage)|
|`Actions`   |Current action                     |[`ControllerActionMessage`](#controlleractionmessage)|
|`Alarms`    |Controller alarms                  |[`ControllerStatusMessage`](#controllerstatusmessage)|
|`Audit`     |Audit trail of setting changes     |[`ControllerStatusMessage`](#controllerstatusmessage)|
|`All`       |All message types                  |All message classes above|
|`JobCards`  |Job card-related messages          |[`RequestJobCardsListMessage`](#requestjobcardslistmessage), [`JobCardsListMessage`](#jobcardslistmessage)|
|`Operators` |Operator-related messages          |[`LoginOperatorMessage`](#loginoperatormessage), [`OperatorInfoMessage`](#operatorinfomessage)|
|`OPCUA`     |OPC UA communications              |*N/A*                    |

If `Filter` is `null`, then *all* messages will be sent, except MIS 
integration messages such as `JobCards`, `Operators` and `OPCUA` -- i.e. 
similar to `All`. 

### Authentication and Authorization

If the `Password` provided is not authenticated or not authorized to access 
the server, then a [`JoinResponseMessage`](#joinresponsemessage) will be sent 
by the server with an error result code. No further messages will be sent by 
the server, and the server may terminate the connection. 

If the `Password` provided is authenticated on the server but not all of the 
message types specified in `Filter` is authorized for that password, then only 
the authorized messages will be sent by the server, effectively limiting the 
`Filter` to only the authorized subset. 

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"Join",
  "language":"EN",
  "version":"4.1",
  "password":"xxxxxxx",
  "filter":"Status, Cycle, Alarms, Audit",
  "sequence":123,
  "priority":10
}
~~~~~~~~~~~~


JoinResponseMessage
-------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.Message`

### Description

This class implements the `RESP_JOIN` message, which is sent from the 
iChen&reg; 4.1 Server to a client in response to a [`JoinMessage`](#joinmessage) 
message. 

### Properties

|Property Name       |.NET Type|JSON Property|JSON Type|           Description                  |
|:-------------------|:-------:|:-----------:|:-------:|:---------------------------------------|
|`Sequence`          |`Int64`  |`sequence`   |`number` |*Inherited from [`Message`](#message)*              |
|`Priority`          |`Int32`  |`priority`   |`number` |*Inherited from [`Message`](#message)*              |
|`Result`            |`UInt32` |`result`     |`number` |Result code, >= 100 indicates success   |
|`Level`             |`UInt32` |`level`      |`number` |The allowed access level for this client|
|`Message`           |`String` |`message`    |`string` |A message (mostly likely an error message in case of failure) to the client, if any|
|`Succeeded`         |`Boolean`|             |         |Success (`Result` >= 100) or failure (`Result` < 100)|

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"JoinResponse",
  "result":100,
  "level":5,
  "message":"Succeeded, your access level is a lowly 5.",
  "sequence":123,
  "priority":10
}
~~~~~~~~~~~~


AliveMessage
------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.Message`

### Description

This class implements the `ALIVE` message, which must be sent to the 
iChen&reg; 4.1 Server at regular intervals. 

If the server does not receive this message after a time-out period 
(configurable, initially set to 10 seconds), the client is assumed to be dead 
and the server may stop sending updates to the client. The server then may or 
may not terminate the connection. 

### JSON Format Example

~~~~~~~~~~~~json
{ "$type":"Alive", "sequence":123, "priority":10 }
~~~~~~~~~~~~


LoginOperatorMessage
--------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.Message`

### Description

This class implements the `REQ_PWD_LEVEL` request message, which is sent by 
the iChen&reg; 4.1 Server when a connected controller attempts to 
authenticate and authorize a user password. 

The client should respond with a [`OperatorInfoMessage`](#operatorinfomessage) 
message. 

### Properties

|Property Name       |.NET Type|JSON Property|JSON Type|           Description         |
|:-------------------|:-------:|:-----------:|:-------:|:------------------------------|
|`Sequence`          |`Int64`  |`sequence`   |`number` |*Inherited from [`Message`](#message)*     |
|`Priority`          |`Int32`  |`priority`   |`number` |*Inherited from [`Message`](#message)*     |
|`ControllerId`      |`UInt32` |`controllerId`|`number`|Unique ID of the controller    |
|`Password`          |`String` |`password`   |`string` |User password                  |

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"LoginOperator",
  "controllerId":234,
  "password":"xxxxxxx",
  "sequence":123,
  "priority":10
}
~~~~~~~~~~~~


OperatorInfoMessage
-------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.Message`

### Description

This class implements the `RESP_PWD_LEVEL` message, which is to the 
iChen&reg; 4.1 Server in response to a `LoginOperator` message. 

### Properties

|Property Name       |.NET Type|JSON Property|JSON Type|           Description                  |
|:-------------------|:-------:|:-----------:|:-------:|:---------------------------------------|
|`Sequence`          |`Int64`  |`sequence`   |`number` |*Inherited from [`Message`](#message)*              |
|`Priority`          |`Int32`  |`priority`   |`number` |*Inherited from [`Message`](#message)*              |
|`ControllerId`      |`UInt32` |`controllerId`|`number`|Unique ID of the controller             |
|`OperatorId`        |`UInt32` |`operatorId` |`number` |Unique ID of the user                   |
|`Password`          |`String` |`password`   |`string` |User password                           |
|`Name`              |`String` |`name`       |`string` |Name of the user                        |
|`Level`             |`Byte`   |`level`      |`number` |The allowed access level for this user (typically 0-10)|

### Authentication and Authorization

The client should authenticate the user based on the provided `ControllerId` 
and the `Password` in the [`LoginOperatorMessage`](#loginoperatormessage) 
request message. 

If the `Password` provided is not authenticated or not authorized to access 
the controller, then this [`OperatorInfoMessage`](#operatorinfomessage) should 
be sent to the iChen&reg; 4.1 Server with the lowest access level (default 
to zero). 

If the `Password` provided is authenticated and authorized to access the 
controller, then this [`OperatorInfoMessage`](#operatorinfomessage) should be 
sent to the iChen&reg; 4.1 Server with the appropriate access level 
(typically 0-10). 

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"OperatorInfo",
  "controllerId":234,
  "operatorId":987,
  "password":"xxxxxxx",
  "name":"Johnny",
  "level":5,
  "sequence":123,
  "priority":10
}
~~~~~~~~~~~~


RequestControllersListMessage
-----------------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.Message`

### Description

This class implements the `REQ_CNTRLER_LIST` message, which is sent to the 
iChen&reg; 4.1 Server to request a list of controllers (i.e. machines) 
currently connected to the server. 

The server responds with a [`ControllersListMessage`](#controllerslistmessage) 
message. 

### JSON Format Example

~~~~~~~~~~~~json
{ "$type":"RequestControllersList", "sequence":123, "priority":10 }
~~~~~~~~~~~~


ControllersListMessage
----------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.DataDictionaryMessage<UInt32, Controller>` : `iChen.OpenProtocol.Message`

### Description

This class implements the `RESP_CNTRLER_LIST` message, which is sent from the 
iChen&reg; 4.1 Server to the client in response to the [`RequestControllersListMessage`](#requestcontrollerslistmessage) 
message. 

### Properties

|Property Name       |.NET Type|JSON Property|JSON Type|           Description         |
|:-------------------|:-------:|:-----------:|:-------:|:------------------------------|
|`Sequence`          |`Int64`  |`sequence`   |`number` |*Inherited from [`Message`](#message)*     |
|`Priority`          |`Int32`  |`priority`   |`number` |*Inherited from [`Message`](#message)*     |
|`Data`              |`IReadOnlyDictionary` `<UInt32,Controller>`|`data`|`object`|A data dictionary containing all connected controllers.|
|`Count`             |`Int32`  |             |         |Same as `Data.Count`           |

### Instance Methods

|Property Name                                  |Returns                  |           Description         |
|:----------------------------------------------|:-----------------------:|:------------------------------|
|Constructor                                    |                         |Constructor                    |
|`ToJSON()`                                     |`String`                 |*Inherited from [`Message`](#message)*     |
|`ContainsKey(UInt32 key)`                      |`Boolean`                |Same as `Data.ContainsKey`     |
|`TryGetValue(UInt32 key, out Controller value)`|`Boolean`                |Same as `Data.TryGetValue`     |
|`Keys`                                         |`IEnumerable<UInt32>`    |Same as `Data.Keys`            |
|`Values`                                       |`IEnumerable<Controller>`|Same as `Data.Values`          |

### Indexers

|Indexer             |Returns |           Description         |
|:-------------------|:------:|:------------------------------|
|`this[UInt32 key]`  |`Controller`|Same as `Data[key]`|

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"ControllersList",
  "sequence":123,
  "priority":10,
  "data":{
    "123": {
             "controllerId":123,
             "displayName":"M1",
             "controllerType":"Ai02",
             "version":"Ai-12",
             "model":"JM138-Ai",
             "IP":"192.168.1.123",
             "opMode":"Automatic",
             "jobMode":"ID08",
             "jobCardId":"XYZ",
             "lastCycleData": {
               "INJEND":401.28,
               "CYCTIME":21.54
             },
             "variables": {
               "RT_TempZ1":231.5,
               "RT_Pump":1.0
             },
             "lastConnectionTime":"2016-01-01T12:23:34+08:00",
             "operatorId":99,
			 "operatorName":"Johnny",
             "moldId":"ABC123"
           },
    "234": {
             "controllerId":234,
             "displayName":"M2",
             "controllerType":"Ai11",
             "version":"Ai-02",
             "model":"EM80-V",
             "IP":"192.168.1.234",
             "opMode":"Manual",
             "jobMode":"Offline",
             "lastCycleData": {
               "INJEND":129.8,
               "CYCTIME":7.33
             },
             "variables": {
               "RT_TempOil":45.2,
               "RT_CPT0":100.0
             },
             "lastConnectionTime":"2016-01-01T00:11:12+08:00",
             "operatorId":0
           }
  }
}
~~~~~~~~~~~~


RequestJobCardsListMessage
--------------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.Message`

### Description

This class implements the `REQ_JOBCARDS_LIST` message, which is sent by the 
iChen&reg; 4.1 Server when a connected controller requests a list of job 
cards. 

The client should respond with a [`JobCardsListMessage`](#jobcardslistmessage) 
message. 

### Properties

|Property Name       |.NET Type|JSON Property |JSON Type|           Description         |
|:-------------------|:-------:|:------------:|:-------:|:------------------------------|
|`Sequence`          |`Int64`  |`sequence`    |`number` |*Inherited from [`Message`](#message)*     |
|`Priority`          |`Int32`  |`priority`    |`number` |*Inherited from [`Message`](#message)*     |
|`ControllerId`      |`UInt32` |`controllerId`|`number` |Unique ID of the controller    |

### JSON Format Example

~~~~~~~~~~~~json
{ "$type":"RequestJobCardsList", "controllerId":345, "sequence":123, "priority":10 }
~~~~~~~~~~~~


JobCardsListMessage
-------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.DataDictionaryMessage<String, JobCard>` : `iChen.OpenProtocol.Message`

### Description

This class implements the `RESP_JOBSLIST` message, which should be sent to the 
iChen&reg; 4.1 Server in response to a [`RequestJobCardsListMessage`](#requestjobcardslistmessage) 
message. 

### Properties

|Property Name       |.NET Type|JSON Property |JSON Type|           Description         |
|:-------------------|:-------:|:------------:|:-------:|:------------------------------|
|`Sequence`          |`Int64`  |`sequence`    |`number` |*Inherited from [`Message`](#message)*     |
|`Priority`          |`Int32`  |`priority`    |`number` |*Inherited from [`Message`](#message)*     |
|`ControllerId`      |`UInt32` |`controllerId`|`number` |Unique ID of the controller    |
|`Data`              |`IReadOnlyDictionary` `<String,JobCard>`|`data`|`object`|A data dictionary containing a set of job cards.|
|`Count`             |`Int32`  |              |         |Same as `Data.Count`           |

### Instance Methods

|Property Name                              |Returns              |           Description         |
|:------------------------------------------|:-------------------:|:------------------------------|
|Constructor                                |                     |Constructor                    |
|`ToJSON()`                                 |`String`             |*Inherited from [`Message`](#message)*     |
|`ContainsKey(String key)`                  |`Boolean`            |Same as `Data.ContainsKey`     |
|`TryGetValue(String key, out JobCard value)`|`Boolean`           |Same as `Data.TryGetValue`     |
|`Keys`                                     |`IEnumerable<String>`|Same as `Data.Keys`            |
|`Values`                                  |`IEnumerable<JobCard>`|Same as `Data.Values`          |

### Indexers

|Indexer             |Returns  |           Description         |
|:-------------------|:-------:|:------------------------------|
|`this[String key]`  |`JobCard`|Same as `Data[key]`            |

### Job Card

A **Job Card** is a unit of work to be performed by the machine, typically a 
production order for a particular product using a particular mold for a 
specified number of pieces or runs. 

Job cards information are typically provided by an MIS (manufacturing 
information system). The MIS assigns a particular production order or project 
run to a particular machine, together with a mold for the job. The controller 
on the machine loads the mold settings data automatically. 

A machine can only select from a list of job cards assigned to it by the MIS. 
Typically the machine is not allowed to start any other production or load 
other mold data sets. 

The MIS keeps track of the amount of production already completed (based on 
the received *cycle data*). The controller on the machine also keeps track of 
the production progress, and should stop the machine automatically when the 
total order quantity is reached, typically also raising an alarm. 

#### Properties

|Property Name       |.NET Type|JSON Property |JSON Type|           Description         |
|:-------------------|:-------:|:------------:|:-------:|:------------------------------|
|`JobCardId`         |`String` |`jobCardId`   |`string` |Unique ID of this job card.    |
|`MoldId`            |`String` |`moldId`      |`string` |Unique ID of the mold data set for this job card.|
|`Progress`          |`UInt32` |`progress`    |`number` |The already-completed production quantity for this job card. When `Progress` is greater than or equals to `Total`, this job card is supposed to be completed.|
|`Total`             |`UInt32` |`total`       |`number` |The total production quantity for this job card.|

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"JobCardsList",
  "controllerId":234,
  "sequence":123,
  "priority":10,
  "data":{
    "ABC001": { "jobCardId":"ABC001", "moldId":"XYZ", "total":5000, "progress":2000 },
    "ABC002": { "jobCardId":"ABC002", "moldId":"WWW", "total":10000, "progress":100 },
    "XYZ-123": { "jobCardId":"XYZ-123", "moldId":"Test", "total":2000, "progress":0 }
  }
}
~~~~~~~~~~~~




CycleDataMessage
----------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.ControllerDictionaryMessage` : `iChen.OpenProtocol.DataDictionaryMessage<String, Double>` : `iChen.OpenProtocol.Message`

### Description

This class implements the `CYCLE_DATA` message, which is sent from the 
iChen&reg; 4.1 Server to the client at the end of each machine cycle for all 
connected controllers. 

### Properties

|Property Name       |.NET Type|JSON Property |JSON Type|           Description            |
|:-------------------|:-------:|:------------:|:-------:|:---------------------------------|
|`Sequence`          |`Int64`  |`sequence`    |`number` |*Inherited from [`Message`](#message)*        |
|`Priority`          |`Int32`  |`priority`    |`number` |*Inherited from [`Message`](#message)*        |
|`TimeStamp`        |`DateTIme`|`timestamp`   |`string` |Date/time (in ISO-8601 format)    |
|`ControllerId`      |`UInt32` |`controllerId`|`number` |*Inherited from [`MoldDataMessage`](#molddatamessage)*|
|`JobCardId`         |`String` |`jobCardId`   |`string` |Unique ID of the current job card loaded (if any)|
|`MoldId`            |`String` |`moldId`      |`string` |Unique ID of the current mold data set loaded (if any)|
|`OperatorId`|`Nullable<UInt32>`|`operatorId` |`number` |Unique ID of the current logged-on user, zero if no user is logged on|
|`Data`              |`IReadOnlyDictionary` `<String,Double>`|`data`|`object`|A data dictionary containing a set of cycle data|
|`Count`             |`Int32`  |              |         |Same as `Data.Count`              |

### Instance Methods

|Property Name                              |Returns              |           Description         |
|:------------------------------------------|:-------------------:|:------------------------------|
|Constructor                                |                     |Constructor                    |
|`ToJSON()`                                 |`String`             |*Inherited from [`Message`](#message)*     |
|`ContainsKey(String key)`                  |`Boolean`            |Same as `Data.ContainsKey`     |
|`TryGetValue(String key, out Double value)`|`Boolean`            |Same as `Data.TryGetValue`     |
|`Keys`                                     |`IEnumerable<String>`|Same as `Data.Keys`            |
|`Values`                                   |`IEnumerable<Double>`|Same as `Data.Values`          |

### Indexers

|Indexer             |Returns |           Description         |
|:-------------------|:------:|:------------------------------|
|`this[String key]`  |`Double`|Same as `Data[key]`            |

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"CycleData",
  "timestamp":"2016-04-01T01:12:23+08:00",
  "controllerId":234,
  "jobCardId":"XYZ",
  "operatorId":987,
  "moldId":"ABC123",
  "sequence":123,
  "priority":10,
  "data":{
    "INJEND":401.28,
    "CYCTIME":21.54
  }
}
~~~~~~~~~~~~


RequestMoldDataMessage
----------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.Message`

### Description

This class implements the `REQ_MOLD` message, which is sent to the 
iChen&reg; 4.1 Server to request the set of mold settings data of a 
controller currently connected to the server. 

The server responds with a [`MoldDataMessage`](#molddatamessage) message. 

### Properties

|Property Name       |.NET Type|JSON Property |JSON Type|           Description         |
|:-------------------|:-------:|:------------:|:-------:|:------------------------------|
|`Sequence`          |`Int64`  |`sequence`    |`number` |*Inherited from [`Message`](#message)*     |
|`Priority`          |`Int32`  |`priority`    |`number` |*Inherited from [`Message`](#message)*     |
|`ControllerId`      |`UInt32` |`controllerId`|`number` |Unique ID of the controller    |

### JSON Format Example

~~~~~~~~~~~~json
{ "$type":"RequestMoldData", "sequence":123, "controllerId":789, "priority":10 }
~~~~~~~~~~~~


MoldDataMessage
---------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.CycleDataMessage` : `iChen.OpenProtocol.ControllerDictionaryMessage` : `iChen.OpenProtocol.DataDictionaryMessage<String, Double>` : `iChen.OpenProtocol.Message`

### Description

This class implements the `RESP_MOLD` message, which is sent from the 
iChen&reg; 4.1 Server to the client in response to a [`RequestMoldDataMessage`](#requestmolddatamessage) 
message. 

Mold settings can be numerous (i.e. more than 1,000), but not all 
variables/fields are used. In order to reduce payload size, variables/fields 
with zero values are not included in the resultant data dictionary. 

### Properties

|Property Name       |.NET Type|JSON Property |JSON Type|           Description            |
|:-------------------|:-------:|:------------:|:-------:|:---------------------------------|
|`Sequence`          |`Int64`  |`sequence`    |`number` |*Inherited from [`Message`](#message)*        |
|`Priority`          |`Int32`  |`priority`    |`number` |*Inherited from [`Message`](#message)*        |
|`TimeStamp`        |`DateTIme`|`timestamp`   |`string` |Date/time (in ISO-8601 format)    |
|`ControllerId`      |`UInt32` |`controllerId`|`number` |*Inherited from [`MoldDataMessage`](#molddatamessage)*|
|`JobCardId`         |`String` |`jobCardId`   |`string` |Unique ID of the current job card loaded (if any)|
|`MoldId`            |`String` |`moldId`      |`string` |Unique ID of the current mold data set loaded (if any)|
|`OperatorId`|`Nullable<UInt32>`|`operatorId` |`number` |Unique ID of the current logged-on user, zero if no user is logged on|
|`Data`              |`IReadOnlyDictionary` `<String,Double>`|`data`|`object`|A data dictionary containing a set of mold settings |
|`Count`             |`Int32`  |              |         |Same as `Data.Count`              |

### Instance Methods

|Property Name                              |Returns              |           Description         |
|:------------------------------------------|:-------------------:|:------------------------------|
|Constructor                                |                     |Constructor                    |
|`ToJSON()`                                 |`String`             |*Inherited from [`Message`](#message)*     |
|`ContainsKey(String key)`                  |`Boolean`            |Same as `Data.ContainsKey`     |
|`TryGetValue(String key, out Double value)`|`Boolean`            |Same as `Data.TryGetValue`     |
|`Keys`                                     |`IEnumerable<String>`|Same as `Data.Keys`            |
|`Values`                                   |`IEnumerable<Double>`|Same as `Data.Values`          |

### Indexers

|Indexer             |Returns |           Description         |
|:-------------------|:------:|:------------------------------|
|`this[String key]`  |`Double`|Same as `Data[key]`            |

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"MoldData",
  "timestamp":"2016-04-01T01:12:23+08:00",
  "controllerId":234,
  "jobCardId":"XYZ",
  "operatorId":987,
  "moldId":"ABC123",
  "sequence":123,
  "priority":10,
  "data":{
    "ClampPos1":401.28,
    "ClampSpeed1":21.54
  }
}
~~~~~~~~~~~~


ReadMoldDataMessage
-------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.Message`

### Description

The iChen&reg; 4.1 Server keeps a cache of the states of all mold settings 
for each controller. This class implements the READ_MOLD_DATA message which is 
sent to the iChen&reg; 4.1 Server to read the current value of a particular 
mold setting. 

The server responds with a [`MoldDataValueMessage`](#molddatavaluemessage) 
message. 

### Properties

|Property Name       |.NET Type|JSON Property |JSON Type|           Description         |
|:-------------------|:-------:|:------------:|:-------:|:------------------------------|
|`Sequence`          |`Int64`  |`sequence`    |`number` |*Inherited from [`Message`](#message)*     |
|`Priority`          |`Int32`  |`priority`    |`number` |*Inherited from [`Message`](#message)*     |
|`ControllerId`      |`UInt32` |`controllerId`|`number` |Unique ID of the controller    |
|`Field`             |`String` |`field`       |`string` |Name of the mold setting to read.|

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"ReadMoldData",
  "sequence":123,
  "controllerId":789,
  "field":"ClampPos1",
  "priority":10
}
~~~~~~~~~~~~


MoldDataValueMessage
--------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.ReadMoldDataMessage` : `iChen.OpenProtocol.Message`

### Description

This class implements the `RESP_MOLD_DATA_VALUE` message, which is sent from 
the iChen&reg; 4.1 Server to the client in response to a [`ReadMoldDataMessage`](#readmolddatamessage) 
message. 

The iChen&reg; 4.1 Server keeps a cache of the states of all mold settings 
for each controller. The current cached value for the particular mold setting 
requested is sent. 

### Properties

|Property Name       |.NET Type|JSON Property |JSON Type|           Description         |
|:-------------------|:-------:|:------------:|:-------:|:------------------------------|
|`Sequence`          |`Int64`  |`sequence`    |`number` |*Inherited from [`Message`](#message)*     |
|`Priority`          |`Int32`  |`priority`    |`number` |*Inherited from [`Message`](#message)*     |
|`ControllerId`      |`UInt32` |`controllerId`|`number` |*Inherited from [`ReadMoldDataMessage`](#readmolddatamessage)*    |
|`Field`             |`String` |`field`       |`string` |*Inherited from [`ReadMoldDataMessage`](#readmolddatamessage)*    |
|`Value`             |`Double` |`value`       |`number` |Current cached value of the mold setting.|

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"MoldDataValue",
  "sequence":123,
  "controllerId":789,
  "field":"ClampPos1",
  "value":401.28,
  "priority":10
}
~~~~~~~~~~~~


ControllerStatusMessage
-----------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.MoldDataMessage<UInt32, Controller>` : `iChen.OpenProtocol.DataDictionaryMessage<UInt32, Controller>` : `iChen.OpenProtocol.Message`

### Description

This class implements the `UPD_CNTRLER` message, which is sent from the 
iChen&reg; 4.1 Server to the client whenever status of a connected 
controller changes. Only the changed fields will be set, with other 
fields/properties being set to `null` or an appropriate default value as they 
are not relevant. 

### Properties

|Property Name       |.NET Type|JSON Property   |JSON Type|           Description                  |
|:-------------------|:-------:|:--------------:|:-------:|:---------------------------------------|
|`Sequence`          |`Int64`  |`sequence`      |`number` |*Inherited from [`Message`](#message)*              |
|`Priority`          |`Int32`  |`priority`      |`number` |*Inherited from [`Message`](#message)*              |
|`TimeStamp`         |`DateTime`|`timestamp`     |`string` |Date/time (in ISO-8601 format)    |
|`ControllerId`      |`UInt32` |`controllerId`  |`number` |Unique ID of the controller             |
|`DisplayName`       |`String` |`displayName`   |`string` |Human-friendly name for display (or `null` if not relevant)|
|`OpMode`            |[`OpModes`](../../doc/enums.md#opmodes) enum|`opMode`        |`string` |Current operation mode of the controller (or `Unknown`/`null` if not relevant)|
|`JobMode`           |[`JobModes`](../../doc/enums.md#jobmodes) enum|`jobMode`       |`string` |Current job mode of the controller (or `Unkonwn`/`null` if not relevant)|
|`JobCardId`         |`String` |`jobCardId`     |`string` |Unique ID of the current job card loaded, empty string if no mold data set is currently loaded (or `null` if not relevant)|
|`IsDisconnected`    |`Boolean`|`isDisconnected`|`boolean`|If true, the controller has disconnected from the iChen&reg; Server|
|`Alarm`             |`KeyValuePair` `<String,Boolean>` |`alarm`  |`object` |State of an alarm (if any) on the controller (or `null` if not relevant). See [here](../../doc/alarms.md) for valid alarm codes.|
|`Audit`             |`KeyValuePair` `<String,Double>` |`audit`  |`object` |Change of a setting (if any) on the controller for audit trail purpose (or `null` if not relevant)|
|`Variable`          |`KeyValuePair` `<String,Double>` |`variable`|`object` |Change of a variable (if any) on the controller (or `null` if not relevant)|
|`OperatorId`        |`Nullable<UInt32>`|`operatorId`|`number` |Unique ID of the current logged-on user, zero if no user is logged on (or `null` if not relevant)|
|`OperatorName`      |`String` |`operatorName`  |`string` |Name of the current logged-on user (or `null` if not available)|
|`MoldId`            |`String` |`moldId`        |`string` |Unique ID of the current mold data set loaded, empty string if no mold data set is currently loaded (or `null` if not relevant)|
|`DisplayName`       |`String` |`displayName`   |`string` |Human-friendly name for display (or `null` if not relevant)|
|`Controller`        |`Controller`|`controller`    |`object` |A `Controller` object containing complete info of the controller. This field is only sent once by the server as soon as a new controller has connected into the network. All subsequent messages have this field set to `null`.  If this field is not `null`, then all other info fields will be `null`|

### JSON Format Example (without the `Controller` field)

~~~~~~~~~~~~json
{
  "$type":"ControllerStatus",
  "timestamp":"2016-04-01T01:12:23+08:00",
  "controllerId":234,
  "displayName":"M2",
  "opMode":"Manual",
  "jobMode":"Offline",
  "jobCardId":"XYZ",
  "isDisconnected":false,
  "alarm":{ "key":"DOOROPEN", "value":true },
  "audit":{ "key":"PRES", "value":50.0 },
  "variable": { "RT_TempZ1", "value": 231.4 },
  "operatorId":987,
  "operatorName":"Johnny",
  "moldId":"ABC123",
  "sequence":123,
  "priority":10
}
~~~~~~~~~~~~


ControllerActionMessage
-----------------------

> Assembly: `iChen.OpenProtocol.dll`  
> Namespace: `iChen.OpenProtocol`  
> Base class: `iChen.OpenProtocol.Message`

### Description

This class implements the `CNTRLER_ACTION` message, which is sent from the 
iChen&reg; 4.1 Server to the client whenever the current *action* of a 
connected controller changes. 

### Properties

|Property Name       |.NET Type|JSON Property |JSON Type|           Description         |
|:-------------------|:-------:|:------------:|:-------:|:------------------------------|
|`Sequence`          |`Int64`  |`sequence`    |`number` |*Inherited from [`Message`](#message)*     |
|`Priority`          |`Int32`  |`priority`    |`number` |*Inherited from [`Message`](#message)*     |
|`TimeStamp`        |`DateTime`|`timestamp`   |`string` |Date/time (in ISO-8601 format) |
|`ControllerId`      |`UInt32` |`controllerId`|`number` |Unique ID of the controller    |
|`actionId`          |`UInt32` |`actionId`    |`number` |Unique ID of the action ([details...](../../doc/actions.md))|

### JSON Format Example

~~~~~~~~~~~~json
{
  "$type":"ControllerAction",
  "timestamp":"2016-04-01T01:12:23+08:00",
  "actionId":1001,
  "sequence":123,
  "priority":10
}
~~~~~~~~~~~~

