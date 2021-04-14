import React, {Component} from 'react';
import SimpleStorage from 'react-simple-storage'
import { Input } from 'antd';

const { TextArea } = Input;
class Out extends Component {
    state={
        output:''
    }
    render() {
        return (
            <div>
                <SimpleStorage parent={this}/>
                <TextArea  style={{'height':'500px'}} value={this.state.output}
                           onChange={e=>{
                               this.setState({output:e.target.value})
                               console.log(this.state.output);}}/>
            </div>
        );
    }
}

export default Out;
