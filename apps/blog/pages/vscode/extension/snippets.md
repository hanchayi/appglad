
## 语法

``` json
{
  "For Loop": {
    "prefix": ["for", "for-const"],
    "body": ["for (const ${2:element} of ${1:array}) {", "\t$0", "}"],
    "description": "A for loop."
  }
}
```

- "For Loop": 代码片段名字
- "prefix": 触发的关键字，支持一个或多个
- "body": 一行或者多行代码


## 贡献点
```
"contributes": {
  "snippets": [
    {
      "language": "javascript",
      "path": "./snippets/jsComponent.code-snippets"
    },
    {
      "language": "typescript",
      "path": "./snippets/tsComponent.code-snippets"
    }
  ]
}
```


## 参考
- [VSCode用户自定义代码片段](https://code.visualstudio.com/docs/editor/userdefinedsnippets)
