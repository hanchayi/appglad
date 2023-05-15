## Vue响应式原理


```
settter

->
Dep.notify()

->
watcher.update

->
get()重新生成以来
patch对比VNode


->
生成新的dom

```

## Vue的diff过程

``` javascript
function updateChildren(parentElm, oldCh, newCh) {
    var oldStartIdx = 0;
    var oldEndIdx = oldCh.length - 1;
    var oldStartVnode = oldCh[0];
    var oldEndVnode = oldCh[oldEndIdx];
    var newStartIdx = 0;
    var newEndIdx = newCh.length - 1;
    var newStartVnode = newCh[0];
    var newEndVnode = newCh[newEndIdx];
    var oldKeyToIdx, idxInOld, vnodeToMove, refElm;

    // 不断地更新 OldIndex 和 OldVnode ，newIndex 和 newVnode
    while (
      oldStartIdx <= oldEndIdx &&
      newStartIdx <= newEndIdx
    ) {
        if (!oldStartVnode) {
          oldStartVnode = oldCh[++oldStartIdx];
        }
        else if (!oldEndVnode) {
          oldEndVnode = oldCh[--oldEndIdx];
        }
        //  旧头 和新头 比较
        else if (sameVnode(oldStartVnode, newStartVnode)) {
            patchVnode(oldStartVnode, newStartVnode);
            oldStartVnode = oldCh[++oldStartIdx];
            newStartVnode = newCh[++newStartIdx];
        }
        //  旧尾 和新尾 比较
        else if (sameVnode(oldEndVnode, newEndVnode)) {
            patchVnode(oldEndVnode, newEndVnode);
            oldEndVnode = oldCh[--oldEndIdx];
            newEndVnode = newCh[--newEndIdx];
        }

        // 旧头 和 新尾 比较
        else if (sameVnode(oldStartVnode, newEndVnode)) {
            patchVnode(oldStartVnode, newEndVnode);
            // oldStartVnode 放到 oldEndVnode 后面，还要找到 oldEndValue 后面的节点
            parentElm.insertBefore(
                oldStartVnode.elm,
                oldEndVnode.elm.nextSibling
            );
            oldStartVnode = oldCh[++oldStartIdx];
            newEndVnode = newCh[--newEndIdx];
        }
        //  旧尾 和新头 比较
        else if (sameVnode(oldEndVnode, newStartVnode)) {
            patchVnode(oldEndVnode, newStartVnode);
            // oldEndVnode 放到 oldStartVnode 前面
            parentElm.insertBefore(oldEndVnode.elm, oldStartVnode.elm);
            oldEndVnode = oldCh[--oldEndIdx];
            newStartVnode = newCh[++newStartIdx];
        }

        // 单个新子节点 在 旧子节点数组中 查找位置
        else {
            // oldKeyToIdx 是一个 把 Vnode 的 key 和 index 转换的 map
            if (!oldKeyToIdx) {
                oldKeyToIdx = createKeyToOldIdx(
                    oldCh, oldStartIdx, oldEndIdx
                );
            }
            // 使用 newStartVnode 去 OldMap 中寻找 相同节点，默认key存在
            idxInOld = oldKeyToIdx[newStartVnode.key]
            //  新孩子中，存在一个新节点，老节点中没有，需要新建
            if (!idxInOld) {
                //  把  newStartVnode 插入 oldStartVnode 的前面
                createElm(
                    newStartVnode,
                    parentElm,
                    oldStartVnode.elm
                );
            }
            else {
                //  找到 oldCh 中 和 newStartVnode 一样的节点
                vnodeToMove = oldCh[idxInOld];
                if (sameVnode(vnodeToMove, newStartVnode)) {
                    patchVnode(vnodeToMove, newStartVnode);
                    // 删除这个 index
                    oldCh[idxInOld] = undefined;
                    // 把 vnodeToMove 移动到  oldStartVnode 前面
                    parentElm.insertBefore(
                        vnodeToMove.elm,
                        oldStartVnode.elm
                    );
                }
                // 只能创建一个新节点插入到 parentElm 的子节点中
                else {
                    // same key but different element. treat as new element
                    createElm(
                        newStartVnode,
                        parentElm,
                        oldStartVnode.elm
                    );
                }
            }

            // 这个新子节点更新完毕，更新 newStartIdx，开始比较下一个
            newStartVnode = newCh[++newStartIdx];
        }
    }

    // 处理剩下的节点
    if (oldStartIdx > oldEndIdx) {
        var newEnd = newCh[newEndIdx + 1]
        refElm = newEnd ? newEnd.elm :null;
        for (; newStartIdx <= newEndIdx; ++newStartIdx) {
            createElm(
               newCh[newStartIdx], parentElm, refElm
            );
        }
    }

    // 说明新节点比对完了，老节点可能还有，需要删除剩余的老节点
    else if (newStartIdx > newEndIdx) {

        for (; oldStartIdx<=oldEndIdx; ++oldStartIdx) {

            oldCh[oldStartIdx].parentNode.removeChild(el);
        }
    }
}
```

