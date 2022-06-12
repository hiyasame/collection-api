import { ReactNode } from "react"
import classnames from 'classnames'
import './style.css'

type Props = {
    elements: ReactNode[],
    onPageChange: (node: ReactNode, page: number) => void,
    selectedIndex: number
}

export default function SideBar(props: Props) {
    const btns = props.elements.map((btn, index) => <div key={index} className={classnames({
        'side-bar-btn': true,
        'side-bar-btn-active': index === props.selectedIndex,
    })} onClick={() => props.onPageChange(btn, index)}>{btn}</div>)
    return (
        <div className='side-bar-container'>
            {btns}
        </div>
    )
}