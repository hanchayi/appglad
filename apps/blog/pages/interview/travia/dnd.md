---
拖拽
---

# Html5拖拽

事件列表
- draggable
- dragstart
- darg （被拖动的元素触发）
- dragenter
- dragover
- dragleave
- drop
- dragend


## drag的一些坑

### dragstart

effectAllowed和dropEffect的彼此间是有制约关系，当我们给effectAllowed设置了对应的属性值，则dropEffect只能设置为effectAllowed允许的值，否则是无效的。

``` javascript
element.addEventListener(‘dragstart’, function (evt) {
  // We'll handle this event so first stop bubbling up
  evt.stopPropagation();
  // If we'd like to prevent dragging because of some condition,
  // we'd simply call preventDefault() and be done here
  if (some_condition) {
    evt.preventDefault();
    return;
  }
  // Now setup our dataTransfer object properly
  // First we'll allow a move action — this is used for the cursor
  evt.dataTransfer.effectAllowed = 'move';
  // Setup some dummy drag-data to ensure dragging
  evt.dataTransfer.setData('text/plain', 'some_dummy_data');
  // Now we'll create a dummy image for our dragImage
  var dragImage = document.createElement('div');
  dragImage.setAttribute('style', 'position: absolute; left: 0px; top: 0px; width: 40px; height: 40px; background: red; z-index: -1');
  document.body.appendChild(dragImage);
  // And finally we assign the dragImage and center it on cursor
  evt.dataTransfer.setDragImage(dragImage, 20, 20);
});
```

### dragover

dropEffect属性顾名思意拖拽效果，在PC web端主要表现在鼠标手形上。不同的dropEffect值，鼠标的手形效果是不一样的。

1. 仅能在 dragover 事件中设置该属性值，其他事件中设置均无效
2. 当显示禁止的指针样式时，将无法触发目标元素的 drop 事件。


### setDragImage
必须是实际显示到文档中的元素，不能直接创一个内存中的元素或是用透明度 定位到窗口外面解决。

解决方案：
将幽灵元素设置样式`z-index：-1`放到`body`内

``` javascript
 let dragImage = document.getElementById(GHOST_ID)

if (!dragImage) {
  dragImage = document.createElement('div');
  dragImage.setAttribute('id', GHOST_ID)
}

dragImage.innerHTML = `${label}${target_comp_id}`;
dragImage.setAttribute('style', 'position: absolute; left: 0px; top: 0px; width: auto; height: 24px; line-height: 24px; text-align: center;background: transparent; z-index: -1');
document.body.appendChild(dragImage);
event.dataTransfer.setDragImage(dragImage, -2, 12)
```


### 事件冒泡

所有drag事件最好都阻止冒泡。

由于所有drag事件都是可以冒泡的，可以用代理做性能优化。

拖进一个目标元素，元素的子元素也会触发拖拽事件，这边最好把子元素的鼠标事件屏蔽掉

解决方案：
1. 进入目标元素的时候设置子元素的`pointer-events: none`类名
2. 离开目标元素的时候移除类名

``` css
/* 这里为了样式优先级加了 id和important */
#gui__render_display_root .gui-drag :not([data-comp_id]) {
  pointer-events: none !important;
}
```

``` typescript
{
  dragenter: (event: DragEvent) => {
    e.stopPropagation();
    const target = e.target;
    target.classList.add('gui-drag')
  },
  dragleave: (event: DragEvent) => {
    e.stopPropagation();
    const target = e.target;
    target.classList.remove('gui-drag')
  },
}
```

## 参考
[dnd](https://medium.com/@reiberdatschi/common-pitfalls-with-html5-drag-n-drop-api-9f011a09ee6c)
[drag and drop](https://www.zhangxinxu.com/wordpress/2018/09/drag-drop-datatransfer-js/)
