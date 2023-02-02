const fs = require('fs')

module.exports = {
  readFile(path) {
    try {
      const text = fs.readFileSync(path, 'utf-8');
      return text;
    } catch(error){
      return '文件不存在' + error;
    }
  }
}
