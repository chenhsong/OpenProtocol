# iChen&reg; 4.1 Open Protocol&trade; Library Reference - Alarm Codes

Copyright &copy; 2016 Chen Hsong Holdings Ltd.  All rights reserved.

## Ai-series Controllers

|Alarm Code|English Alarm Name|Chinese Name|Description|
|----------|------------------|------------|-----------|
|`AL001`|Alarm 2|警报2|
|`AL002`|Main Cylinder Not Aligned|大油缸未定位|
|`AL003`|Barrel Temperature Not Reached|料管温度未达设定|Actual barrel temperature is lower than the minimum set-point temperature.|
|`AL004`|Lubrication Oil Level Low|润滑油油量不足|The level of lubrication oil is too low.|
|`AL005`|Lubrication Pressure Low|润滑油压力过低|Lubrication pressure is too low, leakage or damage possible.|
|`AL006`|Pump Motor Overload|油泵过载|Oil pump motor overload.|
|`AL007`|Mould Adjustment Motor Overload|调模马达过载|Mould-adjustment motor overload.|
|`AL008`|Rear Safety Door Open|后安全门警报|Rear safety door open; also check limit switch.|
|`AL009`|Front Safety Door Open|前安全门警报|Front safety door open; also check limit switch.|
|`AL010`|Mould Adjustment Below Limit|超出调模最小限位|Mould thickness less than the minimum allowed; check limit switch.|
|`AL011`|Mould Adjustment Above Limit|超出调模最大限位|Mould thickness exceeds the maximum allowed; check limit switch.|
|`AL012`|Safety Door Limit Switch Error|安全门限错误|
|`AL013`|Safety Door Latch Error|安全门锁错误|
|`AL014`|Grease Pressure Low|油脂润滑压力不足|
|`AL015`|No Cooling Water|冷却水未开|
|`AL016`|Bad Part|不良品|
|`AL017`|Bad Parts Maximum Reached|不良品模数已达设定|
|`AL018`|mergency Stopped|紧急停止 |
|`AL019`|Nozzle Forward Limit Switch Error|射台前限位故障|Limit switch for carriage forward has not been triggered during automatic operation.|
|`AL020`|Nozzle Guard Open|射咀保护罩未关闭|The purge guard is not closed during injection.|
|`AL021`|Blocked Nozzle|射咀阻塞|The nozzle is blocked; check injection settings or the nozzle.|
|`AL022`|Short-Shot/Over-Shot|短射或过射|Injection end position beyond tolerance; adjust tolerance settings or inspect check ring.|
|`AL023`|Out of Material|料斗无料或阻塞|During automatic operation, plasticization time exceeds cooling time; also check for hopper blockage.|
|`AL024`|Production Completed|成型模数已达设定|Actual production counter has reached the maximum setting under automatic operation.|
|`AL025`|Cycle Too Long|周期时间过长|Cycle time exceeds the maximum tolerance.|
|`AL026`|Mould Protection Alarm|低压护模警报|There are foreign matters inside the Mould, or incorect high-pressure position/low-pressure time settings.|
|`AL027`|Robot Error|请检查机械手|Robot has not returned to the set position during mould opening or clamping.|
|`AL028`|Take Out Error|制品确认讯号异常|Product photocell sensor is on, but no product is detected.|
|`AL029`|Product Sensor Error|电眼故障|Check product photocell sensor and clean product chute.|
|`AL030`|Oil Temp Low|油温过低|Actual hydraulic oil temperature is lower than the allowed minimum.|
|`AL031`|Oil Temp High|油温过高|Actual hydraulic oil temperature is higher than the allowed maximum.|
|`AL032`|Core-Pull Alarm|进芯限位故障|During automatic operation, core-pull time exceeds limit.|
|`AL033`|Ejector Alarm|顶针限位故障|During automatic operation, ejection time exceeds limit.|
|`AL034`|Check Safety Valve for Door|检查门安全位|
|`AL035`|Accumulator Charge Alarm|氮气充压故障|When accumulator is engaged, charging time exceeds cooling time; check charging pressure switch.|
|`AL036`|Mould Adjustment Sensor Error|调模计数开关故障|Mould adjustment sensor is faulty; check Mould-adjustment mechanisms.|
|`AL037`|Low Air Pressure for Robot|机械手气压过低|
|`AL038`|Barrel Pre-heat|料管温度预热中|Pre-heat function turn ON.|
|`AL039`|Unscrew Alarm|检查绞牙计数开关|During automatic operation, unscrew time exceeds limit.|
|`AL040`|Auto Mould-Height Adjustment|自动调模进行中|
|`AL041`|Auto Clamping Force Adjustment|自动锁模力进行中|
|`AL042`|Auto Clamping Force Adjustment Completed|自动调模力完成|
|`AL043`|Barrel Temperature Too High|熔胶筒温度过高|Actual barrel temperature is higher than the maximum set-point temperature.|
|`AL044`|_Not Used_||
|`AL045`|Safety Door Limit Switch Error|门限开关故障|No signal detected on door limit switch.|
|`AL046`|Clamp Open/Close Error|开锁/模故障|
|`AL047`|Product Eject Error|产品顶出故障|
|`AL048`|Clogged Oil Filter|虑油器阻塞|Check and clean oil filter.|
|`AL049`|Robot Alarm|机械手故障|Check robot.|
|`AL050`|Pump Motor Not Started|油泵未启动|Check all voltage phase connections, fuses and breakers.|
|`AL051`|Mould Adjustment Error|调模时间过长|
|`AL052`|Safety Relay Not Yet Reset|安全继电器未复位|
|`AL053`|_Not Used_||
|`AL054`|Clogged Oil Screen|油过滤网阻塞|Oil screen is clogged when using high pressure oil filter.|
|`AL055`|Auto Mould Change|自动换模中|
|`AL056`|Lock-Nut Not Closed|螺母未合|
|`AL057`|Lock-Nut Limit Switch Error|检查转盘限位|
|`AL058`|Clamp Open Pressure Release Error|开模泄压故障|
|`AL059`|High Pressure Cylinder Mis-Aligned|大油缸超行程|
|`AL060`|_Not Used_||
|`AL061`|Oil Level Low|液压油位过低|Check hydraulic oil volume.|
|`AL062`|Mould Adjustment Gear Error|调模齿轮异常|
|`AL063`|Mould Fitting Position Check|模具安装位置检查|
|`AL064`|Hydraulic Clamp Error|油压夹模故障|
|`AL065`|Clamping Force Too Low|锁模力不足|
|`AL066`|Back Pressure Too High|背压过高|
|`AL067`|Material Change|换料中|
|`AL068`|AMC Table Limit Error|换模台限位器故障|
|`AL069`|Oil Filter Error|油泵虑油器异常|
|`AL070`|Plasticizing RPM Sensor Error|熔胶转速开关故障|
|`AL071`|Control Cabinet Door Open|控制箱门未关|
|`AL072`|Out-of-Battery|请换电池|
|`AL073`|Auto Mould-Height Adjustment Completed|自动调模厚度完成|
|`AL074`|Injection Settings Error|射胶设定错误|
|`AL075`|Pressure Transducer Error|压力感应器故障|
|`AL076`|Turn-Table Rotating|转盘中|
|`AL077`|Stopper Not Returned|定位销未回|
|`AL078`|Auto Mould Adjustment Error|自动调模错误|
|`AL079`|Safety Platform Error|安全踏板故障|
|`AL080`|_Not Used_||
|`AL081`|Ejector Plate Not Returned|顶针板未回|
|`AL082`|Safety Valve Error|液压安全阀故障|
|`AL083`|Semi/Auto Mode Only|半全自动中，不可手动！|
|`AL084`|Door Latch Error|门锁故障|
|`AL085`|Air Pressure Low|气压不足|
|`AL086`|_Not Used_||
|`AL087`|_Not Used_||
|`AL088`|Product Drop Not Detected|成品未落|
|`AL089`|_Not Used_||
|`AL090`|Robot Safety Check Error|机械手安全位检查故障|
|`AL091`|Robot Not Returned|机械手未归零|
|`AL092`|Servo Control Alarm|伺服控制出错|
|`AL093`|Clamp Open End Position Error|开模终止位置故障|
|`AL094`|Clamping Not Complete|锁模未终止|
|`AL095`|Plasticization Not Complete|熔胶未终止|
|`AL096`|Barrel Purging|清胶中|
|`AL097`|Machine Adjustment|机器调整中|
|`AL098`|Locking Not Complete|入闸未终止|
|`AL099`|Resin Temperature Low|胶料温度过低|
