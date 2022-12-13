<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
let p: Promise<number> = Promise.resolve(0)

function f(i: number): Promise<number> {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve(i*i)
    }, 1000)
  })
}

function test(i: number) {
  let cur = 0
  function _test() {
    if (cur >= i) {
      return
    }
    cur++
    p = p.then((res) => {
      console.log(res)
      return f(cur)
    })
    _test()
  }
  _test()
}
test(3)
</script>

<template>
  <div>
    <div style="position: fixed; top: 0; left: 0; right: 0;height: 48px">
      <router-link to="/"> home</router-link>
      <router-link to="/about">about</router-link>
    </div>
    <div style="position: fixed; top: 48px; left: 0; bottom: 0;width: 48px">left</div>
    <div style="position: fixed; top: 48px; left: 48px; right: 48px;bottom: 0;">
        <div style="display: flex;margin: 0 -24px;">
          <div v-for="i in 4" style="width: 25%;text-align: center;padding:0 24px;">
            <div style="width: 100%;background-color: blue;" :key="i">{{ i }}</div>
          </div>
        </div>

        <div style="display: flex;width: 200px;">
          <div style="flex: 1 1 25px;"></div>
          <div style="flex: 1 1 83px;"></div>
          <div style="flex: 1 1 163px;"></div>
        </div>
     
      <!-- <router-view /> -->
      <!-- <Canvas /> -->
    </div>
    <div style="position: fixed; top: 48px; right: 0; bottom: 0;width: 48px">right</div>
  </div>
</template>


