Open Protocol™ Viewer
=====================

Rust Edition: 2018

This is a simple client program that connects to an iChen® System using Open Protocol™.

Open Protocol™ messages to and from the server are displayed to the standard output.

This program also acts as a user authentication and job cards provider to test out
the operator login and job card features.

How to Use
----------

Build the project. This will automatically build all example programs as well.
The program executable will be under the `target/debug` or `target/release` directory.

Run the executable (e.g. `openprotocolviewer.exe` on Windows) and enter the following
information:

**`WebSocket URL`** : URL of the Open Protocol™ interface,
usually `ws://MyiChenServerUrl:5788` or `ws://x.x.x.x:5788`
(5788 is the default Open Protocol™ interface port).
`wss:` access to secured WebSocket ports with HTTPS is _not_ supported in this sample.

**`Password`** : A login password to connect to the system.
System default is `chenhsong` for the `admin` user with unlimited admin rights
(other than MIS/MES rights).
To try out the MIS/MES features (e.g. operator login, job cards), first set up a new
user account with the appropriate rights, then login with that password.
Otherwise, the user authentication and job cards provider will not work.

_Warning: If you do not enter a password of a user account that has the appropriate
access rights, you'll fail to see all Open Protocol™ messages._
