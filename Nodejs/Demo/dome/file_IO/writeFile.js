const fs = require('fs')

const data = '哇哈哈哈'
fs.writeFile('./text', data, (error) => {
  if (error) {
    console.log(error);
  } else {
    console.log('写入成功');
  }
})
