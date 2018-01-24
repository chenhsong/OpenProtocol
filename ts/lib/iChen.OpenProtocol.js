"use strict";
/// <reference path="../lib/iChen.OpenProtocol.d.ts" />
var iChen;
(function (iChen) {
    var OpenProtocol;
    (function (OpenProtocol) {
        var seq = 0;
        function throwParamError(msg) { throw "Invalid parameters for " + msg; }
        function getNextSequenceNumber() { return ++seq; }
        OpenProtocol.getNextSequenceNumber = getNextSequenceNumber;
        function createMessage(type, params, priority) {
            var msg = {
                $type: type,
                sequence: iChen.OpenProtocol.getNextSequenceNumber(),
                priority: priority || 0
            };
            switch (type) {
                case "Alive": {
                    msg.priority = (typeof params === "number") ? params : -10; // Default value = -10
                    break;
                }
                case "Join": {
                    if (!params || typeof params !== "object")
                        throw throwParamError(type);
                    var jmsg = msg;
                    jmsg.language = params.language;
                    jmsg.version = params.version;
                    if (params.orgId)
                        jmsg.orgId = params.orgId;
                    jmsg.password = params.password;
                    jmsg.filter = params.filter;
                    break;
                }
                case "RequestControllersList":
                    break;
                case "RequestMoldData": {
                    if (!params || typeof params !== "object")
                        throw throwParamError(type);
                    var rmsg = msg;
                    rmsg.controllerId = params.controllerId;
                    break;
                }
                case "ReadMoldData": {
                    if (!params || typeof params !== "object")
                        throw throwParamError(type);
                    var rmsg = msg;
                    rmsg.controllerId = params.controllerId;
                    rmsg.field = params.field;
                    break;
                }
                case "JobCardsList": {
                    if (!params || typeof params !== "object")
                        throw throwParamError(type);
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
                    if (!params || typeof params !== "object")
                        throw throwParamError(type);
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
//# sourceMappingURL=iChen.OpenProtocol.js.map