<script setup lang="ts">
import { ref, onMounted } from 'vue'

function addEventListeners() {
  window.addEventListener('resize', () => {
    resize()
  })
}

function resize() {
  const canvasEl = canvas.value;
  
  if (!canvasEl) {
    return
  }

  const offsetWidth = canvasEl.offsetWidth;
  const offsetHeight = canvasEl.offsetHeight;

  var realToCSSPixels = window.devicePixelRatio;
  // 获取浏览器显示的画布的CSS像素值
  // 然后计算出设备像素设置drawingbuffer
  var displayWidth  = Math.floor(offsetWidth  * realToCSSPixels);
  var displayHeight = Math.floor(offsetHeight * realToCSSPixels);

  if (canvasEl.width != displayWidth || canvasEl.height != displayHeight) {
    canvasEl.width = displayWidth;
    canvasEl.height = displayHeight;
  }
  
  
}

function initShaderProgram(gl: WebGLRenderingContext, vsSource: string, fsSource: string) {
  const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource);
  const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource);

  const program = gl.createProgram();

  if (!program) {
    throw new Error('create program error');
  }
  gl.attachShader(program, vertexShader);
  gl.attachShader(program, fragmentShader);
  gl.linkProgram(program);

  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    console.log('Unable to initialize the shader program: ' + gl.getProgramInfoLog(program));
    throw new Error('create program error')
  }

  return program;
}

function loadShader(gl: WebGLRenderingContext, type: number, source: string) {
  const shader = gl.createShader(type);

  if (!shader) {
    throw new Error('invalid shader ' + type)
  }

  gl.shaderSource(shader, source);
  gl.compileShader(shader);

  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    console.log('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader));
    gl.deleteShader(shader);
    throw new Error('create shader error')
  }

  return shader;
}

function initBuffers(gl: WebGLRenderingContext) {
  const positionBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
  const positions = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0];
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);
}

function drawScene(gl: WebGLRenderingContext, program: WebGLProgram) {
  // gl.viewport告诉WebGL如何将裁剪空间（-1 到 +1）中的点转换到像素空间
  gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
  gl.clearColor(0.0, 0.0, 0.0, 1.0); // Clear to black, fully opaque
  gl.clearDepth(1.0); // Clear everything
  gl.enable(gl.DEPTH_TEST); // Enable depth testing
  gl.depthFunc(gl.LEQUAL); // Near things obscure far things

  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
  gl.useProgram(program);

  const position = gl.getAttribLocation(program, 'a_position');
  gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 0, 0);
  gl.enableVertexAttribArray(position);

  const time = gl.getUniformLocation(program, 'u_time');
  const timestamp = performance.now()
  gl.uniform1f(time, timestamp);
  gl.drawArrays(gl.TRIANGLES, 0, 6);
}

function draw() {
  if (!gl) {
    throw new Error("not support gl");
  }

  const vsSource = `
  precision mediump float;

  attribute vec2 a_position;

  void main() {
      gl_Position = vec4(a_position, 0.0, 1.0);
  }
  `

  const fsSource = `
  precision mediump float;

  uniform float u_time;

  void main() {
      float r = sin(u_time * 0.0003);
      float g = sin(u_time * 0.0005);
      float b = sin(u_time * 0.0007);

      gl_FragColor = vec4(r, g, b, 1.0);
  }
  `;

  const program = initShaderProgram(gl, vsSource, fsSource);

  initBuffers(gl);
  drawScene(gl, program);
  requestAnimationFrame(() => {
    draw()
  })
}


let gl: WebGLRenderingContext | null;
let canvas = ref<HTMLCanvasElement | null>(null);
let container = ref<HTMLDivElement | null>(null);
onMounted(() => {
  const el = canvas.value;

  if (!el) {
    throw new Error('el not found')
  }
  console.log('canvas', el)
  gl = el.getContext('webgl');

  addEventListeners()
  resize()

  draw();
})
</script>

<template>
  <canvas ref='canvas' style="width: 100%; height: 100%; display: block;" />
</template>

