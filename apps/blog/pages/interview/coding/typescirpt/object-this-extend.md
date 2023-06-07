---
title: typescript代码中对象的this扩展
---

平时我们定义一个对象，它的this只能在typescript中看到当前定义在对象中的属性

而有些属性是运行过程中动态merge进去的

``` typescript
const graphOptions = {
  test() {
    console.log(this.graph) // 这边会找不到graph
  }
}
```

## This type

``` typescript
interface Graph {}

const graphOptions: ThisType<{ graph: Graph }> = {
  test() {
    console.log(this.graph)
}
```

## Vue的实现

在使用Vue的时候，methods的this可以有正确的typescript类型提示

方法的第一个参数可以指定this的类型


``` typescript
// 扩展Vue的interface定义
declare module 'vue/types/vue' {
  interface Vue {
    $Message: Message;
    $prettyDom: any;
  }
}

// 定义Methods指向V
type DefaultMethods<V> =  { [key: string]: (this: V, ...args: any[]) => any };

// V继承了Vue
export interface ComponentOptions<
  V extends Vue,
  Data=DefaultData<V>,
  Methods=DefaultMethods<V>,
  Computed=DefaultComputed,
  PropsDef=PropsDefinition<DefaultProps>,
  Props=DefaultProps> {
  data?: Data;
  props?: PropsDef;
  propsData?: object;
  computed?: Accessors<Computed>;
  methods?: Methods;
}
```

