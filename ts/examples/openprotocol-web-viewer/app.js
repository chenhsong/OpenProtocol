"use strict";
/// <reference path="../../lib/iChen.OpenProtocol.d.ts" />
document.addEventListener("DOMContentLoaded", function () {
    /// Mock job scheduling system
    var Jobs = [
        { jobCardId: "JOB_CARD_1", moldId: "MOULD_001", progress: 0, total: 8000 },
        { jobCardId: "JOB_CARD_2", moldId: "MOULD_002", progress: 2000, total: 10000 },
        { jobCardId: "JOB_CARD_3", moldId: "MOULD_003", progress: 888, total: 3333 },
        { jobCardId: "JOB_CARD_4", moldId: "MOULD_004", progress: 123, total: 45678 }
    ];
    // DOM elements
    var txtUrl = document.getElementById("txtUrl");
    var txtPwd = document.getElementById("txtPwd");
    var btnConnect = document.getElementById("btnConnect");
    var txtId = document.getElementById("txtId");
    var btnGetMoldData = document.getElementById("btnGetMoldData");
    var txtField = document.getElementById("txtField");
    var btnReadMoldData = document.getElementById("btnReadMoldData");
    var divMessages = document.getElementById("messages");
    // Hook up buttons
    btnConnect.addEventListener("click", run);
    btnGetMoldData.addEventListener("click", function () { return getMoldData(parseInt(txtId.value, 10)); });
    btnReadMoldData.addEventListener("click", function () { return readMoldData(parseInt(txtId.value, 10), txtField.value); });
    // Log text to page
    function logText(type, text) {
        var div = document.createElement("div");
        div.className = type;
        if (type === "message" || type === "command") {
            var code = document.createElement("code");
            code.textContent = text;
            div.appendChild(code);
        }
        else {
            div.textContent = text;
        }
        divMessages.appendChild(div);
    }
    // Global variables
    var sequence = 0;
    var websock;
    var loopHandle = 0;
    // Send a message over WebSocket
    function sendMessage(message, type) {
        if (type === void 0) { type = "command"; }
        var json_text = JSON.stringify(message);
        logText(type, "Sent: " + json_text);
        websock.send(json_text);
    }
    // Main loop
    function run() {
        var url = txtUrl.value.trim(); // WebSocket URL
        var password = txtPwd.value.trim(); // Password
        if (!url) {
            alert('Please enter a valid WebSocket URL in the format "ws://host:port" or "wss://host:port". Example: "ws://192.168.1.1:5788".');
            txtUrl.focus();
            return;
        }
        if (!/^wss?\:\/\/.*\:\d+$/.test(url)) {
            alert('Please enter a valid WebSocket URL in the format "ws://host:port" or "wss://host:port". Example: "ws://192.168.1.1:5788".');
            txtUrl.focus();
            return;
        }
        if (!password) {
            alert("Please enter a valid password.");
            txtPwd.focus();
            return;
        }
        // Disable/enable UI elements
        txtUrl.disabled = txtPwd.disabled = btnConnect.disabled = true;
        txtId.disabled = btnGetMoldData.disabled = false;
        txtField.disabled = btnReadMoldData.disabled = false;
        logText("info", "Connecting to iChen 4.0 Server at " + url + "...");
        // Create a WebSocket connection to the server
        websock = new WebSocket(url);
        // Hook up WebSocket events
        websock.onopen = function () {
            // Initialize handshake with server
            logText("info", "WebSocket connection established.");
            // Send a JOIN message
            sendMessage(iChen.OpenProtocol.createMessage("Join", { version: "1.0", language: "EN", password: password, filter: "All, JobCards, Operators" }));
            // Send an ALIVE message once every 5 seconds
            loopHandle = setInterval(function () { return sendMessage(iChen.OpenProtocol.createMessage("Alive")); }, 5000);
        };
        websock.onerror = function (event) {
            console.error(event);
            logText("error", "An error has occurred!");
        };
        websock.onclose = function (event) {
            logText("info", "WebSocket connection to iChen 4.0 Server is closed.");
            logText("info", "Code = " + event.code + ", Reason = " + event.reason);
            clearInterval(loopHandle);
        };
        websock.onmessage = function (msg) {
            try {
                logText("message", "Received: " + msg.data);
                var reply_message = handleMessage(JSON.parse(msg.data));
                if (reply_message)
                    sendMessage(reply_message, "reply");
            }
            catch (err) {
                console.error(err);
            }
        };
    }
    // Handle message
    function handleMessage(message) {
        switch (message.$type) {
            case "JoinResponse": {
                // Send a REQ_CNTRLER_LIST message
                sendMessage(iChen.OpenProtocol.createMessage("RequestControllersList"));
                return null;
            }
            case "LoginOperator": {
                // MIS integration - return access level
                var operator_message = message;
                // Popup prompt to ask for the access level
                var resp = prompt("User on machine " + operator_message.controllerId + " tries to login with password " + operator_message.password + ". Access level (0-10)?");
                // Pressed cancel --> ignore message
                if (resp === null)
                    return null;
                var access_level = parseInt(resp, 10);
                if (access_level > 10)
                    access_level = 10;
                if (access_level <= 0)
                    return iChen.OpenProtocol.createMessage("OperatorInfo", { controllerId: operator_message.controllerId, operatorId: 0, name: "Disallowed", password: operator_message.password, level: 0 });
                return iChen.OpenProtocol.createMessage("OperatorInfo", { controllerId: operator_message.controllerId, operatorId: access_level * 100, name: "User" + access_level, password: operator_message.password, level: access_level });
            }
            case "RequestJobCardsList": {
                // MIS integration - return job cards list
                var jobcards_message = message;
                return iChen.OpenProtocol.createMessage("JobCardsList", { controllerId: jobcards_message.controllerId, jobCards: Jobs });
            }
            default: return null;
        }
    }
    // Send command to get mold data
    function getMoldData(id) {
        if (id === undefined || id === null || isNaN(id)) {
            alert("Please enter a valid machine number.");
            txtId.focus();
            return;
        }
        // Send a REQ_CNTRLER_LIST message
        sendMessage(iChen.OpenProtocol.createMessage("RequestMoldData", { controllerId: id }));
    }
    // Send command to read mold data value
    function readMoldData(id, field) {
        if (id === undefined || id === null || isNaN(id)) {
            alert("Please enter a valid machine number.");
            txtId.focus();
            return;
        }
        // Send a READ_MOLD_DATA message
        sendMessage(iChen.OpenProtocol.createMessage("ReadMoldData", { controllerId: id, field: field }));
    }
});
//# sourceMappingURL=app.js.map