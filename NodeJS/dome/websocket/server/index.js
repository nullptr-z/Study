const websocket = require('ws')
const wskserver = new websocket.Server({ port: 3333 });

wskserver.on('connection', (ws) => {
  console.log('server:收到连接请求');
  ws.on('message', (clientMessage) => {
      console.log('server:接收到客户端信息', clientMessage);
      setTimeout(() => {

        ws.send('server: hi, 客户端');
      },3000)
  });

});