
# 从零开始实现一个WebSocket Server

这篇文章我们将从协议层面实现一个WebSocket的Server从而了解WebScoket协议的内在如何工作的。它将涵盖客户端服务端之间的握手和基本的数据帧解析和传递。

[RFC 6455](https://www.rfc-editor.org/rfc/rfc6455)（WebSocket官方协议规范）将作为我们主要的参考依据。

## 1. 背景

WebSocket是基于TCP/IP协议来交互的应用层协议，它为客户端和服务端之间提供了全双工双向的通道。

相对于HTTP请求响应的范式，它通过一个持久的连接提供了实时客户端-服务端的消息交换。因此特别适合那些实时交互要求高的应用。

本质上，WebSocket是TCP/IP协议栈上一层简单、低开销的封装。


## 原文链接

- [implementing-a-websocket-server-from-scratch](https://betterprogramming.pub/implementing-a-websocket-server-from-scratch-in-node-js-a1360e00a95f)
