import http from 'http';
import formdata from 'formidable';

http.createServer((request, respone) => {
  console.log('这里是图片服务');
  const form = new formdata.IncomingForm();
  console.log('method', request.method);

  form.uploadDir = "./file";

  let msg = {};
  form.parse(request, (err, fields, file) => {
    msg = { err, fields, file };
  });
  console.log(msg);
  respone.writeHead(200, { 'content-type': 'text/html;charset=utf-8' });
  respone.end(`${JSON.stringify(msg)}`);
}).listen(2334);

console.log('http://localhost:2334');
