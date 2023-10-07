function iter(r, l, call) {
  for (let i = 0; i < r; i++) {
    const index = i * l
    for (let j = 0; j < l; j++) {
      call(index + j, i)
    }
  }
}

const arr = []
const r = 8, l = 8
iter(
  r, l,
  (index, line) => arr[index] = 0
)

function getArrLine(arr, rLine, rLen) {
  const line = rLine * rLen
  return (arr.slice(line, line + rLen))
}

const formatPut = () => {
  for (let i = 0; i < r; i++) {
    console.log(getArrLine(arr, i, l).toString().replace(new RegExp(',', 'g'), '　'))
  }
}
// formatPut()


/** -------------------------------------------------------- */

class Backtracking {
  inner = []
  currentPtr

  // 改变最新分支节点
  bcChange(index, value) {
    this.currentPtr = index
    this.inner.push({ index, coordinate: value })
  }

  getIndexByCoordinate(index) {
    const row = Number.parseInt((index) / l)

    const col = index - ((row * l) - 1)
    return { r: row, l: col - 1 }
  }
}

function backtrackingFunc(arr) {
  const bc = new Backtracking()
  for (let i = 0; i < arr.length; i++) {
  }
}


function EightQueen() {
  let bc = new Backtracking()
  const QueenCount = 8
  const startIndex = 0
  const len = arr.length
  let count = 0;
  const counts = new Map()
  for (let s = startIndex; s < QueenCount; s++) {
    bc = new Backtracking()
    for (let i = s; i < len; i++) {
      const thatCoord = bc.getIndexByCoordinate(i)
      const { r: tr, l: tl } = thatCoord
      const findEqual = bc.inner.find((queenCoord) => {
        const { r, l } = queenCoord.coordinate
        const isEq = (r, l) => r === l
        const xEq = isEq(r, tr)
        const yEq = isEq(l, tl)

        let slashEq = false
        for (let k = 1; k < QueenCount - r; k++) {
          slashEq = (isEq(r + k, tr) && isEq(l + k, tl)) || (isEq(r + k, tr) && isEq(l - k, tl))
          if (slashEq) break
        }
        return xEq || yEq || slashEq
      })
      // 如果当前位置没有冲突
      if (!findEqual) {
        bc.bcChange(i, thatCoord)
      }
      if ((i + 1) === len) {
        if (bc.inner.length !== 8) {
          let lastCoord = bc.inner.pop()

          if (!bc.inner.length) {
            break
          }

          if (lastCoord.index === (len - 1)) {
            lastCoord = bc.inner.pop()
            if (lastCoord.coordinate.r === bc.getIndexByCoordinate(s).r) { // 减少重复回溯次数,这些回溯会在后续(行)遍历
              break
            }
          }

          i = lastCoord.index
        } else {
          // let temp = ''
          // bc.inner.forEach(item => {
          //   temp = temp + item.index
          // })
          // counts.set(temp)
          count++

          // markFormat(bc)
          // if (bc.inner.length !== 8) {
          // console.log('--------', bc.inner.length)
          //   while (true);
          // }

          let lastCoord = bc.inner.pop()
          if (lastCoord.coordinate.l === (l - 1)) { // 如果不写会导致又跳到下一行, 这一行之前将永远无法访问,导致回溯不完整
            lastCoord = bc.inner.pop()
          }
          i = lastCoord.index
        }
      }
    }

  }

  console.log('count: ', count);

  // console.log('inner', bc.inner, bc.inner.length)
}

function markFormat(bc) {
  arr.forEach((_, index) => {
    const findEqual = bc.inner.find((queenCoord) => {
      return queenCoord.index === index
    })
    if (findEqual) {
      arr[index] = 'x'
    }
  });
  formatPut()
  arr.forEach((_, i) => arr[i] = 0)
}

console.time('耗时')
EightQueen()
console.timeEnd('耗时')


