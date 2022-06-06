import './style.css'
import icon from './404-icon.png'
import Button from '../../components/button';
import { useNavigate } from 'react-router-dom';

export default function NotFound() {
    const navigate = useNavigate();
    return (
        <div className='page-container'>
            <h1>404 Not Found</h1>
            <div style={{ display: "flex", flexDirection: "row", alignItems: "center" }}>
                <p style={{ margin: "10px" }}>哎呀，页面找不到了...</p>
                <img src={icon}></img>
            </div>
            <Button content={"回到首页"} onClick={() => navigate("/")} />
        </div>
    );
}