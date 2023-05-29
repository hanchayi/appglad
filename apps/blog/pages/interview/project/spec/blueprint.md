---
title: 蓝图架构图
description: 蓝图的核心模块和技术架构
---

## 架构图

![image](./blueprint.drawio.svg)

## 包

- editor     编辑器UI
- generator  代码生成器
- graph      绘图
- store      数据
- core       核心类型定义

##  类

```mermaid
classDiagram
    class Editor{
      constructor(container)
      +changeMode() // 切换变量or方法视图
    }

    class Generator{
      -string framework
      +generate()
    }

    class GraphView {
      -GraphModel graphModel
      -behaviors
      -shapes
      -events
      -anchors
      +init()
    }

    class GraphModel {
      +edges
      +nodes
    }

    class Store {
        -methods
        -variables
        -editor
        +addMethod()
    }

    class NodeFactory {
      +registerNode()
    }

    class NodeConfig {
      +string type
      +string translateType
      +NodeConfigData data
    }

    class NodeModel {
      +string version
      +string tag
    }

    class NodeService {
      +node2code(NodeModel nodeModel)
    }
```
