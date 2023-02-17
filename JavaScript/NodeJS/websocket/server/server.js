var expre = require('express')();
var http = require('http').Server(expre);
var sio = require('socket.io')(http);

expre.get('/', function (req, res) {
    // res.send('/show.html'); // 导入首页
    res.sendFile(__dirname + '/show.html'); // 导入首页
});

// .of('/api') // 定义额外命名空间，共享socket
sio.on('connection', (client) => {
    client.on('sendMsg', ((param) => {
        param.id.map((id) => {
            client.broadcast.emit(id, param.name, param.content);
        });
    }));
});

const port = 8000;
sio.listen(port);
console.log('启动服务', ` http://localhost:${port}`);
