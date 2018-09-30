# Open Protocol&trade; Web Viewer (TypeScript)

Language Version: 2.5 or above

This is a simple web application written in TyepScript that connects to an iChen&reg; System using Open Protocol&trade;.

Open Protocol&trade; messages to and from the server are logged on-screen.

This application also acts as a user authentication and job cards provider to test out the operator login and job card features.

## How to Use

### 1. Enter Credentials

Launch `index.html` in a web browser and enter the following information:

**`URL`** : URL of the Open Protocol&trade; interface, usually `ws://MyiChenServerUrl:5788` or `ws://x.x.x.x:5788` (5788 is the default Open Protocol&trade; interface port).

**`Password`** : A login password to connect to the system.  System default is `chenhsong` for the `admin` user with unlimited admin rights (other than MIS rights).
To try out the MIS features (e.g. operator login, job cards), first set up a new user account with the appropriate rights, then login with that password.
Otherwise, the user authentication and job cards provider will not work.

_Warning: If you do not enter a password of a user account that has the appropriate access rights, you'll fail to see all Open Protocol&trade; messages._

### 2. Connect

Next, press the `Connect to Server` button to connect to the iChen&reg; System server.

### 3. Other Features

**`Get Mold Data`** reads the current state of the recipe (mold data set) on the controller specified.

**`Read Mold Value`** reads the current value of a particular variable on the controller specified.
