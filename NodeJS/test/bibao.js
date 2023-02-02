function A(a) {             // 像这样的一个拥有自由参数a的叫做open lambda
  console.log('a =', ++a);
}

function B(x) {
  let a = x + 1;            // 在这里提供一个参数open lambda所需的参数,
  return () => A(a);              // 把一个open lambda关闭的叫做闭包
}
const result = B(1);
result();