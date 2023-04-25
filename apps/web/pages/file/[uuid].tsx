import { useRouter } from 'next/router'

function File() {
  const router = useRouter()
  const { uuid } = router.query

  return <p>File: {uuid}</p>
}

export default File
