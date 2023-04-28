import '@appgald/ui/styles/global.css';
import { AppProps } from 'next/app';

function App(props: AppProps) {
  console.log('props', props)
  const Component = props.Component;
  return <Component />
}

export default App;
