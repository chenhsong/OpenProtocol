Release 4.2
===========

Enhancements
------------

- Messages can optinally contain a unique `ID` field (randomly
  generated) for tracking purposes. A new method `CreateUniqueID`
  creates this unique ID, over-writing whatever is in the `ID`
  field previously.

- The constructor for `RequestControllersListMessage` can now take
  an optional `ControllerId` parameter which, when set, will limit
  the returned list to only the controller with the specified serial
  number ID.

- A `State` field is added to `ControllerStateMessage` which may
  hold a `StateValues` object containing the state values
  (e.g. op mode, job mode etc.) of the controller at the time
  of the event.

- `JobMode` and `OpMode` fields are added to `CycleDataMessage`.

- As `Message`'s are, mostly, immutable, the `JSON` representation
  of a `Message` is cached for reuse.

Breaking Changes
----------------

- `ControllerType` field in `Controller` is changed to `String` in
  order to accommodate future controller types. 
  
- The `ControllerTypes` `enum` is removed.

- `JSON` representation of `ControllerStateMessage` is refined.


Release 4.1.1
=============

New Features
------------

- Geo-location fields (not yet used)

- `OperatorName` field

Breaking Changes
----------------

- `ControllerStatusMessage` and `Controller` constructors are modified to take an extra `OperatorName` parameter.

- The `Controller` constructor is modified to take extra geo-location fields.

- C# 7.2 or higher is now required.
