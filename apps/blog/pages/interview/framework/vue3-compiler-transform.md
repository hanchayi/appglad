---
title: vue3 compiler transform
description: Vue3重写的compiler实现
---


nodeTransforms 而言，涉及到对 node 节点的转化，同时 node 节点的转化的过程类似 koa 中的洋葱模型，因为每个 nodeTransform 都有类似 enter 和 exit 事件，第一个执行的 nodeTransform 它的 exit 事件最后执行。

```typescript
export function traverseNode(
  node: RootNode | TemplateChildNode,
  context: TransformContext
) {
  context.currentNode = node
  const { nodeTransforms } = context
  const exitFns = []
  for (let i = 0; i < nodeTransforms.length; i++) {
    // 执行每个transform
    const onExit = nodeTransforms[i](node, context)
    if (onExit) {
      if (isArray(onExit)) {
        exitFns.push(...onExit)
      } else {
        exitFns.push(onExit)
      }
    }
    if (!context.currentNode) {
      return
    } else {
      node = context.currentNode
    }
  }

  // 退出每个trasform
  context.currentNode = node
  let i = exitFns.length
  while (i--) {
    exitFns[i]()
  }
}
```


## Reference

https://hkc452.github.io/slamdunk-the-vue3/main/vue/compiler/transform.html
