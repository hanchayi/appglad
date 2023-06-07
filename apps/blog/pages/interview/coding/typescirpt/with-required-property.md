---
title: typescript让可选属性变成必选
---


## 在 TypeScript 中设置某些可选属性为必需

要使可选属性成为必需属性，请创建一个实用程序类型，该实用程序类型使用映射修饰符来删除指定属性的可选性。

新类型将具有标记为必需的指定属性。

``` typescript
interface Employee {
  id?: number;
  name: string;
  salary?: number;
}

type WithRequiredProperty<Type, Key extends keyof Type> = Type & {
  [Property in Key]-?: Type[Property];
};

// 👇️ Make salary required
const emp1: WithRequiredProperty<Employee, 'salary'> = {
  name: 'Bobby Hadz',
  salary: 100,
};

// 👇️ Make salary and id required
const emp1: WithRequiredProperty<Employee, 'salary' | 'id'> = {
  id: 0,
  name: 'Bobby Hadz',
  salary: 100,
};
```

我们创建了一个实用程序类型，它接受一个类型和一个属性名称，并使该属性成为必需的。


您可以使用实用程序类型来创建多个属性，方法是用竖线分隔它们的名称。

`-?:` 语法称为[映射修饰符](https://www.typescriptlang.org/docs/handbook/2/mapped-types.html#mapping-modifiers)，用于影响可选性。


## 让所有属性必选

使用typescript内置的 `Required` 工具类型使类型中的所有属性成为必需的。

``` typescript
interface Employee {
  id?: number;
  name?: string;
  salary?: number;
}

const emp: Required<Employee> = {
  id: 1,
  name: 'Bobby Hadz',
  salary: 1000,
};

```

`Required`源码
``` typescript
type Required<T> = {
  [P in keyof T]-?: P[T];
}
```


## 参考

[make-property-required](https://bobbyhadz.com/blog/typescript-make-property-required)
