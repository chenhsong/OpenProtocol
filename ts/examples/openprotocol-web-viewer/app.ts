/// <reference path="../../lib/iChen.OpenProtocol.d.ts" />

type LogTypes = "command" | "message" | "info" | "error" | "warning" | "reply";

document.addEventListener("DOMContentLoaded", () =>
{
	/// Mock job scheduling system
	const Jobs: iChen.OpenProtocol.IJobCard[] = [
		{ jobCardId: "JOB_CARD_1", moldId: "MOULD_001", progress: 0, total: 8000 },
		{ jobCardId: "JOB_CARD_2", moldId: "MOULD_002", progress: 2000, total: 10000 },
		{ jobCardId: "JOB_CARD_3", moldId: "MOULD_003", progress: 888, total: 3333 },
		{ jobCardId: "JOB_CARD_4", moldId: "MOULD_004", progress: 123, total: 45678 }
	];

	// DOM elements
	const txtUrl = document.getElementById("txtUrl") as HTMLInputElement;
	const txtPwd = document.getElementById("txtPwd") as HTMLInputElement;
	const btnConnect = document.getElementById("btnConnect") as HTMLButtonElement;
	const txtId = document.getElementById("txtId") as HTMLInputElement;
	const btnGetMoldData = document.getElementById("btnGetMoldData") as HTMLButtonElement;
	const txtField = document.getElementById("txtField") as HTMLInputElement;
	const btnReadMoldData = document.getElementById("btnReadMoldData") as HTMLButtonElement;
	const divMessages = document.getElementById("messages") as HTMLDivElement;

	// Hook up buttons
	btnConnect.addEventListener("click", run);
	btnGetMoldData.addEventListener("click", () => getMoldData(parseInt(txtId.value, 10)));
	btnReadMoldData.addEventListener("click", () => readMoldData(parseInt(txtId.value, 10), txtField.value));

	// Log text to page
	function logText(type: LogTypes, text: string)
	{
		const div = document.createElement("div");
		div.className = type;

		if (type === "message" || type === "command") {
			const code = document.createElement("code");
			code.textContent = text;
			div.appendChild(code);
		} else {
			div.textContent = text;
		}

		divMessages.appendChild(div);
	}

	// Global variables
	let sequence = 0;
	let websock: WebSocket;
	let loopHandle = 0;

	// Send a message over WebSocket
	function sendMessage(message: iChen.OpenProtocol.IMessage, type: LogTypes = "command")
	{
		const json_text = JSON.stringify(message);
		logText(type, `Sent: ${json_text}`);
		websock.send(json_text);
	}

	// Main loop
	function run()
	{
		const url = txtUrl.value.trim();		// WebSocket URL
		const password = txtPwd.value.trim();		// Password

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

		logText("info", `Connecting to iChen 4.0 Server at ${url}...`);

		// Create a WebSocket connection to the server
		websock = new WebSocket(url);

		// Hook up WebSocket events
		websock.onopen = () =>
		{
			// Initialize handshake with server
			logText("info", "WebSocket connection established.");

			// Send a JOIN message
			sendMessage(iChen.OpenProtocol.createMessage("Join", { version: "1.0", language: "EN", password, filter: "All, JobCards, Operators" }));

			// Send an ALIVE message once every 5 seconds
			loopHandle = setInterval(() => sendMessage(iChen.OpenProtocol.createMessage("Alive")), 5000);
		};
		websock.onerror = event =>
		{
			console.error(event);
			logText("error", "An error has occurred!");
		}
		websock.onclose = event =>
		{
			logText("info", "WebSocket connection to iChen 4.0 Server is closed.");
			logText("info", `Code = ${event.code}, Reason = ${event.reason}`);
			clearInterval(loopHandle);
		}
		websock.onmessage = msg =>
		{
			try {
				logText("message", `Received: ${msg.data}`);

				const reply_message = handleMessage(JSON.parse(msg.data));

				if (reply_message) sendMessage(reply_message, "reply");
			} catch (err) {
				console.error(err);
			}
		}
	}

	// Handle message
	function handleMessage(message: iChen.OpenProtocol.IMessage): iChen.OpenProtocol.IMessage | null
	{
		switch (message.$type) {
			case "JoinResponse": {
				// Send a REQ_CNTRLER_LIST message
				sendMessage(iChen.OpenProtocol.createMessage("RequestControllersList"));
				return null;
			}

			case "LoginOperator": {
				// MIS integration - return access level
				const operator_message = message as iChen.OpenProtocol.ILoginOperatorMessage;

				// Popup prompt to ask for the access level
				const resp = prompt(`User on machine ${operator_message.controllerId} tries to login with password ${operator_message.password}. Access level (0-10)?`);

				// Pressed cancel --> ignore message
				if (resp === null) return null;

				let access_level = parseInt(resp, 10);
				if (access_level > 10) access_level = 10;
				if (access_level <= 0) return iChen.OpenProtocol.createMessage("OperatorInfo", { controllerId: operator_message.controllerId, operatorId: 0, name: "Disallowed", level: 0 });
				return iChen.OpenProtocol.createMessage("OperatorInfo", { controllerId: operator_message.controllerId, operatorId: access_level * 100, name: "User" + access_level, password: operator_message.password, level: access_level });
			}

			case "RequestJobCardsList": {
				// MIS integration - return job cards list
				const jobcards_message = message as iChen.OpenProtocol.IRequestJobCardsListMessage;
				return iChen.OpenProtocol.createMessage("JobCardsList", { controllerId: jobcards_message.controllerId, jobCards: Jobs });
			}

			default: return null;
		}
	}

	// Send command to get mold data
	function getMoldData(id: number)
	{
		if (id === undefined || id === null || isNaN(id)) {
			alert("Please enter a valid machine number.");
			txtId.focus();
			return;
		}

		// Send a REQ_CNTRLER_LIST message
		sendMessage(iChen.OpenProtocol.createMessage("RequestMoldData", { controllerId: id }));
	}

	// Send command to read mold data value
	function readMoldData(id: number, field: string)
	{
		if (id === undefined || id === null || isNaN(id)) {
			alert("Please enter a valid machine number.");
			txtId.focus();
			return;
		}

		// Send a READ_MOLD_DATA message
		sendMessage(iChen.OpenProtocol.createMessage("ReadMoldData", { controllerId: id, field }));
	}
});