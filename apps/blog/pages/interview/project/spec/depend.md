---
title: 依赖详解
---

## 依赖详解


#### depends依赖
> 在components/xxx.vue的depends中加依赖

代码：
``` typescript
depends: [ 'api.a3452e04183b49df93d882f7ec44c29b.duomai-store-service.ApiItemCategory' ]
```

定义:
``` typescript
export  interface MiniMethodPakcageItemDepend {
  type: 'depend';
  value: string;
}
```

Example:
``` javascript

{
	"type": "depend",
	"value": "api.a3452e04183b49df93d882f7ec44c29b.duomai-store-service.ApiItemCategory"
}
```

#### components依赖
> 在components/xxx.vue的components中加依赖

代码：
``` typescript
components: { LineChart: LineChart }
```

定义:
``` typescript
export interface MiniMethodPakcageItemComponent {
  type: 'component';
  value: MiniMethodPakcageItemComponentValue;
}

export interface MiniMethodPakcageItemComponentValue {
  name: string;
  source: string;
}
```

Example:
``` javascript

{
	"type": "component",
	"value": {
 		name: 'LineChart', // 使用的组件名
		source: 'LineChart', // 导入的名称，比如import { LineChart } from '@idg/charts'
	}
}
```

#### import依赖
> 在components/xxx.vue头部添加导入

代码：
``` typescript
import { Button as IviewButton } from '@idg/iview'
```

定义:
``` typescript
export interface MiniMethodPakcageItemImport {
  type: 'import';
  value: {
    source: string;
    imported: string; // 导入名
    local: string; // 本地名
  };
}
```

Example:
``` javascript

{
	"type": "import",
	"value": {
		"source": "@idg/iview",
		"imported": "Button",
		"local": "IviewButton"
	}
}
```

#### import default依赖
> 在components/xxx.vue头部添加默认导入
> import _ from 'lodash'

定义:
``` typescript
export interface MiniMethodPakcageItemImportDefault {
  type: 'import_default';
  value: {
    source: string;
    local: string;
  };
}
```

Example:
``` javascript
{
	"type": "import_default",
	"value": {
		"source": "lodash",
		"local": "_"
	}
}
```

#### use依赖(Vue插件)
> 在__gui__/uses.ts中生成插件依赖

代码：
``` typescript
import Vue from 'vue'
import element from 'element';
import 'element/dist/main.css';

Vue.use(element);
```


定义:
``` typescript
export interface MiniMethodPakcageItemUse {
  type: 'use';
  value: MiniMethodPakcageItemUseValue;
}

export interface MiniMethodPakcageItemUseValue {
  source: string; // 服务对应的npm包名
  local: string; // 变量名
  css?: string; // css路径
}
```

Example:
``` javascript
{
	"type": "use",
	"value": {
		"source": "element",
		"local": "Element",
		"css": "element/dist/main.css"
	}
}
```

#### service依赖
> 在__gui__/services.ts中生成服务依赖


代码:
``` typescript
import UcenterService from '@idg/ucenter';
new UcenterService({ channelAlias: 'default' })
```

定义:
``` typescript
export interface MiniMethodPakcageItemService {
  type: 'service';
  value: MiniMethodPakcageItemServiceValue;
}

export interface MiniMethodPakcageItemServiceValue {
  source: string; // 服务对应的npm包名
  local: string; // 变量名
  channelAlias: string; // 通道别名
  appid: string; // appid
}
```

Example:
``` javascript
{
	"type": "service",
	"value": {
		"source": "@idg/ucenter",
		"local": "UcenterService",
		"channelAlias": "default",
		"appid": "fsdfsadfasdfaasdfasaf"
	}
}
```

#### npm依赖
> 在package.json中
> "@idg/ucenter": "^0.3.4"

定义:
``` typescript
export  interface MiniMethodPakcageItemNpm {
  type: 'npm';
  value: {
    name: string; // npm包名
    version: string; // 版本
  };
}
```

Example:
``` javascript
{
	"type": "npm",
	"value": {
		"name": "@idg/ucenter",
		"version": "^0.3.4"
	}
}
```

#### package依赖
> 在__gui__/packages下生成文件及文件夹

代码:
``` typescript
__gui__
├── packages
│   ├── test-package
│   ├── index.ts // 内容export default { name: 'test-package' }
```

定义:
``` typescript
export  interface MiniMethodPakcageItemPackage {
  type: 'package';
  value: MiniMethodPakcageItemPackageValue;
}

export  interface MiniMethodPakcageItemPackageValue {
  name: string; // package的名称，__gui__+名字
  // 文件内容，必须base64加密，前端掉用btoa(unescape(encodeURIComponent(content))))即可
  files: Array<{ path: string; content: string; }>;
}
```

Example:
``` javascript
{
	"type": "package",
	"value": {
		"name": "test-package",
		"files": [
			{
				"path": "index.ts",
				"content": "ZXhwb3J0IGRlZmF1bHQgeyBuYW1lOiAnZ3VpLXRlc3QnIH0="
			}
		]
	}
}
```


#### 完整示例
``` json
[
    {
        "type":"use",
        "value":{
            "source":"@idg/iview",
            "local":"iview",
            "css":"@idg/iview/dist/styles/ant.css"
        }
    },
    {
        "type":"component",
        "value":{
            "name":"Button",
            "source":"Button"
        }
    },
    {
        "type":"npm",
        "value":{
            "name":"@idg/ucenter",
            "version":"^0.3.4"
        }
    },
    {
        "type":"packages",
        "value":{
            source: "@idg/gui-table"; // packages对应的npm包名
  	    local: "GuiTable"; // 变量名
        }
    },
    {
        "type":"package",
        "value":{
            "name":"test-package",
            "files":[
                {
                    "path":"index.ts",
                    "content":"ZXhwb3J0IGRlZmF1bHQgeyBuYW1lOiAnZ3VpLXRlc3QnIH0="
                }
            ]
        }
    },
    {
        "type":"service",
        "value":{
            "source":"@idg/ucenter",
            "local":"UcenterService",
            "channelAlias":"default",
	    "appid": "a3452e04183b49df93d882f7ec44c29b"
        }
    },
    {
        "type":"depend",
        "value":"api.a3452e04183b49df93d882f7ec44c29b.duomai-store-service.ApiItemCategory"
    },
    {
        "type":"import",
        "value":{
            "source":"@idg/iview",
            "imported":"Button",
            "local":"Button"
        }
    },
    {
        "type":"import_default",
        "value":{
            "source":"lodash",
            "local":"_"
        }
    }
]
```
