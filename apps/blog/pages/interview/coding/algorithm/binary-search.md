## 二分查找

```typescript
function binarySearch(items, target) {
  let left = 0;
  let right = items.length - 1;

  while (left <= right) {
    // 防止数字越界
    let middle = left + Math.floor((right - left) / 2)
    console.log(left, right, middle, items[middle])
    if (items[middle] < target ) {
      left = middle + 1 // 考虑left=right，则middle=left=right跳不出循环
    } else if (items[middle] > target) {
      right = middle - 1
    } else {
      return middle
    }
  }

  return -1
}
console.log(binarySearch([1, 2, 3, 4], 4))
console.log(binarySearch([1, 2, 3, 4, 5], 4))
console.log(binarySearch([1, 2, 3, 4, 5], 6))
```
