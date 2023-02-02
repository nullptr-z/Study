//loss loss 散列函数
let djb2HashCode = function (key) {
  let hash = 5381;
  for (let i = 0; i < key.length; i++) {
    const hashs = hash << 5, chat = key.charCodeAt(i)
    console.log('hashs + chat: ', hashs, chat);
    console.log('binaryString', binaryString(hash), binaryString(hashs), binaryString(chat))
    hash = hashs + chat;
    console.log('hash binary', binaryString(hash))
    return
  }
  return hash % 1013;  //如果最大限度是 1000 的话，就要比 1000 大
};

//开创创建一个散列表吧
//对 HashTable 类进行改写，分离链接法，即每个值是一个单链表的数据结构
function HashTable() {
  let table = [];
  let valuePair = function (key, value) {
    this.key = key;
    this.value = value;
    this.toString = function () {
      console.log(`${key} --- ${value}`);
    };
  };

  this.put = function (key, value) {
    let position = djb2HashCode(key);
    console.log(`${position} --- ${key}`);
    if (table[position] === undefined) table[position] = new LinkedList();
    table[position].append(new valuePair(key, value));
  };
  this.remove = function (key) {
    let position = djb2HashCode(key);
    if (table[position] !== undefined) {
      let cur = table[postion].getHead();
      while (cur.next) {
        if (cur.element.key === key) {
          table[position].remove(cur.element);
          if (table[postion].isEmpty) {
            table[position] = undefined;
          }
          return true;
        }
        cur = cur.next;
      }
      //为链表的第一项或者最后一项
      if (cur.element.key === key) {
        table[position].remove(cur.element);
        if (table[postion].isEmpty) {
          table[position] = undefined;
        }
        return true;
      }
    }
    return false;
  };
  this.get = function (key) {
    let position = djb2HashCode(key);
    if (table[position] !== undefined) {
      let cur = table[position].getHead();
      while (cur.next) {
        if (cur.element.key === key) {
          return cur.element.value;
        }
        cur = cur.next;
      }
      if (cur.element.key === key) {
        return cur.element.value;
      }
    }
    return undefined;
  };
};

//线性查找
function HashTable2() {
  let table = [];
  let ValuePair = function (key, value) {
    this.key = key;
    this.value = value;

    this.toString = function () {
      console.log(`${key} ---- ${value}`);
    }
  };
  this.put = function (key, value) {
    let position = loseloseHashCode(key);
    if (table[position] === undefined) table[position] = new ValuePair(key, value);
    let index = ++position;
    while (table[index] !== undefined) {
      index++;
    }
    table[index] = new ValuePair(key, value);
  };
  this.remove = function (key) {
    let position = loseloseHashCode(key);
    if (table[position] !== undefined) {
      let index = ++position;
      while ((table[index] === undefined || table[index].key !== key) && index < 36) {
        index++;
      }
      if (table[index] && table[index].key === key) {
        table[index] = undefined;
        return true;
      }
    }
    return false;
  };
  this.get = function (key) {
    let position = loseloseHashCode(key);
    if (table[position] !== undefined) {
      //这个位置上的值是否就是我们要查找的值，如果不是就往下一个位置查找
      if (table[position].key === key) {
        return table[position].value;
      } else {
        let index = ++position;
        while ((table[index] === undefined || table[index].key !== key) && index < 36) {
          index++;
        }
        if (table[index] && table[index].key === key) {
          return table[index].value;
        }
      }
    }
    return undefined;
  };
}

let hash = new HashTable();

hash.put('Gandalf', 'gandalf@emai.com');
// hash.put('John', 'johnsnow@email.com');
// hash.put('Tyrion', 'tyrion@email.com');
// hash.put('Aaron', 'Aaron@email.com')

// console.log('get Gandalf', hash.get('Gandalf'));
// console.log('get Tyrion', hash.get('Tyrion'));
// hash.remove('John');
// console.log('get John', hash.get('John'));






function binaryString(num) {
  //定义变量存放字符串
  var result = '';
  while (true) {
    //取余
    var remiander = num % 2;
    //将余数倒序放入结果中
    result = remiander + result;//+是字符串的拼接
    //求每次除2的商
    num = ~~(num / 2);
    // num= num>>1;
    if (num === 0)
      break;
  }
  return result;
}
