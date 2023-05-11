---
description: 第一个插件
---

## 初始化

安装代码生成器
``` bash
npm install -g yo generator-code
```

执行生成代码
``` bash
yo code
```

执行一些配置
``` bash
? What type of extension do you want to create? New Code Snippets
Folder location that contains Text Mate (.tmSnippet) and Sublime snippets (.sublime-snippet) or press ENTER to start with a new snippet file.
? Folder name for import or none for new:
? What's the name of your extension? markdown-snippets
? What's the identifier of your extension? markdown-snippets
? What's the description of your extension? snippets for markdwon and mermaid
Enter the language for which the snippets should appear. The id is an identifier and is single, lower-case name such as 'php', 'javascript'
? Language id: md
? Initialize a git repository? Yes
```

## 运行

按`F5`按钮，将会编译和执行项目，并且启用一个用于开发插件的VSCode窗口。

修改代码后，按`Command(⌘) + Shift(⇧) + F5`来重载代码

## 发布

``` bash
npm install -g vsce
vsce package
```
