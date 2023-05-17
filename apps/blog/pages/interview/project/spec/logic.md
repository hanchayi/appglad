---
逻辑编辑器
---


# 逻辑编辑器实现

积木块 Blockly
蓝图 G6


## blockly示例

``` json
<xml xmlns="https://developers.google.com/blockly/xml">
    <block type="show_function" id="xDCRtO4QI92@3(;(Ft]i" deletable="false" x="288" y="283">
        <mutation funcId="1d0a0a876655498ba263f2ad4de65cc1"></mutation>
        <statement name="showFunc">
            <block type="show_message" id="/-$];P6WykyxPTaAf|c)">
                <mutation arguments=""></mutation>
                <field name="type">info</field>
                <value name="content">
                    <block type="data_schema_get_cascader" id="u`udzSGQL^OC5((y3fX0">
                        <mutation dataId="9616a478992646a0a5cd57ef41f1e438"
                            valueList="[&quot;9616a478992646a0a5cd57ef41f1e438&quot;]"></mutation>
                        <field name="data">9616a478992646a0a5cd57ef41f1e438</field>
                    </block>
                </value>
            </block>
        </statement>
    </block>
</xml>
```

## 蓝图示例

``` xml
{
    "methods": [
        {
            "uuid": "6fcb0dad15e74e73904f389db4e1b8f1",
            "type": "api",
            "variables": [
                {
                    "id": "7fbaa58993164d5c88784f3bd5ccb50b",
                    "label": "默认整型",
                    "name": "morenzhengxing",
                    "value": 0,
                    "schema": {
                        "type": "integer"
                    }
                }
            ],
            "methodType": "get",
            "parameters": [],
            "funcReturn": {
                "defaultValue": "1",
                "schema": {
                    "type": "integer"
                }
            },
            "funcLabel": "test",
            "funcName": "test",
            "path": "test",
            "folderId": "sal200005a4dc54b2305174jbg5faeyf",
            "graphData": {
                "nodes": [
                    {
                        "id": "logic-start-node",
                        "x": 175,
                        "y": 284,
                        "type": "logic-start-node",
                        "label": {
                            "en_us": "Start",
                            "zh_cn": "开始"
                        },
                        "data": {
                            "label": {
                                "en_us": "Start",
                                "zh_cn": "开始"
                            },
                            "anchors": [
                                {
                                    "nodeId": "logic-start-node",
                                    "tag": "statement_output",
                                    "index": 0,
                                    "connected": true,
                                    "data": {
                                        "label": {
                                            "zh_cn": "开始",
                                            "en_us": "Start"
                                        },
                                        "name": "output"
                                    }
                                }
                            ]
                        },
                        "style": {
                            "nodeState:default": {
                                "cursor": "default",
                                "stroke": "#d9d9d9",
                                "lineWidth": 1,
                                "shadowColor": null
                            },
                            "nodeState:hover": {
                                "lineWidth": 1,
                                "stroke": "#d9d9d9",
                                "shadowColor": "#d9d9d9"
                            },
                            "nodeState:selected": {
                                "lineWidth": 2,
                                "stroke": "#1890ff",
                                "shadowColor": null
                            },
                            "nodeState:matched": {
                                "lineWidth": 2,
                                "stroke": "#1890ff",
                                "shadowColor": null
                            }
                        },
                        "nodeWidth": 210,
                        "nodeHeight": 134
                    },
                    {
                        "type": "logic-end-node",
                        "id": "logic-end-node",
                        "x": 740,
                        "y": 284,
                        "label": {
                            "en_us": "End",
                            "zh_cn": "结束"
                        },
                        "data": {
                            "label": {
                                "en_us": "End",
                                "zh_cn": "结束"
                            },
                            "anchors": [
                                {
                                    "tag": "statement_input",
                                    "index": 0,
                                    "nodeId": "logic-end-node",
                                    "connected": true,
                                    "data": {
                                        "_isEntry": true,
                                        "label": {
                                            "zh_cn": "结束",
                                            "en_us": "End"
                                        }
                                    }
                                },
                                {
                                    "nodeId": "logic-end-node",
                                    "tag": "var_input",
                                    "index": 1,
                                    "connected": true,
                                    "data": {
                                        "tag": "var_input",
                                        "type": "integer",
                                        "label": {
                                            "zh_cn": "返回值",
                                            "en_us": "Return value"
                                        },
                                        "name": "return_value",
                                        "schema": {
                                            "type": "integer"
                                        }
                                    }
                                }
                            ]
                        },
                        "style": {
                            "nodeState:default": {
                                "cursor": "default",
                                "stroke": "#d9d9d9",
                                "lineWidth": 1,
                                "shadowColor": null
                            },
                            "nodeState:hover": {
                                "lineWidth": 1,
                                "stroke": "#d9d9d9",
                                "shadowColor": "#d9d9d9"
                            },
                            "nodeState:selected": {
                                "lineWidth": 2,
                                "stroke": "#1890ff",
                                "shadowColor": null
                            },
                            "nodeState:matched": {
                                "lineWidth": 2,
                                "stroke": "#1890ff",
                                "shadowColor": null
                            }
                        },
                        "nodeWidth": 210,
                        "nodeHeight": 134
                    },
                    {
                        "id": "16843090773545346",
                        "x": 420.796875,
                        "y": 419,
                        "type": "logic-integer-node",
                        "data": {
                            "varId": "7fbaa58993164d5c88784f3bd5ccb50b",
                            "varName": "morenzhengxing",
                            "varLabel": "默认整型",
                            "schema": {
                                "type": "integer"
                            },
                            "anchors": [
                                {
                                    "nodeId": "16843090755052236",
                                    "tag": "var_output",
                                    "index": 0,
                                    "data": {
                                        "label": "默认整型",
                                        "value": 0,
                                        "type": "integer",
                                        "name": "morenzhengxing",
                                        "_route_path": "7fbaa58993164d5c88784f3bd5ccb50b",
                                        "schema": {
                                            "type": "integer"
                                        }
                                    },
                                    "connected": true
                                }
                            ]
                        },
                        "style": {
                            "nodeState:default": {
                                "cursor": "default",
                                "stroke": "#d9d9d9",
                                "lineWidth": 1,
                                "shadowColor": null
                            },
                            "nodeState:hover": {
                                "lineWidth": 1,
                                "stroke": "#d9d9d9",
                                "shadowColor": "#d9d9d9"
                            },
                            "nodeState:selected": {
                                "lineWidth": 2,
                                "stroke": "#1890ff",
                                "shadowColor": null
                            },
                            "nodeState:matched": {
                                "lineWidth": 2,
                                "stroke": "#1890ff",
                                "shadowColor": null
                            }
                        },
                        "nodeHeight": 40,
                        "nodeWidth": 150
                    },
                    {
                        "label": {
                            "en_us": "Throw exception",
                            "zh_cn": "抛出异常"
                        },
                        "id": "16843090888216512",
                        "type": "logic-throw-exception-node",
                        "x": 419.796875,
                        "y": 208,
                        "data": {
                            "anchors": [
                                {
                                    "tag": "statement_input",
                                    "data": {
                                        "label": {
                                            "zh_cn": "输入",
                                            "en_us": "Input"
                                        },
                                        "name": "input"
                                    },
                                    "index": 0,
                                    "nodeId": "16843090888216512",
                                    "connected": true
                                },
                                {
                                    "tag": "statement_output",
                                    "data": {
                                        "label": {
                                            "zh_cn": "输出",
                                            "en_us": "Output"
                                        },
                                        "name": "output"
                                    },
                                    "index": 1,
                                    "nodeId": "16843090888216512",
                                    "connected": true
                                },
                                {
                                    "tag": "var_input",
                                    "data": {
                                        "label": {
                                            "zh_cn": "提示内容",
                                            "en_us": "content"
                                        },
                                        "schema": {
                                            "type": "string"
                                        },
                                        "name": "content",
                                        "type": "string",
                                        "value": "\"sdf\""
                                    },
                                    "index": 2,
                                    "nodeId": "16843090888216512",
                                    "connected": false
                                },
                                {
                                    "tag": "var_input",
                                    "data": {
                                        "label": {
                                            "zh_cn": "错误码",
                                            "en_us": "code"
                                        },
                                        "schema": {
                                            "type": "number"
                                        },
                                        "name": "code",
                                        "type": "number",
                                        "value": "1"
                                    },
                                    "index": 3,
                                    "nodeId": "16843090888216512",
                                    "connected": false
                                }
                            ]
                        },
                        "style": {
                            "nodeState:default": {
                                "cursor": "default",
                                "stroke": "#d9d9d9",
                                "lineWidth": 1,
                                "shadowColor": null
                            },
                            "nodeState:hover": {
                                "lineWidth": 1,
                                "stroke": "#d9d9d9",
                                "shadowColor": "#d9d9d9"
                            },
                            "nodeState:selected": {
                                "lineWidth": 2,
                                "stroke": "#1890ff",
                                "shadowColor": null
                            },
                            "nodeState:matched": {
                                "lineWidth": 2,
                                "stroke": "#1890ff",
                                "shadowColor": null
                            }
                        },
                        "nodeWidth": 210,
                        "nodeHeight": 164
                    }
                ],
                "edges": [
                    {
                        "type": "logic-variable-edge",
                        "id": "16843090794992032",
                        "source": "16843090773545346",
                        "target": "logic-end-node",
                        "sourceAnchor": 0,
                        "targetAnchor": 1,
                        "style": {
                            "edgeState:default": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 2,
                                "stroke": "#f759ab",
                                "lineAppendWidth": 10,
                                "endArrow": {
                                    "lineDash": [
                                        0
                                    ],
                                    "path": "M 0,0 L 8,4 L 7,0 L 8,-4 Z",
                                    "d": 0,
                                    "fill": "#f759ab",
                                    "stroke": "#f759ab"
                                }
                            },
                            "edgeState:selected": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 2,
                                "stroke": "#f759ab",
                                "lineAppendWidth": 10,
                                "endArrow": {
                                    "lineDash": [
                                        0
                                    ],
                                    "path": "M 0,0 L 8,4 L 7,0 L 8,-4 Z",
                                    "d": 0,
                                    "fill": "#f759ab",
                                    "stroke": "#f759ab"
                                }
                            },
                            "edgeState:hover": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 2,
                                "stroke": "#f759ab",
                                "lineAppendWidth": 10,
                                "endArrow": {
                                    "lineDash": [
                                        0
                                    ],
                                    "path": "M 0,0 L 8,4 L 7,0 L 8,-4 Z",
                                    "d": 0,
                                    "fill": "#f759ab",
                                    "stroke": "#f759ab"
                                }
                            },
                            "radius": 6,
                            "offset": -15,
                            "lineWidth": 2,
                            "stroke": "#f759ab",
                            "lineAppendWidth": 10,
                            "endArrow": {
                                "lineDash": [
                                    0
                                ],
                                "path": "M 0,0 L 8,4 L 7,0 L 8,-4 Z",
                                "d": 0,
                                "fill": "#f759ab",
                                "stroke": "#f759ab"
                            }
                        },
                        "stateStyles": {
                            "edgeState:default": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 2,
                                "stroke": "#f759ab",
                                "lineAppendWidth": 10,
                                "endArrow": {
                                    "lineDash": [
                                        0
                                    ],
                                    "path": "M 0,0 L 8,4 L 7,0 L 8,-4 Z",
                                    "d": 0,
                                    "fill": "#f759ab",
                                    "stroke": "#f759ab"
                                }
                            },
                            "edgeState:selected": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 2,
                                "stroke": "#f759ab",
                                "lineAppendWidth": 10,
                                "endArrow": {
                                    "lineDash": [
                                        0
                                    ],
                                    "path": "M 0,0 L 8,4 L 7,0 L 8,-4 Z",
                                    "d": 0,
                                    "fill": "#f759ab",
                                    "stroke": "#f759ab"
                                }
                            },
                            "edgeState:hover": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 2,
                                "stroke": "#f759ab",
                                "lineAppendWidth": 10,
                                "endArrow": {
                                    "lineDash": [
                                        0
                                    ],
                                    "path": "M 0,0 L 8,4 L 7,0 L 8,-4 Z",
                                    "d": 0,
                                    "fill": "#f759ab",
                                    "stroke": "#f759ab"
                                }
                            }
                        },
                        "startPoint": {
                            "x": 496.296875,
                            "y": 419,
                            "anchorIndex": 0
                        },
                        "endPoint": {
                            "x": 634.5,
                            "y": 321.27611940298505,
                            "anchorIndex": 1
                        }
                    },
                    {
                        "type": "logic-statement-edge",
                        "id": "16843090955277911",
                        "source": "logic-start-node",
                        "target": "16843090888216512",
                        "sourceAnchor": 0,
                        "targetAnchor": 0,
                        "style": {
                            "edgeState:default": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            },
                            "edgeState:selected": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            },
                            "edgeState:hover": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            },
                            "radius": 6,
                            "offset": -15,
                            "lineWidth": 3,
                            "stroke": "#1890FF",
                            "lineAppendWidth": 10,
                            "endArrow": null
                        },
                        "stateStyles": {
                            "edgeState:default": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            },
                            "edgeState:selected": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            },
                            "edgeState:hover": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            }
                        },
                        "startPoint": {
                            "x": 280.5,
                            "y": 291.05223880597015,
                            "anchorIndex": 0
                        },
                        "endPoint": {
                            "x": 314.296875,
                            "y": 199.9512195121951,
                            "anchorIndex": 0
                        }
                    },
                    {
                        "type": "logic-statement-edge",
                        "id": "16843090974267047",
                        "source": "16843090888216512",
                        "target": "logic-end-node",
                        "sourceAnchor": 1,
                        "targetAnchor": 0,
                        "style": {
                            "edgeState:default": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            },
                            "edgeState:selected": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            },
                            "edgeState:hover": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            },
                            "radius": 6,
                            "offset": -15,
                            "lineWidth": 3,
                            "stroke": "#1890FF",
                            "lineAppendWidth": 10,
                            "endArrow": null
                        },
                        "stateStyles": {
                            "edgeState:default": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            },
                            "edgeState:selected": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            },
                            "edgeState:hover": {
                                "radius": 6,
                                "offset": -15,
                                "lineWidth": 3,
                                "stroke": "#1890FF",
                                "lineAppendWidth": 10,
                                "endArrow": null
                            }
                        },
                        "startPoint": {
                            "x": 525.296875,
                            "y": 199.9512195121951,
                            "anchorIndex": 1
                        },
                        "endPoint": {
                            "x": 634.5,
                            "y": 291.05223880597015,
                            "anchorIndex": 0
                        }
                    }
                ],
                "combos": []
            }
        }
    ]
}
```

##
