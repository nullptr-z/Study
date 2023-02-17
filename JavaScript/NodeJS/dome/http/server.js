import http from 'http';
import url from 'url';
import path from 'path';
import fs from 'fs';
import fio from '../file_IO/readFile';


const thisPwd = path.resolve('.');
const server = http.createServer((request, response) => {
  console.log(`接收到请求:${request.method}: ${request.url}`);
  const pathName = url.parse(request.url).pathname;
  const filepath = path.join(thisPwd, pathName);
  const text = fio.readFile(filepath, 'text');

  fs.stat(filepath, (err, stats) => { // 获取文件状态
    response.writeHead(200, { 'Content-Type': 'text/html;charset=utf-8' });
    if (!err && stats.isFile()) { // 正常
      console.log('正常加载文件');
      fs.createReadStream(filepath).pipe(response)
    } else {
      response.end(text);
    }
  });
});
server.listen(2333);
console.log('http://localhost:2333');