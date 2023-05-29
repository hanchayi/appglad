实现泛型`GetReadonlyKeys<T>`，`GetReadonlyKeys<T>`返回由对象 T 所有只读属性的键组成的联合类型。

例如

```ts
interface Todo {
  readonly title: string
  readonly description: string
  completed: boolean
}

type GetReadonlyKeys<T> = {
  [K in keyof T]:
    (<S>() => S extends { [Z in K]: T[Z] } ? 2 : 1) extends
    (<S>() => S extends { -readonly [Z in K]: T[Z] } ? 2: 1) ? never : K
}[keyof T];

type Keys = GetReadonlyKeys<Todo> // expected to be "title" | "description"
```



首先可以确定的是返回类型是联合类型，类型值是泛型 T 中被 readonly 修饰的索引键名


1. 为了挑选出符合条件的键需要对泛型 T 的每一项进行比较，这时候需要确定比较的方式，一个比较好的思路是利用 Equal 直接和符合条件的索引作比较，\
  即 Equal<{ [key in keyof T]: T[key] } , { readonly [Q]: T[Q] }>
2. 为了实现对泛型的每一项进行比较需要有两个前置工作 - 利用联合类型分发 key、通过 Pick 提取当前 key 的类型
3. 利用联合类型分发 key 要求联合类型 key 约束为 T 类型，即 type GetReadonlyKeys<T, Key extends keyof T>
4. Pick<T, K> 会从类型 T 中选择出属性 K，构造成一个新的类型，所以有 Equal<Pick<T, K>, Readonly<Pick<T, K>>>

结合上述分析得到 type GetReadonlyKeys<T, K extends keyof T> = Equal<Pick<T, K>, Readonly<Pick<T, K>>> extends true ? K : never

