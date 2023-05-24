
## dragon计算规则

拖拽的过程中

```mermaid
flowchart TD
    A[确定父级容器] --> B{是否有children?}
    B -- Yes --> C[遍历孩子]
    C --> D[需找最邻近的元素]
    D --> F[]
    B -- No ----> E[直接加入到父级容器的Children]
```



## locode engine 源码分析

``` typescript
// packages/designer/src/builtin-simulator/host.ts

export class BuiltinSimulatorHost implements ISimulatorHost<BuiltinSimulatorProps> {
  // ========= drag location logic: helper for locate ==========
  locate(e: ILocateEvent): any {
    const { dragObject } = e;

    const nodes = dragObject?.nodes;

    const operationalNodes = nodes?.filter((node) => {
      const onMoveHook = node.componentMeta?.advanced.callbacks?.onMoveHook;
      const canMove = onMoveHook && typeof onMoveHook === 'function' ? onMoveHook(node.internalToShellNode()) : true;

      let parentContainerNode: INode | null = null;
      let parentNode = node.parent;

      while (parentNode) {
        if (parentNode.isContainer()) {
          parentContainerNode = parentNode;
          break;
        }

        parentNode = parentNode.parent;
      }

      const onChildMoveHook = parentContainerNode?.componentMeta?.advanced.callbacks?.onChildMoveHook;

      const childrenCanMove = onChildMoveHook && parentContainerNode && typeof onChildMoveHook === 'function' ? onChildMoveHook(node.internalToShellNode(), parentContainerNode.internalToShellNode()) : true;

      return canMove && childrenCanMove;
    });

    if (nodes && (!operationalNodes || operationalNodes.length === 0)) {
      return;
    }

    this.sensing = true;
    this.scroller.scrolling(e);
    const document = this.project.currentDocument;
    if (!document) {
      return null;
    }

    // 查找合适的投放容器
    const dropContainer = this.getDropContainer(e);

    const lockedNode = getClosestNode(dropContainer?.container, (node) => node.isLocked);
    if (lockedNode) return null;
    if (
      !dropContainer
    ) {
      return null;
    }

    if (isLocationData(dropContainer)) {
      return this.designer.createLocation(dropContainer);
    }

    const { container, instance: containerInstance } = dropContainer;

    const edge = this.computeComponentInstanceRect(
      containerInstance,
      container.componentMeta.rootSelector,
    );

    if (!edge) {
      return null;
    }

    const { children } = container;

    const detail: IPublicTypeLocationChildrenDetail = {
      type: IPublicTypeLocationDetailType.Children,
      index: 0,
      edge,
    };

    const locationData = {
      target: container,
      detail,
      source: `simulator${document.id}`,
      event: e,
    };

    if (
      e.dragObject &&
      e.dragObject.nodes &&
      e.dragObject.nodes.length &&
      e.dragObject.nodes[0].componentMeta.isModal &&
      document.focusNode
    ) {
      return this.designer.createLocation({
        target: document.focusNode,
        detail,
        source: `simulator${document.id}`,
        event: e,
      });
    }

    if (!children || children.size < 1 || !edge) {
      return this.designer.createLocation(locationData);
    }

    let nearRect: IPublicTypeRect | null = null;
    let nearIndex: number = 0;
    let nearNode: INode | null = null;
    let nearDistance: number | null = null;
    let minTop: number | null = null;
    let maxBottom: number | null = null;

    for (let i = 0, l = children.size; i < l; i++) {
      const node = children.get(i)!;
      const index = i;
      const instances = this.getComponentInstances(node);
      const inst = instances
        ? instances.length > 1
          ? instances.find(
            (_inst) => this.getClosestNodeInstance(_inst, container.id)?.instance === containerInstance,
          )
          : instances[0]
        : null;
      const rect = inst
        ? this.computeComponentInstanceRect(inst, node.componentMeta.rootSelector)
        : null;

      if (!rect) {
        continue;
      }

      const distance = isPointInRect(e as any, rect) ? 0 : distanceToRect(e as any, rect);

      if (distance === 0) {
        nearDistance = distance;
        nearNode = node;
        nearIndex = index;
        nearRect = rect;
        break;
      }

      // 标记子节点最顶
      if (minTop === null || rect.top < minTop) {
        minTop = rect.top;
      }
      // 标记子节点最底
      if (maxBottom === null || rect.bottom > maxBottom) {
        maxBottom = rect.bottom;
      }

      if (nearDistance === null || distance < nearDistance) {
        nearDistance = distance;
        nearNode = node;
        nearIndex = index;
        nearRect = rect;
      }
    }

    detail.index = nearIndex;

    if (nearNode && nearRect) {
      const el = getRectTarget(nearRect);
      const inline = el ? isChildInline(el) : false;
      // 判断容器方向
      const row = el ? isRowContainer(el.parentElement!) : false;
      const vertical = inline || row;

      // TODO: fix type
      const near: {
        node: IPublicModelNode;
        pos: 'before' | 'after' | 'replace';
        rect?: IPublicTypeRect;
        align?: 'V' | 'H';
      } = {
        node: nearNode.internalToShellNode()!,
        pos: 'before',
        align: vertical ? 'V' : 'H',
      };
      detail.near = near;
      if (isNearAfter(e as any, nearRect, vertical)) {
        near.pos = 'after';
        detail.index = nearIndex + 1;
      }
      if (!row && nearDistance !== 0) {
        const edgeDistance = distanceToEdge(e as any, edge);
        if (edgeDistance.distance < nearDistance!) {
          const { nearAfter } = edgeDistance;
          if (minTop == null) {
            minTop = edge.top;
          }
          if (maxBottom == null) {
            maxBottom = edge.bottom;
          }
          near.rect = new DOMRect(edge.left, minTop, edge.width, maxBottom - minTop);
          near.align = 'H';
          near.pos = nearAfter ? 'after' : 'before';
          detail.index = nearAfter ? children.size : 0;
        }
      }
    }

    return this.designer.createLocation(locationData);
  }
}

```
