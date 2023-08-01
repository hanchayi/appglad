
# 从零开始实现一个WebSocket Server

这篇文章我们将从协议层面实现一个WebSocket的Server从而了解WebScoket协议的内在如何工作的。它将涵盖客户端服务端之间的握手和基本的数据帧解析和传递。

[RFC 6455](https://www.rfc-editor.org/rfc/rfc6455)（WebSocket官方协议规范）将作为我们主要的参考依据。

源代码链接:[websocket-server](https://github.com/hanchayi/websocket-server)

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

`WebSocket`通过客户端初始化一个`opening handshake`连接，服务端需要返回特定的`header`来完成捂手。然后这个`HTTP`连接会被`WebSocket`连接替换，并且在之后复用当前这个`TCP`连接。

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

基本上，为了确保客户端和服务端支持`WebSocket`协议，这是必须的。如果服务端接受`WebSocket`接受`HTTP`连接，但将数据解释成`HTTP`格式，可能会有潜在的安全问题。

握手的响应头也需要包含`Connection: Upgrade`和`Upgrade: websocket`

当客户端收到服务端的响应后，一个`WebSocket`的连接就建立完成，并等待传输数据

修改我们的`_init`函数

``` javascript
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

  // connection = upgrade 触发
  this._server.on('upgrade', (req, socket) => {
    this.emit('headers', req);

    // 验证请求头upgrade是不是websocket 不是返回400错误
    if (req.headers.upgrade !== 'websocket') {
      socket.end('HTTP/1.1 400 Bad Request');
      return;
    }

    const acceptKey = req.headers['sec-websocket-key'];
    const acceptValue = this._generateAcceptValue(acceptKey);

    const responseHeaders = [
      'HTTP/1.1 101 Switching Protocols',
      'Upgrade: websocket',
      'Connection: Upgrade',
      `Sec-WebSocket-Accept: ${acceptValue}`,
    ];

    socket.write(responseHeaders.concat('\r\n').join('\r\n'));
  });

  // 生成Accept的header值
   _generateAcceptValue(key) {
    // 固定的字符串 和规范中提到的要一致
    const GUID = '258EAFA5-E914-47DA-95CA-C5AB0DC85B11'
    const { createHash } = require('crypto');
    const digest = createHash('sha1')
    .update(key + GUID)
    .digest('base64');
    return digest
  }
}
```


## 4. 接受数据

首先我们需要了解协议的数据帧组成

帧数据

[ `Fin`, `RSV1`, `RSV2`, `RSV3`, `opcode` ]

[ `Mask`, `Payload length`, `Extended payload length` ]

[ `Payload Data` ]


含义

- 第一个字节 `Fin`(1bit) + `RSV1`(1bit) + `RSV2`(1bit) + `RSV3`(1bit) + `opcode`(4bit)
- 第二个字节 `Mask`(1bit) + `Payload length`(7bit)
- `Extended payload length`(根据Payload length的值确定实际长度)
- `Mask key`(如果`Mask`=1才存在)
- `Payload Data`(根据`Extended payload length`确定长度)


### Fin
如果设置了，标识是消息的最后一帧

### RSV
插件标志，本文忽略

### opcode
该条消息对应的操作

| opcode     | 描述 |
| ----------- | ----------- |
| 0x00      | 标识这个接着上一帧      |
| 0x01   | 文本帧        |
| 0x02   | 二进制帧        |
| 0x08   | 关闭连接        |
| 0x09，0x0a   | ping pong心跳        |

### Mask， Mask key

是否使用掩码，默认客户端->服务端mask为1，服务端到客户端mask为0

`Mask key`用于对数据做编码的字符串，只有mask等于1时才生效

### Payload length
确定`Extended payload length`长度

- 如果`Payload`的数据长度在0 ~ 125之间， 则 `Payload length`等于`Payload`的实际长度
- 如果`Payload`的数据长度在126 ~ 65535之间， 则 `Payload length`等于126，`Extended payload length`等于`Payload`的长度，占两个字节
- 如果`Payload`的数据长度在65535 ~ 9223372036.85 G之间， 则 `Payload length`等于127，`Extended payload length`等于`Payload`的长度，占8个字节

### Payload Data
实际传输的数据


### 帧解析
根据以上协议定义，我们可以实现帧的解析函数
``` javascript
parseFrame(buffer) {
  // 第一个字节
  const firstByte = buffer.readUInt8(0);
  // 第一个字节后四位 opcode
  const opCode = firstByte & 0b00001111;

  if (opCode === this.OPCODES.close) {
    this.emit('close');
    return null;
  } else if (opCode !== this.OPCODES.text) {
    return;
  }



  // 第二个字节
  const secondByte = buffer.readUInt8(1);

  let offset = 2;
  // 获取payload的长度，即第二个字节后七位
  let payloadLength = secondByte & 0b01111111;

  if (payloadLength === 126) {
    offset += 2; // extended payload占两个字节
  } else if (payloadLength === 127) {
    offset += 8; // extended payload占四个字节
  }


  // 是否mask
  const isMasked = Boolean((secondByte >>> 7) & 0b00000001);

  if (isMasked) {
    // 读取四个字节的mask key
    const maskingKey = buffer.readUInt32BE(offset);
    offset += 4;
    const payload = buffer.subarray(offset);
    // 解码后返回数据
    const result = this._unmask(payload, maskingKey);
    return result.toString('utf-8');
  }

  // 直接返回数据
  return buffer.subarray(offset).toString('utf-8');
}

_unmask(payload, maskingKey) {
  const result = Buffer.alloc(payload.byteLength);

  for (let i = 0; i < payload.byteLength; ++i) {
    const j = i % 4;
    const maskingKeyByteShift = j === 3 ? 0 : (3 - j) << 3;
    const maskingKeyByte = (maskingKeyByteShift === 0 ? maskingKey : maskingKey >>> maskingKeyByteShift) & 0b11111111;
    const transformedByte = maskingKeyByte ^ payload.readUInt8(i);
    result.writeUInt8(transformedByte, i);
  }

  return result;
}


```

server监听`data`和`close`事件
``` javascript
this._server.on('upgrade', (req, socket) => {
  // ...upgrade request code...
  socket.on('data', (buffer) =>
    this.emit('data', this.parseFrame(buffer))
  );

  this.on('close', () => {
    console.log('closing....', socket);
    socket.destroy();
  });
});
```

这样我们的server就可以接受来自客户端的数据了

## 发送数据

知道了协议格式发送数据更简单因为不要对数据进行mask操作

我们这边写死发送`text`帧

``` javascript
createFrame(data) {
  const payload = JSON.stringify(data);

  const payloadByteLength = Buffer.byteLength(payload);
  let payloadBytesOffset = 2;
  let payloadLength = payloadByteLength;

  if (payloadByteLength > 65535) { // length value cannot fit in 2 bytes
    payloadBytesOffset += 8;
    payloadLength = 127;
  } else if (payloadByteLength > 125) {
    payloadBytesOffset += 2;
    payloadLength = 126;
  }

  const buffer = Buffer.alloc(payloadBytesOffset + payloadByteLength);

  // first byte
  buffer.writeUInt8(0b10000001, 0); // [FIN (1), RSV1 (0), RSV2 (0), RSV3 (0), Opсode (0x01 - text frame)]

  buffer[1] = payloadLength; // second byte - actual payload size (if <= 125 bytes) or 126, or 127

  if (payloadLength === 126) { // write actual payload length as a 16-bit unsigned integer
    buffer.writeUInt16BE(payloadByteLength, 2);
  } else if (payloadByteLength === 127) { // write actual payload length as a 64-bit unsigned integer
    buffer.writeBigUInt64BE(BigInt(payloadByteLength), 2);
  }

  buffer.write(payload, payloadBytesOffset);
  return buffer;
}
```


## 原文链接

- [implementing-a-websocket-server-from-scratch](https://betterprogramming.pub/implementing-a-websocket-server-from-scratch-in-node-js-a1360e00a95f)
