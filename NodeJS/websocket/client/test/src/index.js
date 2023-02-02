import React from 'react';
import ReactDOM from 'react-dom';
import UserWindow from './userWindow';
import User from './user';

class App extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        const users = [
            new User(1000, '大白'),
            new User(1001, '小白'),
            new User(1002, '白白')
        ];
        return (
            <div>
                {
                    users.map((use, index) => {
                        return (
                            <UserWindow
                                key={index}
                                {...use}
                                use={users}
                            />
                        );
                    })
                }
            </div>
        );
    }
}

ReactDOM.render(<App />, document.getElementById('root'));