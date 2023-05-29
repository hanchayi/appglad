---
title: infer
description: extends中推导的条件变量
---

简单示例如下：
``` typescript
type ParamType<T> = T extends (args: infer P) => any ? P : T;
```

其中`infer P`表示待推断的函数参数类型。

整句的语意如果T能赋值给`(args: infer P) => any`则返回P，否则返回T。


