// Type Definitions

namespace iChen.OpenProtocol
{
	// Enum types
	export type Languages = "EN" | "B5" | "GB" | "FR" | "DE" | "IT" | "ES" | "PT" | "JA" | "Unknown";
	export type ControllerTypes = "Ai01" | "Ai02" | "Ai11" | "Ai12" | "CPC60" | "MPC60" | "CDC2000" | "CDC3000" | "CDC2000WIN" | "SPS3300" | "NewAge" | "CBmold300" | "CBmold800" | "Unknown";
	export type OpModes = "Manual" | "SemiAutomatic" | "Automatic" | "Others" | "Offline" | "Unknown";
	export type JobModes = "ID01" | "ID02" | "ID03" | "ID04" | "ID05" | "ID06" | "ID07" | "ID08" | "ID09" | "ID10" | "ID11" | "ID12" | "ID13" | "ID14" | "ID15" | "Offline";

	export type CommandMessageTypes = "Alive" | "RequestControllersList" | "Join" | "RequestMoldData" | "ReadMoldData" | "JobCardsList" | "OperatorInfo";
	export type ResponseMessages = "ControllersList" | "JoinResponse" | "MoldData" | "MoldDataValue" | "CycleData" | "ControllerAction" | "ControllerStatus" | "RequestJobCardsList" | "LoginOperator";
	export type ValidMessageTypes = CommandMessageTypes | ResponseMessages;

	// Other types
	export type KeyValue<K, V> = { key: K; value: V; }
	export type Expando<T> = { [prop: string]: T; }
	export type Dictionary<T> = { [key: string]: T; };
	export type PropertiesMap = Expando<string>;

	// Controller status
	export interface IControllerStatus
	{
		controllerId: number;
		displayName: string;
		controllerType: ControllerTypes;
		version: string;
		model: string;
		IP: string;
		opMode: OpModes;
		jobMode: JobModes;
		jobCardId?: string;
		lastCycleData?: Dictionary<number>;
		lastConnectionTime?: string;
		moldId?: string;
		operatorId: number;
		operatorName?: string;
	}

	// WebSocket message base
	export interface IMessage
	{
		sequence: number;
		priority?: number;
		$type: ValidMessageTypes;
	}

	// WebSocket messages
	export interface IDataDictionaryMessage<T> extends IMessage
	{
		data: Dictionary<T>;
	}
	export interface IControllerSpecificMessage extends IMessage
	{
		controllerId: number;
	}
	export interface IAliveMessage extends IMessage
	{
		$type: "Alive";
	}
	export interface IRequestControllersListMessage extends IMessage
	{
		$type: "RequestControllersList";
		controllerId?: number;
	}
	export interface IControllersListMessage extends IDataDictionaryMessage<IControllerStatus>
	{
		$type: "ControllersList";
	}
	export interface IJoinMessage extends IMessage
	{
		$type: "Join";
		language: Languages;
		version: string;
		orgId?: string;
		password: string;
		filter?: string;
	}
	export interface IJoinResponseMessage extends IMessage
	{
		$type: "JoinResponse";
		result: number;
		level: number;
		message: string;
	}
	export interface IRequestMoldDataMessage extends IControllerSpecificMessage
	{
		$type: "RequestMoldData";
	}
	export interface IReadMoldDataMessage extends IControllerSpecificMessage
	{
		$type: "ReadMoldData";
		field: string;
	}
	export interface IMoldDataValueMessage extends IControllerSpecificMessage
	{
		$type: "MoldDataValue";
		field: string;
		value: number;
	}
	export interface IDictionaryMessage extends IDataDictionaryMessage<number>, IControllerSpecificMessage
	{
		timestamp: Date;
		jobCardId?: string;
		moldId?: string;
		operatorId: number;
	}
	export interface ICycleDataMessage extends IDictionaryMessage
	{
		$type: "CycleData";
	}
	export interface IMoldDataMessage extends IDictionaryMessage
	{
		$type: "MoldData";
	}
	export interface IControllerActionMessage extends IControllerSpecificMessage
	{
		$type: "ControllerAction";
		timestamp: Date;
		actionId: number;
	}
	export interface IControllerStatusMessage extends IControllerSpecificMessage
	{
		$type: "ControllerStatus";
		timestamp: Date;
		displayName?: string;
		isDisconnected?: boolean;
		opMode?: OpModes;
		jobMode?: JobModes;
		jobCardId?: string | null;
		alarm?: KeyValue<string, boolean>;
		audit?: KeyValue<string, number>;
		variable?: KeyValue<string, number>;
		operatorId?: number;
		operatorName?: string | null;
		moldId?: string | null;
		controller?: IControllerStatus;
	}
	export interface IJobCard
	{
		jobCardId: string;
		moldId: string;
		progress: number;
		total: number;
	}
	export interface IRequestJobCardsListMessage extends IControllerSpecificMessage
	{
		$type: "RequestJobCardsList";
	}
	export interface IJobCardsListMessage extends IDataDictionaryMessage<IJobCard>, IControllerSpecificMessage
	{
		$type: "JobCardsList";
	}
	export interface ILoginOperatorMessage extends IControllerSpecificMessage
	{
		$type: "LoginOperator";
		password: string;
	}
	export interface IOperatorInfoMessage extends IControllerSpecificMessage
	{
		$type: "OperatorInfo";
		operatorId: number;
		name: string;
		password: string;
		level: number;
	}
}
