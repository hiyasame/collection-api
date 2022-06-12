import { Route, Routes } from 'react-router-dom'
import HomePage from './page/home'
import NotFound from './page/not-found'
import Scaffold from './page/scaffold'

function App() {
  return (
    <Routes>
      <Route path='/'>
        <Route index element={ <HomePage /> } />
        <Route path='wall' element={ <Scaffold elements={[["Saying", <div></div>], ["Lyrics", <div></div>]]} /> } />
      </Route>
      <Route path='*' element={ <NotFound /> } />
    </Routes>
  )
}

export default App
