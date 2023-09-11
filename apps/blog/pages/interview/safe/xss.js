document.addEventListener('DOMContentLoaded', () => {
  // h5 不执行标签内部的script
  // document.getElementById('innerHTML').innerHTML = `<script>alert('sdf')</script>`;
  document.getElementById('innerHTML').innerHTML = `<img src=x onerror="alert('sdf')">`;
  // document.getElementById('innerHTML').textContent = `<script>alert('sdf')</script>`;
})
