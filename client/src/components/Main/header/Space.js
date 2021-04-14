import React, {Component} from 'react';
import PubSub from 'pubsub-js'
import { Input } from 'antd';
class Space extends Component {
    state={
        space:''
    }
    input=(event)=>{
        const {value}=this.keyWord.state
        this.setState(function (state,props) {
            return{
                space:value
            }
        })
        PubSub.publish("space",value)
    }
    render() {
        return (
            <Input placeholder="nameSpace"  ref={c=>this.keyWord=c} onBlur={this.input}/>
        );
    }
}

export default Space;
