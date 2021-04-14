import React, {Component} from 'react';
import PubSub from 'pubsub-js'
import { Select } from 'antd';

const { Option } = Select;

function onChange(value) {
    PubSub.publish("sel",value)
}

function onBlur() {
    //console.log('blur');
}

function onFocus() {
    //console.log('focus');
}

function onSearch(val) {
    //console.log('search:', val);
}
class Sel extends Component {
    render() {
        return (
                <Select
                    showSearch
                    placeholder="Select a language"
                    optionFilterProp="children"
                    onChange={onChange}
                    onFocus={onFocus}
                    onBlur={onBlur}
                    onSearch={onSearch}
                    filterOption={(input, option) =>
                        option.children.toLowerCase().indexOf(input.toLowerCase()) >= 0
                    }
                >
                    <Option value="c++">C++</Option>
                    <Option value="c#">C#</Option>
                </Select>
        );
    }
}

export default Sel;
