import { useNavigate } from 'react-router-dom'
import GithubCorner from '../../components/github-corner'
import OutlineButton from '../../components/outline-button'
import Spacer from '../../components/spacer'
import './style.css'

export default function HomePage() {
    const navigate = useNavigate()
    return (
        <div className='home-background'>
            <div className='home-bg-color'>
                <h1 className='home-title'>Rain's Collection</h1>
                <p className='home-sub-title'>Rain's collection in various fields...</p>
                <Spacer width={400} />
                <div>
                    <OutlineButton content={"Saying"} onClick={() => navigate("/saying")} />
                    <OutlineButton content={"Lyrics"} onClick={() => navigate("/lyrics")} />
                    <OutlineButton content={"Emoticon"} onClick={() => navigate("/emoticon")} />
                    <OutlineButton content={"Wallpaper"} onClick={() => navigate("/wallpaper")} />
                </div>
                <div style={{ margin: 20 }}>
                    <OutlineButton content={"API Docs"} onClick={() => navigate("/api-docs")} />
                </div>
                <GithubCorner url='https://github.com/ColdRain-Moro' />
            </div>
        </div>
    )
}