import React, {Component} from 'react';
import PubSub from 'pubsub-js'
import { Button} from 'antd';
import axios from "axios";
class Btn extends Component {
    state={
        language:'',
        output_name:'',
        contents:''
    }
    componentDidMount() {
        PubSub.subscribe("sel",(msg,data)=>{
            this.setState({
                language:data
            })
        })
        PubSub.subscribe("space",(msg,data)=>{
            this.setState({
                output_name:data
            })
        })
        PubSub.subscribe("file",(msg,data)=>{
            this.setState({
                contents:data
            })
        })
    }

    post=()=>{
         let myHeaders = new Headers({
             // 'Content-Type': 'application/json'

         });
        console.log(this.state)
        axios({
            headers:myHeaders,
            method:'post',
            url:'http://10.166.0.125:3000/api',
            data:this.state,
            mode:'no-cors'
        }).then(function (response,data) {
            console.log(response)
            PubSub.publish('url',response.data.content)
        }).catch((error)=>{
            console.log(error)
        })
        // fetch('http://47.110.143.150:8000/protoc',{
        //     method:'post',
        //     headers:myHeaders,
        //     body:JSON.stringify(this.state),
        //     mode: "no-cors"
        // }).then((response)=>{
        //     console.log()
        //     return response
        // }).then((data)=>{
        //     console.log(data)
        // }).catch(function (error){
        //     console.log(error)
        // })
    }
    render() {
        return (
                <Button style={{maxWidth:'100%'}} onClick={this.post}>Generate</Button>
        );
    }
}

export default Btn;
