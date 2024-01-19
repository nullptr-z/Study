const webs = require('ws');
const wsk = new webs('ws://localhost:3333');

wsk.onopen = function () {
  console.log('client: 连接服务器');
  wsk.send('client: hi 服务器');
  wsk.onmessage = function (e) {
    console.log('client: 接收到服务器信息' + e.data);
  }

  wsk.onclose = function (close) {
    console.log('client: 关闭连接');
  };
}