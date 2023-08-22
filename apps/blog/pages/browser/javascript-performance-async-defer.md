## async defer

js会阻塞dom构建的过程

以前我们的做法是将js放到body最下面

现在可以用defer的方式让js提前下载

如果js不依赖其他js，且不操作dom的情况下可以使用async


## Reference

https://www.freecodecamp.org/chinese/news/javascript-performance-async-defer/
