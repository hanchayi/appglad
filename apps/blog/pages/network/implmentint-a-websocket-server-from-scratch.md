
# 从零开始实现一个WebSocket Server

这篇文章我们将从协议层面实现一个WebSocket的Server从而了解WebScoket协议的内在如何工作的。它将涵盖客户端服务端之间的握手和基本的数据帧解析和传递。

[RFC 6455](https://www.rfc-editor.org/rfc/rfc6455)（WebSocket官方协议规范）将作为我们主要的参考依据。

## 1. 背景

`WebSocket`是基于`TCP/IP`协议来交互的应用层协议，它为客户端和服务端之间提供了全双工双向的通道。

相对于`HTTP`请求响应的范式，它通过一个持久的连接提供了实时客户端-服务端的消息交换。因此特别适合那些实时交互要求高的应用。

本质上，`WebSocket`是`TCP/IP`协议栈上一层简单、低开销的封装。

## 2. 协议概览

WebSocket协议在一个持久的连接上运行，但是它是通过一个HTTP请求Upgrade建立的，你通过`Network`面板调试WebSocket时可以看到`101 Switch Protocals`的http请求。

基本上，为了使用WebSocket协议我们先需要开启一个`HTTP Server`, 然后通过客户端发送一个`Upgrade Request`来将`HTTP`协议切换到`WebSocket`协议。

让我们创建一个`ws.js`的文件，来开始实现我们的`WebSocket Server`:

```javascript
const http = require('node:http');
const { EventEmitter } = require('node:events');

class WebSocketServer extends EventEmitter {
  constructor(options = {}) {
    super();
    this.port = options.port || 4000;
    this._init();
  }

  _init() {
    if (this._server) throw new Error('Server already initialized');

    this._server = http.createServer((req, res) => {
      const UPGRADE_REQUIRED = 426;
      const body = http.STATUS_CODES[UPGRADE_REQUIRED];
      res.writeHead(UPGRADE_REQUIRED, {
        'Content-Type': 'text/plain',
        'Upgrade': 'WebSocket',
      });
      res.end(body);
    });
  }
}
```

我们继承了`EventEmitter`类以便后续可以使用`emit`抛出事件，或者`on`监听事件。

`_init`函数创建了`HTTP Server`实例，这边所有非`Upgrade`的请求，全部返回`UPGRADE_REQUIRED`状态码

## 3. 握手连接

`WebSocket`通过客户端初始化一个`opening handshake`连接，服务端需要返回特定的`header`来完成捂手。然后这个`HTTP`连接会被`WebSocket`连接替换，并且使用同一个`TCP`连接。

客户端通过发送以下`header`的`GET`请求发起握手：

```
GET /HTTP/1.1
Connection: Upgrade
Upgrade: websocket
Sec-WebSocket-Key: kB2x1cO5zjL1ynwrLTSXUQ==
Sec-WebSocket-Version: 13
```

`Connection`和`Upgrade`请求头告诉服务端这是一个要建立`WebSocket`的连接。

`Sec-WebSocket-Version`是`WebSocket`的版本目前固定13。

`Sec-WebSocket-Key`是客户端随机生成的字符串，根据`RFC`规定，这个`header`是一个16位的随机数，并且被base64编码过。这个随机数必须保证每个连接不一样。

这个`Sec-WebSocket-Key`在服务端创建握手的时候被使用，用以标志接受了该连接。

为了接受进来的连接，服务端必须返回`101 Switching Protocols`状态，同时必须包含`Sec-WebSocket-Accept`的`header`。

`Sec-WebSocket-Accept`这样产生

- 服务端获取`Sec-WebSocket-Key`的值和一个固定的GUID字符串（258EDFA5-E914–47DA-95CA-C5AB0DC85B11）连接
- 执行`SHA-1`哈希
- 执行`base64`编码








## 原文链接

- [implementing-a-websocket-server-from-scratch](https://betterprogramming.pub/implementing-a-websocket-server-from-scratch-in-node-js-a1360e00a95f)
