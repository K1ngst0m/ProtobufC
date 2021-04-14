import React, {Component} from 'react';
import { Button} from 'antd';
import { DownloadOutlined } from '@ant-design/icons';
import PubSub from 'pubsub-js'

class Download extends Component {
    state = {
        size: 'large',
        url:''
    };
    componentDidMount() {
        PubSub.subscribe("url",(msg,data)=>{
            this.setState({
                url:data
            })
        })
    }

    render() {
        const { size } = this.state;
        return (
            <Button type="primary" shape="round" icon={<DownloadOutlined />} size={size}>
                <a href={this.state.url} style={{color:"white"}}>
                    Download
                </a>
            </Button>
        );
    }
}

export default Download;
