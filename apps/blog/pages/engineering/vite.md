---
title: vite
description: page desc
---

``` bash
pnpm create vite --template vue-ts
```

## How does create vite work?

If you run the command `npm create vite`, the `npm` will download the `create-vite` package and run the command in `bin`.

Here's the `package.json` in [create-vite](https://github.com/vitejs/vite/blob/main/packages/create-vite/package.json)

``` package.json
{
  "name": "create-vite",
  "version": "4.3.2",
  "type": "module",
  "license": "MIT",
  "author": "Evan You",
  "bin": {
    "create-vite": "index.js",
    "cva": "index.js"
  },
  "engines": {
    "node": "^14.18.0 || >=16.0.0"
  },
}
```
