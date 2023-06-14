---
title: 如何根据参数返回不同的类型
---


## 实现

```typescript
interface ResourceTypeMap {
  a: ResourceA;
  b: ResourceB;
}


function getResourceByType(type: T extends keyof ResourceTypeMap): ResourceTypeMap[T]{}
```


## 参考

```typescript
createElement<K extends keyof HTMLElementTagNameMap>(tagName: K, options?: ElementCreationOptions): HTMLElementTagNameMap[K];

interface HTMLElementTagNameMap {
  "a": HTMLAnchorElement;
  "abbr": HTMLElement;
  "address": HTMLElement;
  "area": HTMLAreaElement;
  "article": HTMLElement;
  "aside": HTMLElement;
  "audio": HTMLAudioElement;
  "b": HTMLElement;
}
```
