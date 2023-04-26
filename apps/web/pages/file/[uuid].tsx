import { useRouter } from 'next/router'
import { greet } from '@appglad/engine'

function File() {
  const router = useRouter()
  const { uuid } = router.query

  return <p onClick={() => {
    greet("sdf")
  }}>File: {uuid}</p>
}

export default File
