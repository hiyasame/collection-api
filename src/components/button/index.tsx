import { ReactElement, ReactNode } from 'react'
import './style.css'

type Props = {
    icon?: ReactNode
    content: ReactNode
    onClick?: () => void
}

export default function Button(props: Props) {
    return (
        <div className='button-container' onClick={props.onClick}>
            {props.icon}
            {props.content}
        </div>
    )
}