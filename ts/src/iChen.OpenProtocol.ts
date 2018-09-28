/// <reference path="../lib/iChen.OpenProtocol.d.ts" />

namespace iChen.OpenProtocol
{
	let seq = 0;

	function throwParamError(msg: string) { throw `Invalid parameters for ${msg}`; }

	export function getNextSequenceNumber() { return ++seq; }

	export function createMessage(type: "Alive", priority?: number): IAliveMessage;
	export function createMessage(type: "Join", params: { language: Languages; version: string; orgId?: string; password: string; filter?: string; }, priority?: number): IJoinMessage;
	export function createMessage(type: "RequestControllersList", priority?: number): IRequestControllersListMessage;
	export function createMessage(type: "RequestMoldData", params: { controllerId: number; }, priority?: number): IRequestMoldDataMessage;
	export function createMessage(type: "ReadMoldData", params: { controllerId: number; field: string; }, priority?: number): IReadMoldDataMessage;
	export function createMessage(type: "JobCardsList", params: { controllerId: number; jobCards: IJobCard[]; }, priority?: number): IJobCardsListMessage;
	export function createMessage(type: "OperatorInfo", params: { controllerId: number; operatorId: number; password: string; name: string; level: number; }, priority?: number): IOperatorInfoMessage;
	export function createMessage(type: CommandMessageTypes, params?: Expando<any> | number, priority?: number): IMessage
	{
		const msg = {
			$type: type,
			sequence: iChen.OpenProtocol.getNextSequenceNumber(),	// Allow iChen.OpenProtocol.getNextSequenceNumber to be overridden
			priority: priority || 0
		} as IMessage;

		switch (type) {
			case "Alive": {
				msg.priority = (typeof params === "number") ? params : -10;		// Default value = -10
				break;
			}

			case "Join": {
				if (!params || typeof params !== "object") throw throwParamError(type);

				const jmsg = msg as IJoinMessage;
				jmsg.language = params.language;
				jmsg.version = params.version;
				if (params.orgId) jmsg.orgId = params.orgId;
				jmsg.password = params.password;
				jmsg.filter = params.filter;
				break;
			}

			case "RequestControllersList":
				break;

			case "RequestMoldData": {
				if (!params || typeof params !== "object") throw throwParamError(type);

				const rmsg = msg as IRequestMoldDataMessage;
				rmsg.controllerId = params.controllerId;
				break;
			}

			case "ReadMoldData": {
				if (!params || typeof params !== "object") throw throwParamError(type);

				const rdmsg = msg as IReadMoldDataMessage;
				rdmsg.controllerId = params.controllerId;
				rdmsg.field = params.field;
				break;
			}

			case "JobCardsList": {
				if (!params || typeof params !== "object") throw throwParamError(type);

				const jcmsg = msg as IJobCardsListMessage;
				const list = params.jobCards as IJobCard[];
				jcmsg.controllerId = params.controllerId;
				jcmsg.data = {};
				for (const jc of list) jcmsg.data[jc.jobCardId] = jc;
				break;
			}

			case "OperatorInfo": {
				if (!params || typeof params !== "object") throw throwParamError(type);

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