<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Canvas from './components/Canvas.vue'
</script>

<template>
  <div>
    <div style="position: fixed; top: 0; left: 0; right: 0;height: 48px">topbar</div>
    <div style="position: fixed; top: 48px; left: 0; bottom: 0;width: 48px">left</div>
    <div style="position: fixed; top: 48px; left: 48px; right: 48px;bottom: 0;">
      <Canvas />
    </div>
    <div style="position: fixed; top: 48px; right: 0; bottom: 0;width: 48px">right</div>
  </div>
</template>


