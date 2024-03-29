---
title: 组件协议规范
---

# 组件协议

## 版本信息

最新版本: 0.1.0

创建时间: 2022-09-24

更新时间: 2022-09-24

## 变更日志

| 时间 | 版本号 | 变更人 |  主要变更内容
| --- | ---- | --- | --- |
| 2022-09-24 | 0.1.0  |  张逸 | 创建协议

## 背景

组件是平台中左侧组件资源栏拖动出来的物料。

一个拖出来的组件实际对应到一个或多个真正的代码中的组件，比如拖一个上传组件。

上传组件本身是没有任何视图部分的，我们会在上传组件孩子里面加上一个布局和一个按钮，这样用户可以修改这个上传的显示也能使用上传的功能。

而每一个真正的代码组件会对应一份[组件配置](./component_config.md)，该配置记录改组件的属性、事件、舞台渲染和代码依赖。

由于跨端或某些其他需求，会需要我们组件的渲染和代码的数据不一致，故组件存在渲染树和代码树，如果没有代码树则认为他们是一致的。

## 协议结构

### Component
组件

组件资源栏能选到的所有组件

|参数|说明|类型|必须|
|--|--|--|--|
|uuid|唯一id|`string`|是|
|version| 组件版本|`string`|是|
|component_library_uuid| 	库uuid|`string`|是|
|component_category_uuid|分类uuid|`string`|是|
|terminal_uuid| 终端uuid|`string`|是|
|name| 组件名|`string`|是|
|label| 显示名|`string`|是|
|alias|	别名|`string`|是|
|render_tree| 渲染树|`ComponentTree`|是|
|code_tree| 组件树|`ComponentTree`|否|
|operation_tree| 操作树|`ComponentTree`|是|
|type| 组件类型|`string`|是|
|keywords| 	关键词|`string[]`|是|
|cover| 封面图|`string`|是|
|introduce| 组件介绍|`string`|是|
|config| 操作配置|`string`|是|
|meta|	元数据|`string`|是|
|sort_order| 排序优先级 |`number`|是|
|status| 状态 |`number`|是|

### ComponentTree
组件树

|参数|说明|类型|必须|
|--|--|--|--|
|id| 当前节点|`number`|是|
|parentId| 父节点|`number`|是|
|tag| 组件tag|`string`|是|
|data| 组件实例的数据|`ComponentData`|是|
|info| 附加信息 |`ComponentInfo`|否|
|children| 子节点|`ComponentTree[]`|是|
|version| 版本 |`string`|否|
|ins_id| 实例_id, 便于回溯和检索 |`string`|否|
|alias| 显示别名 |`string`|否|
|config_uuid| 组件的配置id |`string`|是|

### ComponentData

组件数据

|参数|说明|类型|必须|
|--|--|--|--|
|[propName]| 属性 |`any`|否|
|$binds| 绑定数据 |`Bind[]`|否|
|$event| 交互绑定 |`Record<string, BindMethod>`|否|
|$data_sync| 双向绑定  |`BindParam`|否|
|$condition_render|  条件渲染 |`ConditionRender[]`|否|
|$list_render| 循环渲染 |`ListRender[]`|否|
|$style|基础样式|`Style`|是|
|$styles| 其他分辨率下的样式|`ResolutionStyles`|否|
|$dynamic_style| 属性定义和值定义 单个或者多个|`DynamicStyle`|否|
|$slot| 具名插槽名|`string`|否|
|$slot_scope| 作用域插槽 |`SlotScope`|否|

### Bind

组件绑定

|参数|说明|类型|必须|
|--|--|--|--|
|attrKey| 暴露的属性key|`string`|否|
|eventKey| 暴露事件的key|`string`|否|
|bindParamId| 绑定的前端变量 id|`BindParam`|否|
|bindMethod| 绑定的前端方法调用,不是给交互绑方法用的，是函数类型的prop绑方法|`BindMethod`|否|

### BindParam

变量绑定

|参数|说明|类型|必须|
|--|--|--|--|
|id| id列表|`string[]`|是|
|type| 来自变量区，来自v-for，来自默认属性,可扩展|`ParamSource`|是|
|field| 例如取了表格的row,column或者index|`string`|否|
|vforKey| 绑了vfor需要设置一个属性作为key的值|`string`|否|


