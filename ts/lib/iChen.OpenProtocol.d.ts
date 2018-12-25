declare namespace iChen.OpenProtocol {
    function getNextSequenceNumber(): number;
    function createMessage(type: "Alive", priority?: number): IAliveMessage;
    function createMessage(type: "Join", params: {
        language: Languages;
        version: string;
        orgId?: string;
        password: string;
        filter?: string;
    }, priority?: number): IJoinMessage;
    function createMessage(type: "RequestControllersList", priority?: number): IRequestControllersListMessage;
    function createMessage(type: "RequestControllersList", params: {
        controllerId: number;
    }, priority?: number): IRequestControllersListMessage;
    function createMessage(type: "RequestMoldData", params: {
        controllerId: number;
    }, priority?: number): IRequestMoldDataMessage;
    function createMessage(type: "ReadMoldData", params: {
        controllerId: number;
        field: string;
    }, priority?: number): IReadMoldDataMessage;
    function createMessage(type: "JobCardsList", params: {
        controllerId: number;
        jobCards: IJobCard[];
    }, priority?: number): IJobCardsListMessage;
    function createMessage(type: "OperatorInfo", params: {
        controllerId: number;
        operatorId: number;
        password: string;
        name: string;
        level: number;
    }, priority?: number): IOperatorInfoMessage;
}
declare namespace iChen.OpenProtocol {
    type Languages = "EN" | "B5" | "GB" | "FR" | "DE" | "IT" | "ES" | "PT" | "JA" | "Unknown";
    type ControllerTypes = "Ai01" | "Ai02" | "Ai11" | "Ai12" | "CPC60" | "MPC60" | "CDC2000" | "CDC3000" | "CDC2000WIN" | "SPS3300" | "NewAge" | "CBmold300" | "CBmold800" | "Unknown";
    type OpModes = "Manual" | "SemiAutomatic" | "Automatic" | "Others" | "Offline" | "Unknown";
    type JobModes = "ID01" | "ID02" | "ID03" | "ID04" | "ID05" | "ID06" | "ID07" | "ID08" | "ID09" | "ID10" | "ID11" | "ID12" | "ID13" | "ID14" | "ID15" | "Offline";
    type CommandMessageTypes = "Alive" | "RequestControllersList" | "Join" | "RequestMoldData" | "ReadMoldData" | "JobCardsList" | "OperatorInfo";
    type ResponseMessages = "ControllersList" | "JoinResponse" | "MoldData" | "MoldDataValue" | "CycleData" | "ControllerAction" | "ControllerStatus" | "RequestJobCardsList" | "LoginOperator";
    type ValidMessageTypes = CommandMessageTypes | ResponseMessages;
    type KeyValue<K, V> = {
        key: K;
        value: V;
    };
    type Expando<T> = {
        [prop: string]: T;
    };
    type Dictionary<T> = {
        [key: string]: T;
    };
    type PropertiesMap = Expando<string>;
    interface IControllerStatus {
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
    interface IMessage {
        sequence: number;
        priority?: number;
        $type: ValidMessageTypes;
    }
    interface IDataDictionaryMessage<T> extends IMessage {
        data: Dictionary<T>;
    }
    interface IControllerSpecificMessage extends IMessage {
        controllerId: number;
    }
    interface IAliveMessage extends IMessage {
        $type: "Alive";
    }
    interface IRequestControllersListMessage extends IMessage {
        $type: "RequestControllersList";
        controllerId?: number;
    }
    interface IControllersListMessage extends IDataDictionaryMessage<IControllerStatus> {
        $type: "ControllersList";
    }
    interface IJoinMessage extends IMessage {
        $type: "Join";
        language: Languages;
        version: string;
        orgId?: string;
        password: string;
        filter?: string;
    }
    interface IJoinResponseMessage extends IMessage {
        $type: "JoinResponse";
        result: number;
        level: number;
        message: string;
    }
    interface IRequestMoldDataMessage extends IControllerSpecificMessage {
        $type: "RequestMoldData";
    }
    interface IReadMoldDataMessage extends IControllerSpecificMessage {
        $type: "ReadMoldData";
        field: string;
    }
    interface IMoldDataValueMessage extends IControllerSpecificMessage {
        $type: "MoldDataValue";
        field: string;
        value: number;
    }
    interface IDictionaryMessage extends IDataDictionaryMessage<number>, IControllerSpecificMessage {
        timestamp: Date;
        jobCardId?: string;
        moldId?: string;
        operatorId: number;
    }
    interface ICycleDataMessage extends IDictionaryMessage {
        $type: "CycleData";
    }
    interface IMoldDataMessage extends IDictionaryMessage {
        $type: "MoldData";
    }
    interface IControllerActionMessage extends IControllerSpecificMessage {
        $type: "ControllerAction";
        timestamp: Date;
        actionId: number;
    }
    interface IControllerStatusMessage extends IControllerSpecificMessage {
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
    interface IJobCard {
        jobCardId: string;
        moldId: string;
        progress: number;
        total: number;
    }
    interface IRequestJobCardsListMessage extends IControllerSpecificMessage {
        $type: "RequestJobCardsList";
    }
    interface IJobCardsListMessage extends IDataDictionaryMessage<IJobCard>, IControllerSpecificMessage {
        $type: "JobCardsList";
    }
    interface ILoginOperatorMessage extends IControllerSpecificMessage {
        $type: "LoginOperator";
        password: string;
    }
    interface IOperatorInfoMessage extends IControllerSpecificMessage {
        $type: "OperatorInfo";
        operatorId: number;
        name: string;
        password: string;
        level: number;
    }
}
