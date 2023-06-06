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

1. 由于给定的数组是排序的，我们可以假设它是给定树的中序遍历的结果。
2. 在这种情况下，给定排序数组的中间值将表示可以从给定数组元素构造的一个可能的 BST 的根。
3. 为了与二叉搜索树的定义保持一致，数组中位于中间值左侧的元素将有助于我们根的左子树，而数组中位于中间值右侧的元素将有助于我们根的右子树。
4. 因此我们可以递归地构造出二叉搜索树，通过对数组使用二分搜索算法,通过递归调用具有适当边界条件的convertToBstHelper方法分别构造根子树、左子树和右子树，左子树的边界条件为low, mid -1，右子树的边界条件为mid+1, high。
5. 终止递归的基本条件是如果低边界索引超过高边界索引，在这种情况下返回 null。

## 测试：

1. 使用 null 或空值等健全输入进行测试。
2. 用负数和正数数组元素进行测试。
3. 构造BST后测试构造树的顺序是否返回数组元素。

## 解决方案：

``` java
/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
class Solution {
    public TreeNode sortedArrayToBST(int[] nums) {

       return helper(nums, 0, nums.length - 1);
    }

    private TreeNode helper(int[] nums, int low, int high) {
        if(low > high) {
            return null;
        }

        int mid = low + (high - low)/2;
        //center val of sorted array as the root of the bst
        TreeNode head = new TreeNode(nums[mid]);

        //left of the mid value should go to the left of this root node to satisfy bst
        head.left = helper(nums, low, mid - 1);
        //right of the mid value should go to the right of this root node to satisfy bst
        head.right = helper(nums, mid + 1, high);
        return head;

    }

    //T O(log n) S O(n) recursion stack space
}
```

## 复杂性分析：

由于我们对数组元素执行二进制搜索，通过每次递归将输入大小分成一半，因此，上述算法产生的时间复杂度与二进制搜索中产生的时间复杂度相同，即 T O(log n)。在 S O(n) 的最坏情况下，递归堆栈空间会导致空间复杂度。

## 译文

- [Convert Sorted Array to BST](https://medium.com/@harycane/convert-sorted-array-to-bst-35781e940ca5)
