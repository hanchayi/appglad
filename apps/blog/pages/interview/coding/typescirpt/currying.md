
Currying 是一种将带有多个参数的函数转换为每个带有一个参数的函数序列的技术。

例如：

``` typescript
const add = (a: number, b: number) => a + b
const three = add(1, 2)

// type Currying<T> = T extends (arg: infer a, ...: infer rest) => infer b ? (a) => b : never;

type Unshift<T> = T extends [infer K, ...infer U] ? U : unknown
type Head<T> = T extends [infer K, ...infer U] ? K : unknown

type Curried<T, R> = T extends Array<any>
  ? T['length'] extends 1
    ? (args: Head<T>) => R
    : (args: Head<T>) => Curried<Unshift<T>, R>
  : never

declare function Currying<T extends unknown[], R>(fn: (...args: T) => R): Curried<T, R>

const curriedAdd = Currying(add)
const five = curriedAdd(2)(3)
```
