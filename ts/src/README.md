# OpenProtocol&trade; Access Library (TypeScript)

Language Version: 2.5 or above

## Location

The library module is located in `../lib/iChen.OpenProtocol.js` or `../lib/iChen.OpenProtocol.min.js`

## Typings

A typings file can be found in `../lib/iChen.OpenProtocol.d.ts`

## How to Use in a TypeScript Project

### Typings

* Copy the typings file `iChen.OpenProtocol.d.ts` directly into the root directory of the project, or

* Use `/// <reference path="path/to/iChen.OpenProtocol.d.ts" />` as the first line in a TypeScript source file to refer to the typings file.

### Library

Load the library module in the HTML page:

~~~html
<script src="path/to/iChen.OpenProtocol.js"></script>
~~~

or (if minified version)

~~~html
<script src="path/to/iChen.OpenProtocol.min.js"></script>
~~~

### API Usage

An global object called `iChen.OpenProtocol` will be available for you to create various OpenProtocol&trade; messages.
In particular, the `iChen.OpenProtocol.createMessage` function can create different messages based on parameters provided.

To converse with an iChen&reg; System server, simply:

1. Create a WebSocket connection to the server
2. Create a `Join` message
3. Login to the server by sending the `Join` message
4. Start listening to reply messages and respond

See the various example programs for details.
