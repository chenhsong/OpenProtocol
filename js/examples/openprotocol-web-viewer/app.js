document.addEventListener("DOMContentLoaded", function ()
{
	/// Mock job scheduling system
	var Jobs = [
		{ jobCardId: "JOB_CARD_1", moldId: "MOULD_001", progress: 0, total: 8000 },
		{ jobCardId: "JOB_CARD_2", moldId: "MOULD_002", progress: 2000, total: 10000 },
		{ jobCardId: "JOB_CARD_3", moldId: "MOULD_003", progress: 888, total: 3333 },
		{ jobCardId: "JOB_CARD_4", moldId: "MOULD_004", progress: 123, total: 45678 }
	];

	// DOM Elements
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
	function logText(type, text)
	{
		var div = document.createElement("div");
		div.className = type;

		if (type === "message" || type === "command") {
			var code = document.createElement("code");
			code.textContent = text;
			div.appendChild(code);
		} else {
			div.textContent = text;
		}

		divMessages.appendChild(div);
	}

	// Global variables
	var seq = 0;
	var ws;
	var handle = 0;

	// Send a message via WebSocket
	function sendMessage(msg, type)
	{
		if (!type) type = "command";
		var json = JSON.stringify(msg);
		logText(type, "Sent: " + json);
		ws.send(json);
	}

	// Main loop
	function run()
	{
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

		// Enable/disable UI elements
		txtUrl.disabled = txtPwd.disabled = btnConnect.disabled = true;
		txtId.disabled = btnGetMoldData.disabled = false;
		txtField.disabled = btnReadMoldData.disabled = false;

		logText("info", "Connecting to iChen 4.0 Server at " + url + "...");

		// Create a WebSocket connection to the server
		ws = new WebSocket(url);

		ws.onopen = function ()
		{
			// Initialize handshake with server
			logText("info", "WebSocket connection established.");

			// Send a JOIN message
			sendMessage({ $type: "Join", language: "EN", version: "1.0", password: password, filter: "All, JobCards, Operators", sequence: ++seq });

			loopHandle = setInterval(function () { sendMessage({ $type: "Alive", sequence: ++seq, priority: -10 }); }, 5000);
		};

		// Wire up WebSocket events
		ws.onerror = function (ev)
		{
			console.error(ev);
			logText("error", "An error has occurred!");
		};
		ws.onclose = function (ev)
		{
			logText("info", "WebSocket connection to iChen 4.0 Server is closed.");
			logText("info", "Code = " + ev.code + ", Reason = " + ev.reason);
			clearInterval(loopHandle);
		};
		ws.onmessage = function (msg)
		{
			try {
				logText("message", "Received: " + msg.data);
				var reply_message = handleMessage(JSON.parse(msg.data));
				if (reply_message) sendMessage(reply_message, "reply");
			} catch (err) { console.error(err); }
		};
	}

	// Handle message
	function handleMessage(message)
	{
		switch (message.$type) {
			case "JoinResponse": {
				// Send a REQ_CNTRLER_LIST message
				sendMessage({ $type: "RequestControllersList", sequence: ++seq });
				return null;
			}

			case "LoginOperator": {
				// MIS integration - return access level
				// Popup prompt to ask for the access level
				var resp = prompt("User on machine " + message.controllerId + " tries to login with password " + message.password + ". Access level (0-10)?");

				// Pressed cancel --> ignore message
				if (resp === null) return null;

				var access_level = parseInt(resp, 10);
				if (access_level > 10) access_level = 10;
				if (access_level <= 0) return { $type: "OperatorInfo", controllerId: message.controllerId, operatorId: 0, name: "Disallowed", level: 0 };
				return { $type: "OperatorInfo", controllerId: message.controllerId, operatorId: access_level * 100, name: "User" + access_level, password: message.password, level: access_level, sequence: ++seq };
			}

			case "RequestJobCardsList": {
				// MIS integration - return job cards list
				var data = {};
				for (var x = 0; x < Jobs.length; x++) data[Jobs[x].jobCardId] = Jobs[x];
				return { $type: "JobCardsList", controllerId: message.controllerId, data: data, sequence: ++seq };
			}

			default: return null;
		}
	}

	// Send command to get mold data
	function getMoldData(id)
	{
		if (id === undefined || id === null || isNaN(id)) {
			alert("Please enter a valid machine number.");
			txtId.focus();
			return;
		}

		// Send a REQ_CNTRLER_LIST message
		sendMessage({ $type: "RequestMoldData", controllerId: id, sequence: ++seq });
	}

	// Send command to read mold data value
	function readMoldData(id, field)
	{
		if (id === undefined || id === null || isNaN(id)) {
			alert("Please enter a valid machine number.");
			txtId.focus();
			return;
		}
		// Send a READ_MOLD_DATA message
		sendMessage({ $type: "ReadMoldData", controllerId: id, field: field, sequence: ++seq });
	}
});