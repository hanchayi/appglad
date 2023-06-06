---
title: 将已经排序的数组转成BST
---

[leetcode](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/description/)

给定一个元素按升序排序的数组，将其转换为高度平衡的 BST。

对于这个问题，高度平衡的二叉树被定义为一个二叉树，其中每个节点的两个子树的深度从不相差超过 1。

## 例子：

```
给定已排序的数组: [-10,-3,0,5,9],
结果如下: [0,-3,9,-10,null,5], 代表如下的平衡二叉树
      0
     / \
   -3   9
   /   /
 -10  5
```

## 算法：

1 由于给定的数组是排序的，我们可以假设它是给定树的中序遍历的结果。


## 译文

- [Convert Sorted Array to BST](https://medium.com/@harycane/convert-sorted-array-to-bst-35781e940ca5)
