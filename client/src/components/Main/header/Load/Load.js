import React, {Component} from 'react';
import PubSub from 'pubsub-js'
import "./Load.css"
class Load extends Component {
    showFile=async(e)=>{
        e.preventDefault()
        const reader=new FileReader()
        reader.onload=async (e)=>{
            const text=(e.target.result)
            PubSub.publish("file",text)
        };
        reader.readAsText(e.target.files[0])
    }
    render() {
        return (
           <div>
               <input type="file" name='file' id='file' className="inputfile"
                      onChange={(e)=>this.showFile(e)}/>
               <label htmlFor='file' style={{verticalAlign:"center"}}>Choose a file</label>
           </div>
        );
    }
}

export default Load;
