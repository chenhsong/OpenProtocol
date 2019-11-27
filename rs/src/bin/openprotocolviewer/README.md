Open Protocol™ Viewer
=====================

Rust Edition: 2018

This is a simple client program that connects to an iChen® System using Open Protocol™.

Open Protocol™ messages to and from the server are displayed to the standard output.

This program also acts as a user authentication and job cards provider to test out
the operator login and job card features.

WebSocket Client
----------------

For this example, the WebSocket client in the [`websocket`](https://crates.io/crates/websocket)
crate is used to connect to the iChen® server.
In a production environment, other WebSocket implementations may be used instead.
Open Protocol™ does not depend on the particular WebSocket implementation employed.

How to Run
----------

First build the project. This automatically builds all example programs as well.
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

_Warning: If you enter a password to a user account that does not have enough access
rights, you will fail to receive all Open Protocol™ messages. The iChen® Server will
silently discard any message that the password does not have rights to. You will not
even know that messages are missing._
