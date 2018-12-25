"use strict";
var iChen;
(function (iChen) {
    var OpenProtocol;
    (function (OpenProtocol) {
        // Auto-increment sequence number
        var seq = 0;
        // Allow iChen.OpenProtocol.getNextSequenceNumber to be overridden
        function getNextSequenceNumber() { return ++seq; }
        OpenProtocol.getNextSequenceNumber = getNextSequenceNumber;
        // Main createMessage method overload template
        function createMessage(type, params, priority) {
            // Create message object
            var msg = { $type: type, sequence: iChen.OpenProtocol.getNextSequenceNumber(), priority: priority || 0 };
            // Some messages require no parameter
            if (params === undefined || typeof params === "number") {
                switch (type) {
                    case "Alive": {
                        // Default priority value for Alive messages = -10
                        msg.priority = (params !== undefined) ? params : -10;
                        break;
                    }
                    case "RequestControllersList": break;
                    default: throw "Parameters missing for " + type + " message.";
                }
                return msg;
            }
            // Other messages require a parameter
            if (typeof params !== "object")
                throw "Invalid parameters for " + type + " message.";
            switch (type) {
                case "Alive": throw "No parameters allowed for " + type + " message.";
                case "Join": {
                    var jmsg = msg;
                    jmsg.language = params.language;
                    jmsg.version = params.version;
                    if (params.orgId)
                        jmsg.orgId = params.orgId;
                    jmsg.password = params.password;
                    jmsg.filter = params.filter;
                    break;
                }
                case "RequestControllersList": {
                    var rmsg = msg;
                    rmsg.controllerId = params.controllerId;
                    break;
                }
                case "RequestMoldData": {
                    var rmsg = msg;
                    rmsg.controllerId = params.controllerId;
                    break;
                }
                case "ReadMoldData": {
                    var rdmsg = msg;
                    rdmsg.controllerId = params.controllerId;
                    rdmsg.field = params.field;
                    break;
                }
                case "JobCardsList": {
                    var jcmsg = msg;
                    var list = params.jobCards;
                    jcmsg.controllerId = params.controllerId;
                    jcmsg.data = {};
                    for (var _i = 0, list_1 = list; _i < list_1.length; _i++) {
                        var jc = list_1[_i];
                        jcmsg.data[jc.jobCardId] = jc;
                    }
                    break;
                }
                case "OperatorInfo": {
                    var omsg = msg;
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
        OpenProtocol.createMessage = createMessage;
    })(OpenProtocol = iChen.OpenProtocol || (iChen.OpenProtocol = {}));
})(iChen || (iChen = {}));
// Type Definitions
//# sourceMappingURL=iChen.OpenProtocol.js.map