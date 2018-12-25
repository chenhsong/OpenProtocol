namespace iChen.OpenProtocol
{
	// Auto-increment sequence number
	let seq = 0;

	// Allow iChen.OpenProtocol.getNextSequenceNumber to be overridden
	export function getNextSequenceNumber() { return ++seq; }

	// Main createMessage method overloads
	export function createMessage(type: "Alive", priority?: number): IAliveMessage;
	export function createMessage(type: "Join", params: { language: Languages; version: string; orgId?: string; password: string; filter?: string; }, priority?: number): IJoinMessage;
	export function createMessage(type: "RequestControllersList", priority?: number): IRequestControllersListMessage;
	export function createMessage(type: "RequestControllersList", params: { controllerId: number; }, priority?: number): IRequestControllersListMessage;
	export function createMessage(type: "RequestMoldData", params: { controllerId: number; }, priority?: number): IRequestMoldDataMessage;
	export function createMessage(type: "ReadMoldData", params: { controllerId: number; field: string; }, priority?: number): IReadMoldDataMessage;
	export function createMessage(type: "JobCardsList", params: { controllerId: number; jobCards: IJobCard[]; }, priority?: number): IJobCardsListMessage;
	export function createMessage(type: "OperatorInfo", params: { controllerId: number; operatorId: number; password: string; name: string; level: number; }, priority?: number): IOperatorInfoMessage;
	// Main createMessage method overload template
	export function createMessage(type: CommandMessageTypes, params?: Expando<any> | number, priority?: number): IMessage
	{
		// Create message object
		const msg: IMessage = { $type: type, sequence: iChen.OpenProtocol.getNextSequenceNumber(), priority: priority || 0 };

		// Some messages require no parameter
		if (params === undefined || typeof params === "number") {
			switch (type) {
				case "Alive": {
					// Default priority value for Alive messages = -10
					msg.priority = (params !== undefined) ? params : -10;
					break;
				}
				case "RequestControllersList": break;
				default: throw `Parameters missing for ${type} message.`;
			}

			return msg;
		}

		// Other messages require a parameter
		if (typeof params !== "object") throw `Invalid parameters for ${type} message.`;

		switch (type) {
			case "Alive": throw `No parameters allowed for ${type} message.`;

			case "Join": {
				const jmsg = msg as IJoinMessage;
				jmsg.language = params.language;
				jmsg.version = params.version;
				if (params.orgId) jmsg.orgId = params.orgId;
				jmsg.password = params.password;
				jmsg.filter = params.filter;
				break;
			}

			case "RequestControllersList": {
				const rmsg = msg as IRequestControllersListMessage;
				rmsg.controllerId = params.controllerId;
				break;
			}

			case "RequestMoldData": {
				const rmsg = msg as IRequestMoldDataMessage;
				rmsg.controllerId = params.controllerId;
				break;
			}

			case "ReadMoldData": {
				const rdmsg = msg as IReadMoldDataMessage;
				rdmsg.controllerId = params.controllerId;
				rdmsg.field = params.field;
				break;
			}

			case "JobCardsList": {
				const jcmsg = msg as IJobCardsListMessage;
				const list = params.jobCards as IJobCard[];
				jcmsg.controllerId = params.controllerId;
				jcmsg.data = {};
				for (const jc of list) jcmsg.data[jc.jobCardId] = jc;
				break;
			}

			case "OperatorInfo": {
				const omsg = msg as IOperatorInfoMessage;
				omsg.controllerId = params.controllerId;
				omsg.operatorId = params.operatorId;
				omsg.name = params.name;
				omsg.password = params.password;
				omsg.level = params.level;
				break;
			}
		}

		return msg;
	}
}