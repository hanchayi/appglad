---
title: Typescript严格模式
description: 严格模式的介绍及其影响
---

## 没有严格模式会有什么问题

让我们通过一个小例子进入这个话题。

以下 TypeScript 类在“默认模式”（也称为“草率模式”）下是可能的：

```typescript
class Beer {
  name: string;
  constructor() {}

  public test() {
    this.name.split('')
  }
}

const beer = new Beer()
beer.test() // this.name undifined
```

这边会有`undefined`的报错

结论：如果没有“严格模式”，TypeScript 编译器就无法保护我们的软件开发人员免受因缺少定义而导致的意外错误。


## 开启严格模式

`tsconfig.json`中开启

``` json
{
  "compilerOptions": {
    "strict": true
  }
}
```

## 严格模式开启的规则

- noImplicitAny // 不定义类型
- noImplicitThis // this未指定
- strictNullChecks // 对象可能为空
- strictPropertyInitialization // 构造函数初始化
- strictBindCallApply // bind apply类型校验
- strictFunctionTypes // 函数类型校验




## 参考

- [strict-mode-in-typescript](https://medium.com/@benlue/better-code-thanks-to-strict-mode-in-typescript-ae5ba74f17aa)

