
## 加载优化
- 包体积
  - webpack-bundle-analyzer分析包体积
  - import()
  - cdn引入静态资源，异步加载第三方资源
  - 代码优化 treeshaking
- js
  - async defer
- 开启http2（listen       443 ssl http2;）
  - 使用多路复用技术，在同一个连接中可以并行处理多个请求。
  - 可以压缩HTTP头，减少请求的大小。
  - 数据传输格式是以二进制进行的，所以传输更加有效。
  - 服务器可以向客户端推送数据，从而让应用程序可以处理更加复杂的功能。
- cdn
- 压缩
  - 开启gzip
- 缓存
  - 开启缓存(nginx expires)
- 图片压缩
  - tinypng
  - webp
  - base64

## 渲染优化
- 减少dom节点
- 动画
  - 尽量使用css3
  - js动画合理使用requestAnimationFrame
  - gpu加速 transform

## 脚本优化
  - 避免全局变量
  - 减少重绘和回流
    - fragment
    - 缓存dom
  - 事件代理
  - debounce和throttle
