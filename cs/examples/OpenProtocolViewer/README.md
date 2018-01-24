# OpenProtocol&trade; Viewer (C# Console)

.NET Framework Version: 4.6.1 or up

This is a simple console application written in C# that connects to an iChen&reg; System using OpenProtocol&trade;.

OpenProtocol&trade; messages to and from the server are logged on-screen.

This application also acts as a user authentication and job cards provider to test out the operator login and job card features.

## How to Use

Launch `OpenProtocolViewer.exe` in and enter the following information:

**`WebSocket URL`** : URL of the OpenProtocol&trade; interface, usually `ws://MyiChenServerUrl:5788` or `ws://x.x.x.x:5788` (5788 is the default OpenProtocol&trade; interface port).

**`Password`** : A login password to connect to the system.  System default is `chenhsong` for the `admin` user with unlimited admin rights (other than MIS rights).
To try out the MIS features (e.g. operator login, job cards), first set up a new user account with the appropriate rights, then login with that password.
Otherwise, the user authentication and job cards provider will not work.

_Warning: If you do not enter a password of a user account that has the appropriate access rights, you'll fail to see all OpenProtocol&trade; messages._
