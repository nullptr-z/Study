function mapp(func, e = undefined) {
    for (let i = 0; i < this.length; i += 1) {
        func.call(e, this[i], i);
    }
}

const arr = [1, 2, 3, 4, 5];
arr.map = mapp;

arr.map(function (a, index) {
    console.log('-->', a, index, this);
});

arr.map(
    function (a, index) {
        console.log('==>', this.b, a, index);
    },
    { a: '哈哈', b: '呵呵' },
);