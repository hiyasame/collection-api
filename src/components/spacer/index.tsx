import './style.css'

type Props = {
    width?: string | number
}

export default function Spacer(props: Props) {
    return (
        <div className='spacer-container' style={{ width: props.width ? props.width : 100 }}></div>
    )
}