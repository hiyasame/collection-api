import { ReactNode } from 'react'
import './style.css'

type Props = {
    content: ReactNode
    onClick?: () => void
}

export default function OutlineButton(props: Props) {
    return (
        <div className='outline-button-container' onClick={props.onClick}>
            {props.content}
        </div>
    )
}