## Promise
``` javascript
Promise.reject(2)
.catch(err=>console.log("err1,",err))
.then(res=>{console.log("then1",res)})
.catch(err=>console.log("err2,",err))

// err1 2
// then1 undefined
```
