## 使用

``` vue
<keep-alive
  include=""
  exclude=""
  max=""
>
  <ComponentA />
</keep-alive>
```
``` vue
<template>
 <div>ComponentA</div>
</template>

<script>
export default {
  activated() {}
  deactivated() {}
}
</script>
```

## 原理
component
``` javascript
export default {
  abstract: true,
  props: ["include", "exclude", "max"]
  created() {
    this.cache = Object.create(null)
  },
  render() {
    const vnode = this.$slots.default
    const name = vnode.options.name
    if (!this.cache[name]) {
      this.cache[name] = vnode
    }
    return this.cache[name]
  }
}
```

init
``` javascript
function initLifecycle() {
  if (vnode.abstract) {
    return vnode.parent
  }
}
```

patch
``` javascript
function createComponent() {
  if (vnode.componentInstance && vnode.keepalive) {
    insert(parent, vnode.el)
  }
}
```

