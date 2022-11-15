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

function initBuffers(gl: WebGLRenderingContext): Buffers {
  // Create a buffer for the square's positions.
  const positionBuffer = gl.createBuffer();

  if (!positionBuffer) {
    throw new Error('create Buffer error')
  }
  // Select the positionBuffer as the one to apply buffer
  // operations to from here out.
  gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
  // Now create an array of positions for the square.
  const positions = [
    1.0,  1.0,
    -1.0,  1.0,
    1.0, -1.0,
    -1.0, -1.0,
  ];
  // Now pass the list of positions into WebGL to build the
  // shape. We do this by creating a Float32Array from the
  // JavaScript array, then use it to fill the current buffer.

  gl.bufferData(gl.ARRAY_BUFFER,
                new Float32Array(positions),
                gl.STATIC_DRAW);

  return {
    position: positionBuffer,
  };
}

function drawScene(gl: WebGLRenderingContext, programInfo: ProgramInfo, buffers: Buffers) {
  gl.clearColor(0.0, 0.0, 0.0, 1.0);  // Clear to black, fully opaque
  gl.clearDepth(1.0);                 // Clear everything
  gl.enable(gl.DEPTH_TEST);           // Enable depth testing
  gl.depthFunc(gl.LEQUAL);            // Near things obscure far things

  // Clear the canvas before we start drawing on it.

  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

  // Create a perspective matrix, a special matrix that is
  // used to simulate the distortion of perspective in a camera.
  // Our field of view is 45 degrees, with a width/height
  // ratio that matches the display size of the canvas
  // and we only want to see objects between 0.1 units
  // and 100 units away from the camera.

  const fieldOfView = 45 * Math.PI / 180;   // in radians
  const aspect = gl.canvas.clientWidth / gl.canvas.clientHeight;
  const zNear = 0.1;
  const zFar = 100.0;
  const projectionMatrix = mat4.create();

  // note: glmatrix.js always has the first argument
  // as the destination to receive the result.
  mat4.perspective(projectionMatrix,
                  fieldOfView,
                  aspect,
                  zNear,
                  zFar);

  // Set the drawing position to the "identity" point, which is
  // the center of the scene.
  const modelViewMatrix = mat4.create();

  // Now move the drawing position a bit to where we want to
  // start drawing the square.

  mat4.translate(modelViewMatrix,     // destination matrix
                modelViewMatrix,     // matrix to translate
                [-0.0, 0.0, -6.0]);  // amount to translate

  // Tell WebGL how to pull out the positions from the position
  // buffer into the vertexPosition attribute.
  {
    const numComponents = 3;  // pull out 3 values per iteration
    const type = gl.FLOAT;    // the data in the buffer is 32bit floats
    const normalize = false;  // don't normalize
    const stride = 0;         // how many bytes to get from one set of values to the next
                              // 0 = use type and numComponents above
    const offset = 0;         // how many bytes inside the buffer to start from
    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
    gl.vertexAttribPointer(
        programInfo.attribLocations.vertexPosition,
        numComponents,
        type,
        normalize,
        stride,
        offset);
    gl.enableVertexAttribArray(
        programInfo.attribLocations.vertexPosition);
  }

  // Tell WebGL to use our program when drawing

  gl.useProgram(programInfo.program);

  // Set the shader uniforms

  gl.uniformMatrix4fv(
      programInfo.uniformLocations.projectionMatrix,
      false,
      projectionMatrix);
  gl.uniformMatrix4fv(
      programInfo.uniformLocations.modelViewMatrix,
      false,
      modelViewMatrix);

  {
    const offset = 0;
    const vertexCount = 4;
    gl.drawArrays(gl.TRIANGLE_STRIP, offset, vertexCount);
  }
}


addEventListeners()
let gl: WebGLRenderingContext | null;
let canvas = ref<HTMLCanvasElement | null>(null);
onMounted(() => {
  const el = canvas.value;

  if (!el) {
    throw new Error('el not found')
  }
  console.log('canvas', el)
  gl = el.getContext('webgl');

  if (!gl) {
    throw new Error("not support gl");
  }

  // 黑色清空
  gl.clearColor(0.0, 0.0, 0.0, 1.0);
  // 用刚刚设置的颜色刷新缓冲区
  gl.clear(gl.COLOR_BUFFER_BIT);

  const vsSource = `
  attribute vec4 aVertexPosition;

  uniform mat4 uModelViewMatrix;
  uniform mat4 uProjectionMatrix;

  void main() {
    gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
  }
  `

  const fsSource = `
    void main() {
      gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
  `;

  const program = initShaderProgram(gl, vsSource, fsSource);
  const projectionMatrix = gl.getUniformLocation(program, 'uProjectionMatrix');

  if (!projectionMatrix) {
    throw new Error('projectionMatrix not found')
  }
  const modelViewMatrix = gl.getUniformLocation(program, 'uModelViewMatrix')
  if (!modelViewMatrix) {
    throw new Error('modelViewMatrix not found')
  }

  const programInfo: ProgramInfo = {
    program: program,
    attribLocations: {
      vertexPosition: gl.getAttribLocation(program, 'aVertexPosition'),
    },
    uniformLocations: {
      projectionMatrix,
      modelViewMatrix,
    },
  };

  const buffers = initBuffers(gl);
  drawScene(gl, programInfo, buffers);  
})
</script>

<template>
  <canvas ref='canvas'/>
</template>