### ConditionRender

条件渲染数据

|参数|说明|类型|必须|
|--|--|--|--|
|bind||`Bind`|是|
|renderState| 舞台渲染控制 |`boolean`|否|


### ListRender

循环渲染

|参数|说明|类型|必须|
|--|--|--|--|
|bind||`Bind`|是|
|loopCount| 舞台渲染控制|`boolean`|否|

### ComponentInfo

附加的信息

|参数|说明|类型|必须|
|--|--|--|--|
|component_uuid| 对应表的uuid|`string`|是|
|code_tree| 代码树|`ComponentTree`|否|



### Depend
依赖

|参数|说明|类型|必须|
|--|--|--|--|
|type| 类型|`"npm" ｜ "use" | "import" | "import_default"`|是|
|value| 值|`any`|是|


### Style

基础分辨率静态样式

|参数|说明|类型|必须|
|--|--|--|--|
|[style]| 样式|`string`|是|

### ResolutionStyles

不同分辨率的样式

|参数|说明|类型|必须|
|--|--|--|--|
|resolution| 不同分辨率的样式|Style|是|


### DynamicStyle

动态样式

|参数|说明|类型|必须|
|--|--|--|--|
|[style]| 样式|`Bind`|是|


### SlotScope
作用域插槽

|参数|说明|类型|必须|
|--|--|--|--|
|label| 中文名|`string`|是|
|enLabel| 英文名|`string`|是|
|value| 插槽值|`SlotValue[]`|是|

### SlotValue

|参数|说明|类型|必须|
|--|--|--|--|
|key||`string`|否|
|label| 显示名|`string`|否|
|type| 类型|`string`|是|
|items| 数组类型时定义每一项|`SlotValue`|否|
|properties||`SlotValue[]`|否|
|attrKey| 变量类型时指定使用的变量key|`string`|否|
|defaultSchema| type=variable未绑定变量时，默认值的schema|`SlotValue`|否|


### BindMethod

|参数|说明|类型|必须|
|--|--|--|--|
|id| 方法对应的id|`string`|是|
|parameters| 参数|`string[]`|否|
|modifier| 事件修饰符的集合，例 stop|`Modifier[]`|否|




### Schema

|参数|说明|类型|必须|
|--|--|--|--|
|type| 数据类型|`DataType`|是|
|key| key|`string`|否|
|enumList| 枚举值|`EnumList[]\|`|否|
|items| type数组的时候|`Schema`|否|
|properties| type对象的时候|`properties`|否|
|label| 中文显示|`string`|否|

``` typescript
// 所有变量可能存在的类型
export enum DataType {
  Undefined = 'undefined',
  String = 'string',
  Number = 'number',
  Boolean = 'boolean',
  Object = 'object',
  Array = 'array',
  Function = 'function',
  Any = 'any',
  Integer = 'integer',
  Null = 'null',
}
```


## 示例数据

### ComponentTree


``` json
[
  {
    "id": 0,
    "parentId": -1,
    "tag": "div",
    "data": {
      "$style": {
        "width": "100%",
        "height": "100%"
      }
    },
    "config_uuid": "sal20000_org_key2211174ahjb33472"
  },
  {
    "id": 2,
    "parentId": 0,
    "ins_id": "063149f4cea740ea91161c701201df88",
    "tag": "Link",
    "data": {
      "disabled": false,
      "href": "https://www.baidu.com",
      "$text": "链接",
      "$style": {},
      "$binds": [
        {
          "attrKey": "$text",
          "bindParamId": {
            "type": "",
            "id": [
              "80372697f5104612b67c24e67f4ee70d"
            ]
          }
        },
      ],
      "$event":{
        "click":{
          "id":"cc11c76224b04a8cb2eaac4b6779e37b",
          "parameters":[],
          "modifier":[],
        }
      },
      "$condition_render": [
        {
          "renderState": false,
          "value": {
            "bindParamId": {
              "type": "",
              "id": [
                "36e718e00844452ea5c9ad56e3f81191"
              ]
            }
          }
        }
      ]
    },
    "config_uuid": "sal20000_org_key2211021urqvq34qm",
    "info": {
      "component_uuid": "sal20000_org_key22110241czlc343t",
    },
    "version": "0.0.1"
  }
]
```
