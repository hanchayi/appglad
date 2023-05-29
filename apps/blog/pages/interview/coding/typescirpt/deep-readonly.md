实现一个通用的DeepReadonly<T>，它将对象的每个参数及其子对象递归地设为只读。

您可以假设在此挑战中我们仅处理对象。数组，函数，类等都无需考虑。但是，您仍然可以通过覆盖尽可能多的不同案例来挑战自己。

例如

``` typescript
type X = {
  x: {
    a: 1
    b: 'hi'
  }
  y: 'hey'
}

type DeepReadonly<T> = keyof T extends never // 判断简单类型
  ? T
  : {readonly [P in keyof T]: DeepReadonly<T[P]>}

type Expected = {
  readonly x: {
    readonly a: 1
    readonly b: 'hi'
  }
  readonly y: 'hey'
}

type Todo = DeepReadonly<X> // should be same as `Expected`
```