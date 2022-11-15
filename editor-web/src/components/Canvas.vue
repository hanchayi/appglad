<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { mat4 } from 'gl-matrix';

interface ProgramInfo {
  program: WebGLProgram,
  attribLocations: {
    vertexPosition: GLint,
  },
  uniformLocations: {
    projectionMatrix: WebGLUniformLocation,
    modelViewMatrix: WebGLUniformLocation,
  },
}

interface Buffers {
  position: WebGLBuffer
}

function addEventListeners() {
  console.log('addEventListeners')
  window.addEventListener('resize', () => {
    console.log('resize')
  })
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

  // Send the source to the shader object
  gl.shaderSource(shader, source);
  // Compile the shader program
  gl.compileShader(shader);

  // See if it compiled successfully
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    console.log('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader));
    gl.deleteShader(shader);
    throw new Error('create shader error')
  }

  return shader;
}

function initBuffers(gl: WebGLRenderingContext) {
  // Create a buffer for the square's positions.

  const positionBuffer = gl.createBuffer();

  // Select the positionBuffer as the one to apply buffer
  // operations to from here out.

  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

  // Now create an array of positions for the square.

  const positions = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0];

  // Now pass the list of positions into WebGL to build the
  // shape. We do this by creating a Float32Array from the
  // JavaScript array, then use it to fill the current buffer.

  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);
}

function drawScene(gl: WebGLRenderingContext, program: WebGLProgram) {
  gl.clearColor(0.0, 0.0, 0.0, 1.0); // Clear to black, fully opaque
  gl.clearDepth(1.0); // Clear everything
  gl.enable(gl.DEPTH_TEST); // Enable depth testing
  gl.depthFunc(gl.LEQUAL); // Near things obscure far things

  // Clear the canvas before we start drawing on it.

  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

  // Create a perspective matrix, a special matrix that is
  // used to simulate the distortion of perspective in a camera.
  // Our field of view is 45 degrees, with a width/height
  // ratio that matches the display size of the canvas
  // and we only want to see objects between 0.1 units
  // and 100 units away from the camera.

  const fieldOfView = (45 * Math.PI) / 180; // in radians
  const aspect = gl.canvas.clientWidth / gl.canvas.clientHeight;
  const zNear = 0.1;
  const zFar = 100.0;
  const projectionMatrix = mat4.create();

  // note: glmatrix.js always has the first argument
  // as the destination to receive the result.
  mat4.perspective(projectionMatrix, fieldOfView, aspect, zNear, zFar);

  // Set the drawing position to the "identity" point, which is
  // the center of the scene.
  const modelViewMatrix = mat4.create();

  // Now move the drawing position a bit to where we want to
  // start drawing the square.

  mat4.translate(
    modelViewMatrix, // destination matrix
    modelViewMatrix, // matrix to translate
    [-0.0, 0.0, -6.0]
  ); // amount to translate

  // Tell WebGL how to pull out the positions from the position
  // buffer into the vertexPosition attribute.
  // {
  //   const numComponents = 2;
  //   const type = gl.FLOAT;
  //   const normalize = false;
  //   const stride = 0;
  //   const offset = 0;
  //   gl.vertexAttribPointer(
  //     programInfo.attribLocations.vertexPosition,
  //     numComponents,
  //     type,
  //     normalize,
  //     stride,
  //     offset
  //   );
  //   gl.enableVertexAttribArray(programInfo.attribLocations.vertexPosition);
  // }

  // Tell WebGL to use our program when drawing

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
onMounted(() => {
  const el = canvas.value;

  if (!el) {
    throw new Error('el not found')
  }
  console.log('canvas', el)
  gl = el.getContext('webgl');

  draw();
})
</script>

<template>
  <canvas ref='canvas' width="640" height="480"/>
</template>

