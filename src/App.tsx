import { Route, Routes } from 'react-router-dom'
import HomePage from './page/home'
import NotFound from './page/not-found'

function App() {
  return (
    <Routes>
      <Route path='/'>
        <Route index element={ <HomePage /> } />
        <Route path='wall'>
          <Route path='saying' />
          <Route path='lyrics' />
          <Route path='emoticon' />
          <Route path='wallpaper' />
          <Route path='api-docs' />
        </Route>
      </Route>
      <Route path='*' element={ <NotFound /> } />
    </Routes>
  )
}

export default App
