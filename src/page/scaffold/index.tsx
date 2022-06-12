import { ReactNode, useState } from 'react'
import SideBar from '../../components/side-bar'
import './style.css'

type Props = {
    elements: [string, ReactNode][]
}

export default function Scaffold(props: Props) {

    const [selectedIndex, setSelectedIndex] = useState(0)

    return (
        <div className='scaffold-container'>
            <SideBar selectedIndex={selectedIndex}
                elements={props.elements.map(([name]) => name)}
                onPageChange={(_, index) => { setSelectedIndex(index) }} />
            <div className='scaffold-page-container'>
                {props.elements[selectedIndex][1]}
            </div>
        </div>
    )
}