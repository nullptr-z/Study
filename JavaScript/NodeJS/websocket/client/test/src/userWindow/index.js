import React from 'react';
import ReactDOM from 'react-dom';
import sio from 'socket.io-client';


export default class UserWindow extends React.Component {
    socket = sio.connect('http://192.168.188.108:8000/');

    state = {
        mag: {},
        acceptMsg: [],
        actionID: [],
    }
    componentDidMount() {
        const { id, name } = this.props;
        this.socket.on(id, (name, content) => {
            const acceptMsg = this.state.acceptMsg;
            acceptMsg.push({ name, content });
            this.setState({ acceptMsg });
        }); // 接受消息
    }
    sendMsg() {
        const { id, name } = this.props;
        const actionID = this.state.actionID.filter(m => m);
        this.socket.emit('sendMsg',
            { id: actionID, name, content: this.state.value }
        ); // 发送消息
    }
    render() {
        const { name, id, use } = this.props;
        const { actionID } = this.state;
        return (
            <div>
                <div
                    style={{
                        border: '1px solid #888888',
                        height: 400,
                        width: 400,
                        float: 'left'
                    }}
                >
                    <h3>用户名:{name}</h3>
                    <div>
                        <label>发送消息给:</label>
                        {
                            use.map((item, index) => {
                                if (item.id === id) return;
                                return (
                                    <button
                                        style={{ borderColor: actionID[index] ? 'red' : '#000' }}
                                        onClick={() => {
                                            actionID[index] = actionID[index] ? null : item.id;
                                            this.setState({ actionID });
                                        }}
                                        key={index}
                                    >
                                        {item.name}
                                    </button>
                                );
                            })
                        }
                    </div>
                    <h4>聊天内容</h4>
                    <div
                        style={
                            {
                                height:215,
                                width: 400,
                                color: 'blue',
                                overflowY: 'auto'
                            }
                        }
                    >
                        {this.state.acceptMsg.map((item, index) => {
                            return (
                                <span key={index} style={{ color: 'blue', display: 'block' }}>
                                    {item.name}: {item.content}
                                </span>
                            );
                        })
                        }
                    </div>
                    <textarea onChange={(e) => {
                        this.setState({ value: e.target.value });
                    }}
                        style={{ height: 30,width:'85%'}}
                    />
                    <button onClick={() => this.sendMsg()}>发送</button>
                </div>
            </div>
        );
    }
}
