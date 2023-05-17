---
title: 组件配置协议
---

# 组件配置协议

## 版本信息

最新版本: 0.1.0

创建时间: 2022-09-24

更新时间: 2022-09-24

## 变更日志

| 时间 | 版本号 | 变更人 |  主要变更内容
| --- | ---- | --- | --- |
| 2022-09-24 | 0.1.0  |  张逸 | 创建协议

## 背景

每一个真正的代码组件会对应一份组件配置，该配置记录改组件的属性、事件、舞台渲染和代码依赖。

## 协议结构



### ComponentConfig
组件配置

|参数|说明|类型|必须|
|--|--|--|--|
|uuid| 唯一id|`string`|是|
|version| 版本|`string`|是|
|name| 组件名|`string`|是|
|label| 显示名|`string`|是|
|props| 属性|`ComponentConfigProp`|是|
|depends| 依赖|`string`|是|

### ComponentConfigProp

配置属性

|参数|说明|类型|必须|
|--|--|--|--|
|key| 属性英文名|`string`|是|
|label| 属性中文名|`string`|是|
|schema| 值结构|`Schema`|否|
|category| prop类型|`PropType`|是|
|value| 设定值|`any`|否|
|default|默认值|`any`|否|
|payload| 事件参数定义|`PayLoad[]`|否|


``` typescript
type PropType = 'attr' | 'event' | 'slot' | 'param' | 'render' | 'cdn';
```


### 特殊render的属性

| key      | 类型 |  描述 |
| --- | --- | ---
| path      |     `RenderPath`    |  渲染路径
| isLayout   | boolean        |   是不是布局
| doubleClickEdit   |     DoubleClickEdit    |   支持双击编辑
| dynamicView   | DymanicView        |   是不是动态视图
| ...   | ...        |   ...


#### RenderPath

|参数|说明|类型|必须|
|--|--|--|--|
|type||`RenderPathType`|是|
|appid||`string`|是|
|remoteEntry||`string`|是|
|path||`string`|是|
|dependServiceAppid||`string`|否|

#### DoubleClickEdit

|参数|说明|类型|必须|
|--|--|--|--|
|matchType| 匹配类型 |`"tag"\|"className"`|是|
|match| 匹配值 |`string`|是|
|key| 修改属性key|`string`|是|
|target| 目标元素|`string`|否|

### DymanicView

|参数|说明|类型|必须|
|--|--|--|--|
|showKey| 显示修改的属性 |`string`|是|



### Depend
依赖是使用该组件所需的外部依赖，详情参考[依赖](./depend.md)

|参数|说明|类型|必须|
|--|--|--|--|
|type| 类型|`"npm" ｜ "use" | "import" | "import_default"`|是|
|value| 值|`any`|是|


### Schema

数据类型定义

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

### ComponentConfig

``` json
{
  "uuid": "sal20000_org_key2112063qv5zi6dpn",
  "name": "IdgNetdiskUpload",
  "label": "网盘上传",
  "props": [
    {
      "key": "default-file-list",
      "label": "默认已上传的文件列表",
      "category": "attr",
      "schema": {
        "type": "array",
        "items": {
          "type": "object",
          "properties": {
            "name": { "type": "string" },
            "url": { "type": "string" },
          }
        }
      },
      "default": [],
    },
    {
      "key": "before-upload",
      "label": "上传文件之前的钩子",
      "category": "attr",
      "schema": {
        "type": "function"
      },
      "default": {},
      "payload": [
        {
          "key": "file",
          "label": "文件",
          "schema": {
            "type": "object",
            "properties": {
              "name": { "type": "string" },
              "size": { "type": "number" },
              "type": { "type": "string" },
            }
          }
        }
      ]
    },
    {
      "key": "path",
      "value":{
          "type":"remote",
          "appid":"34df8e51573c4a7686f74bda9b55c915",
          "remoteEntry":"//pkg.oneitfarm.com/@idg/uplpad-cropper/0.0.11/remoteEntry.js",
          "path":"34df8e51573c4a7686f74bda9b55c915.uplpadcopper.UploadCopper"
      },
      "category":"render"
    }
  ],
  "depends": [
    {
      "type":"async-service",
      "value":{
          "source":"@idg/netdisk",
          "channelAlias":"default",
          "appid":"ole8vcm1jivxzjywstgh9kfror2qa3d7",
          "version":"2.7.11"
      }
    },
    {
      "type":"depend",
      "value":{
          "id":"component.ole8vcm1jivxzjywstgh9kfror2qa3d7.upload.Upload",
          "variableName":"IdgNetdiskUpload"
      }
    }
  ]
}

```
