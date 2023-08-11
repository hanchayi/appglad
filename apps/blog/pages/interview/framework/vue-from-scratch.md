# 从零开始实现一个Vue

为了更好的认识Vue，应付面试或者写出更好的代码，本文逐步创建一个类似于 Vue这样的响应式框架（或至少是它的原型）。

[github地址](https://github.com/hanchayi/vue-from-scratch/tree/main)

本文默认你已经使用过Vue并对其相对熟悉

## vnode

虚拟dom本质就是用一个对象代表一个实际的dom，然后根据vnode的变化去更新对应的dom。

新建`h.js`:

``` javascript
export function h(tag, props, children) {
  return {
    tag,
    props,
    children
  }
}
```

我们知道Vue提供了[h](https://cn.vuejs.org/guide/extras/render-function.html#creating-vnodes)函数来渲染虚拟DOM，我们这边将参数包装成一个对象返回即可。

新建`element.js`:

``` javascript
export function createElement(vnode) {
  const element = document.createElement(vnode.tag)
  if (vnode.$on && vnode.$on.click) {
    element.addEventListener('click', vnode.$on.click)
  }
  Object.keys(vnode.props).forEach(prop => {
    element.setAttribute(prop, vnode.props[prop])
  })

  if (vnode.children) {
    if (Array.isArray(vnode.children)) {
      vnode.children.forEach(child => {
        element.appendChild(createElement(child))
      })
    } else if (typeof vnode.children === 'string') {
      element.innerHTML = vnode.children
    } else {
      throw new Error('invalid vnode')
    }
  }

  vnode.el = element
  return element
}
```

`createElement`的作用将`VNode`转成dom节点
- 消费$on来进行事件绑定
- 消费props设置属性
- children如果是字符串则设置文本，否则递归创建


新建`mount.js`:

```javascript
import { createElement } from './element.js'

export function mount(vnode, container) {
  if (typeof container === 'string') {
    container = document.querySelector(container)
  }
  container.appendChild(createElement(vnode))
}

export function unmount(vnode) {
  if (!vnode.el || !vnode.el.parentNode) {
    throw new Error('invalid vnode')
  }

  vnode.el.parentNode.removeChild(vnode.el)
}
```

`mount`函数则将虚拟节点转换成`DOM`并挂载到对应`container`上，这边如果传入的是字符串则当`selector`处理剩下的节点


新建`vue.js`:

``` javascript

import { mount } from "./mount.js"
import { h } from './h.js'

export function createApp(App) {
  const app = App.render(h)
  return {
    mount: (container) => {
      mount(app, container)
      return app
    },
  }
}

```

导出启动函数

新建`index.html`和`test.js`用于我们测试的入口：

``` html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>
    <div id="app">
    </div>
    <script type="module" src="test.js">
    </script>
</body>
</html>
```

``` javascript
import { createApp } from './vue.js'
import { reactive } from './reactive.js'

const data = reactive({
  hello: 'hello',
  world: 'world'
})

const App = {
  render(h) {
    return h('div', {}, [
      h('span', { style: 'color: red' }, data.hello),
      h('span', { style: 'color: green' }, data.world),
    ])
  }
}

createApp(App).mount('#app')
```


这边作为测试的入口主要掉用`vue.js`中的`createApp`函数，然后`App`是一个简单的Vue组件。

> 这边能够直接使用import语法是因为使用了 `type="module"`

运行的结果将能看到`hello world`

## patch

patch函数用于对比新旧VNode，然后转化成对应的dom操作。

新增`patch.js`:

``` javascript
import { mount, unmount } from "./mount.js"

export function patch(oldNode, newNode) {
  // tag不一样
  if (oldNode.tag !== newNode.tag) {
    mount(newNode, oldNode.el.parentNode)
    unmount(oldNode)
    return
  }

  // props不一样
  if (JSON.stringify(oldNode.$props) !== JSON.stringify(newNode.$props)) {
    mount(newNode, oldNode.el.parentNode)
    unmount(oldNode)
    return
  }

  newNode.el = oldNode.el
  // 文本节点
  if (typeof newNode.children === 'string') {
    if (oldNode.children === newNode.children) {
      return
    }
    oldNode.el.innerHTML = newNode.children
    return
  }

  if (typeof oldNode.children === 'string') {
    oldNode.el.innerHTML = ''
    newNode.children.forEach(child => {
      mount(child, oldNode.el)
    })
    return
  }

  // 数组节点
  let max = Math.max(newNode.children.length, oldNode.children.length)

  for(let i = 0; i < max; i++) {
    if (oldNode.children[i] && !newNode.children[i]) {
      unmount(oldNode.children[i])
    } else if (!oldNode.children[i] && newNode.children[i]) {
      mount(newNode.children[i], oldNode.el)
    } else {
      patch(oldNode.children[i], newNode.children[i])
    }
  }
}
```

1. 这边判断新旧tag或者props是否一致，如果不一致直接创建新的dom替换掉当前的dom
2. 如果存在文本节点修改文本节点
3. 数组节点这边遍历了数组节点直接进行对比，从而实现对dom的增删改


## reactive

reactive主要用于实现响应式数据，即数据改变自动触发视图刷新

新建`reactive.js`：

``` javascript
let observers = new Map()
let handler
let queue = new Set()
let flushing = false

function flushQueue() {
  if (queue.size > 0) {
    flushing = true
    new Promise((resolve) => {
      resolve()
    }).then(() => {
      queue.forEach(handler => {
        handler()
      })
      queue.clear()
    })
  }
}

export function reactive(obj) {
  return new Proxy(obj, {
    get(_obj, key) {
      observers[obj] = handler
      return obj[key]
    },
    set(_obj, key, value) {
      obj[key] = value
      queue.add(observers[obj])
      flushQueue()
      return true
    }
  })
}

export function effect(cb) {
  handler = cb
  cb()
}
```

通过`Proxy`劫持对象的set和get函数，get的时候加入依赖，handler是在effect被调用时触发的回掉函数，这边其实是组件的render

而queue的作用是，不让set立即执行回掉，在下一次微任务队列再执行，这样类似的数据修改被合并了，不用重复执行

修改`vue.js`：

``` javascript
import { mount } from "./mount.js"
import { h } from './h.js'
import { effect } from './reactive.js'
import { patch } from './patch.js'

function render(Component) {
  let newNode
  effect(() => {
    const oldNode = newNode
    newNode = Component.render(h)

    if (oldNode) {
      patch(oldNode, newNode)
    }
  })
  return newNode
}

export function createApp(App) {
  const app = render(App)
  return {
    mount: (container) => {
      mount(app, container)
      return app
    },
  }
}

```

这边它掉用`effect`函数去收集依赖并注册回掉，这样当数据变化时该回掉会自动触发

最后我们修改`index.html`来测试

``` javascript
import { createApp } from './vue.js'
import { reactive } from './reactive.js'

const data = reactive({
  hello: 'hello',
  world: 'world'
})

// ....

setTimeout(() => {
  data.hello = 'hello1'
  data.world = 'world1'
}, 1000)

```

这边将`data`改成响应式的这样就能自动更新视图了


## component

### h支持传组件

``` javascript
export function h(tag, props, children) {
  if (typeof tag === 'string') {
    return {
      tag,
      props,
      children
    }
  } else {
    const vnode = tag.render(h, props.props)
    vnode.$props = props.props
    vnode.$on = props.on
    return vnode
  }
}

```

这边简单认为tag是string是普通标签，非string就是组件


测试`test.js`:

``` javascript
// ...

const Button = {
  props: [ 'type' ],
  render(h, props) {
    let color = props && props.type === 'primary' ? 'blue' : 'red'
    return h('button', { style: `color: ${color}` }, 'click me')
  }
}

const App = {
  render(h) {
    return h('div', {}, [
      h('span', { style: 'color: red' }, data.hello),
      h('span', { style: 'color: green' }, data.world),
      h(Button, {
        props: {
          type: data.type
        },
        on: {
          click: () => {
            data.type = data.type === 'primary' ? '' : 'primary'
          }
        }
      }, 'click me')
    ])
  }
}
// ...
```


## 参考链接

- [coding-a-vue-js-like-framework-from-scratch](https://dev.to/themarcba/coding-a-vue-js-like-framework-from-scratch-part-1-introduction-3nbf)
-
