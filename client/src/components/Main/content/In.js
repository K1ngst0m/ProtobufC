import React, {Component} from 'react';
import PubSub from 'pubsub-js'
import { Input } from 'antd';

const { TextArea } = Input;
class In extends Component {
    state={
        content:''
    }
    input=(event)=>{
        const {value}=this.keyWord.resizableTextArea.props

        this.setState(function (state,props) {
            return{
                content:value
            }
        })
        PubSub.publish("con",value)
    }
    componentDidMount() {

        PubSub.subscribe("file",(msg,data)=>{
            this.setState({
                content:data
            })
        })
    }
    render() {
        return (
            <TextArea ref={c=>this.keyWord=c} value={this.state.content} style={{'height':'500px'}} onBlur={this.input} />
        );
    }
}

export default In;
