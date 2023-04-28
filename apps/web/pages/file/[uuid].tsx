import { useRouter } from 'next/router'
import { greet } from '@appglad/engine'
import Head from 'next/head'

function File() {
  const router = useRouter()
  const { uuid } = router.query

  return <>
    <Head>
      <title>sdf</title>
    </Head>
      <p onClick={() => {
      greet("sdf")
    }}>File: {uuid}</p>
  </>
}

export default File