## 谈谈你对 keep-alive 的了解？


``` vue
<keep-alive include=""></keep-alive>
```

一般结合路由和动态组件一起使用，用于缓存组件；
提供 include 和 exclude 属性，两者都支持字符串或正则表达式， include 表示只有名称匹配的组件会被缓存，exclude 表示任何名称匹配的组件都不会被缓存 ，其中 exclude 的优先级比 include 高；
对应两个钩子函数 activated 和 deactivated ，当组件被激活时，触发钩子函数 activated，当组件被移除时，触发钩子函数 deactivated。


``` javascript
// 源码位置：src/core/components/keep-alive.js
export default {
  name: 'keep-alive',
  // 不在实际的vnode中
  abstract: true,
  props: {
    include: patternTypes,
    exclude: patternTypes,
    max: [String, Number]
  },
  created () {
    this.cache = Object.create(null)
    this.keys = []
  },
  destroyed () {
    for (const key in this.cache) {
      pruneCacheEntry(this.cache, key, this.keys)
    }
  },
  mounted () {
    this.$watch('include', val => {
      pruneCache(this, name => matches(val, name))
    })
    this.$watch('exclude', val => {
      pruneCache(this, name => !matches(val, name))
    })
  },
  render () {
    const slot = this.$slots.default
    const vnode: VNode = getFirstComponentChild(slot)
    const componentOptions: ?VNodeComponentOptions = vnode && vnode.componentOptions
    if (componentOptions) {
      // check pattern
      const name: ?string = getComponentName(componentOptions)
      const { include, exclude } = this
      if (
        // not included
        (include && (!name || !matches(include, name))) ||
        // excluded
        (exclude && name && matches(exclude, name))
      ) {
        return vnode
      }

      const { cache, keys } = this
      const key: ?string = vnode.key == null
        // same constructor may get registered as different local components
        // so cid alone is not enough (#3269)
        ? componentOptions.Ctor.cid + (componentOptions.tag ? `::${componentOptions.tag}` : '')
        : vnode.key
      if (cache[key]) {
        vnode.componentInstance = cache[key].componentInstance
        // make current key freshest
        remove(keys, key)
        keys.push(key)
      } else {
        cache[key] = vnode
        keys.push(key)
        // prune oldest entry
        if (this.max && keys.length > parseInt(this.max)) {
          pruneCacheEntry(cache, keys[0], keys, this._vnode)
        }
      }
      vnode.data.keepAlive = true
    }
    return vnode || (slot && slot[0])
  }
}

```

abstract
``` javascript
// 源码位置： src/core/instance/lifecycle.js
export function initLifecycle (vm: Component) {
  const options = vm.$options

  // locate first non-abstract parent
  let parent = options.parent
  if (parent && !options.abstract) {
    while (parent.$options.abstract && parent.$parent) {
      parent = parent.$parent
    }
    parent.$children.push(vm)
  }
  vm.$parent = parent
  // ...
}
```

## patch

根据VNode关系生成真实的dom节点

``` javascript
// 源码位置：src/core/vdom/patch.js
function createComponent (vnode, insertedVnodeQueue, parentElm, refElm) {
  let i = vnode.data
  if (isDef(i)) {
    const isReactivated = isDef(vnode.componentInstance) && i.keepAlive
    if (isDef(i = i.hook) && isDef(i = i.init)) {
      i(vnode, false /* hydrating */)
    }
    // after calling the init hook, if the vnode is a child component
    // it should've created a child instance and mounted it. the child
    // component also has set the placeholder vnode's elm.
    // in that case we can just return the element and be done.
    if (isDef(vnode.componentInstance)) {
      initComponent(vnode, insertedVnodeQueue)
      insert(parentElm, vnode.elm, refElm)
      if (isTrue(isReactivated)) {
        reactivateComponent(vnode, insertedVnodeQueue, parentElm, refElm)
      }
      return true
    }
  }
}
```

