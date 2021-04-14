import React, {Component} from 'react';
import { Layout,Row, Col} from 'antd';
import In from "./content/In";
import Out from "./content/Out";
import Btn from "./header/Btn";
import Sel from "./header/Sel";
import Space from "./header/Space";
import 'antd/dist/antd.css'
import Load from "./header/Load/Load";
import Download from "./header/Download";
const { Header, Content, Footer } = Layout;
class Main extends Component {
    render() {
        return (
            <div>
                <Layout>
                    <Header style={{ position: 'fixed', zIndex: 1, width: '100%' }}>
                        <div className="logo" />
                    </Header>
                    <Content className="site-layout" style={{ padding: '0 50px', marginTop: 64 }}>
                        <div className="site-layout-background" style={{ padding: 24, minHeight: 600 }}>
                            <Row style={{textAlign:'left'}}>
                                <Col span={3} >
                                    <Sel/>
                                </Col>
                                <Col span={2} >
                                    <Space/>
                                </Col>
                                <Col span={3} offset={3} >
                                    <Load/>
                                </Col>
                                <Col span={3}>

                                    <Btn/>
                                </Col>
                                <Col span={3}  offset={7}>
                                    <Download/>
                                </Col>
                            </Row>
                            <Row style={{textAlign:'center', paddingTop:'10px'}}>
                                <Col span={11} >
                                    <In/>
                                </Col>
                                <Col span={2} >
                                </Col>
                                <Col span={11}>
                                    <Out/>
                                </Col>
                            </Row>
                        </div>
                    </Content>
                    <Footer style={{ textAlign: 'center' }}>COMBO-V</Footer>
                </Layout>
            </div>
        );
    }
}

export default Main;
