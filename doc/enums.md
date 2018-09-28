# iChen&reg; 4.1 Open Protocol&trade; Library Reference - Enum Types

Copyright &copy; 2016 Chen Hsong Holdings Ltd.  All rights reserved.

## Enum: Languages

`Languages` represent the language for textual information sent between the
iChen&reg; 4.1 server and clients.

|Code|Numeric Value|Description|
|----|:-----------:|-----------|
|`Unknown`|0|Unknown|
|`EN`|1|English|
|`B5`|2|Traditional Chinese (Big-5)|
|`GB`|3|Simplified Chinese (GB2312)|
|`FR`|4|French|
|`DE`|5|German|
|`IT`|6|Italian|
|`ES`|7|Spanish|
|`PT`|8|Portuguese|
|`JA`|9|Japanese|

## Enum: Filters

`Filters` controls what type(s) of messages the iChen&reg; 4.1 server will send to each client.

### Regular messages

|Code|Description|
|----|-----------|
|`Status`|Controller status|
|`Cycle`|Cycle data|
|`Mold`|Mold settings data|
|`Actions`|Current action|
|`Alarms`|Controller alarms|
|`Audit`|Audit trail of setting changes|
|`All`|All messages above (implies administrator)|

### MIS integration messages

|Code|Description|
|----|-----------|
|`JobCards`|Job cards-related|
|`Operators`|Operator-related|

### Industrial automation

|Code|Description|
|----|-----------|
|`OPCUA`|OPC UA communications|

## Enum: ControllerTypes

|Code|Numeric Value|Description|
|----|:-----------:|-----------|
|`Unknown`|99|Unknown|
|`Ai01`|1|Ai-01 <i>(deprecated)</i>|
|`Ai02`|6|Ai-02|
|`Ai11`|2|Ai-11 <i>(deprecated)</i>|
|`Ai12`|7|Ai-12|
|`CPC6`|8|CPC-6.0|
|`MPC6`|9|MPC-6.0|
|`CDC2000`|98|CDC2000 <i>(deprecated)</i>|
|`CDC2000WIN`|3|CDC2000WIN|
|`CDC3000`|4|CDC3000 <i>(deprecated)</i>|
|`SPS3300`|0|R3000/SPS3300 <i>(deprecated)</i>|
|`NewAge`|5|Beckhoff New-Age <i>(deprecated)</i>|
|`CBmold300`|10|Beckhoff CBmold<sup>300</sup>|
|`CBmold800`|11|Beckhoff CBmold<sup>800</sup>|
|`MPC7`|12|MPC-7|

## Enum: OpModes

`OpModes` represent the operating modes of a machine.

|Code|Numeric Value|Description|
|----|:-----------:|-----------|
|`Unknown`|0|Unknown|
|`Manual`|1|Manual mode|
|`SemiAutomatic`|2|Semi-Automatic|
|`Automatic`|3|Automatic|
|`Others`|4|Others|
|`Offline`|99|Controller is off-line|

## Enum: JobModes

`JobModes` are used-defined codes for internal categorization purposes.
The modes #1 to #15 can be freely defined by the user.

Some controllers (e.g. the Ai-series) do not have user-definable job modes,
but have fixed job mode categories.

|Code|Numeric Value|Description|Ai/CPC/MPC-series hard-coded|
|----|:-----------:|-----------|----------------------------|
|`Unknown`|0|Unknown|
|`ID01`|1|Job mode #1|Idle|
|`ID02`|2|Job mode #2|Production|
|`ID03`|3|Job mode #3|Color Change|
|`ID04`|4|Job mode #4|Mold Maintenance|
|`ID05`|5|Job mode #5|Mold Trial|
|`ID06`|6|Job mode #6|Mold Change|
|`ID07`|7|Job mode #7|Resin Change|
|`ID08`|8|Job mode #8|Waiting for Resin|
|`ID09`|9|Job mode #9|Waiting for Mold|
|`ID10`|10|Job mode #10|Downtime|
|`ID11`|11|Job mode #11|Stoppage|
|`ID12`|12|Job mode #12|Fine-Tuning|
|`ID13`|13|Job mode #13|Other 1|
|`ID14`|14|Job mode #14|Other 2|
|`ID15`|15|Job mode #15|Other 3|
|`Offline`|99|Controller is off-line|
