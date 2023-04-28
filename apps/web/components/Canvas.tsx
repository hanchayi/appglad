import { useEffect, useRef } from "react"

function Canvas(props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    canvasRef.current?.focus();
    console.log(canvasRef.current);
  }, []);

  return <canvas ref={ canvasRef } width="640" height="480">
    你的浏览器似乎不支持或者禁用了 HTML5 <code>&lt;canvas&gt;</code> 元素。
  </canvas>
}

export default Canvas;
