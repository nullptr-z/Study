import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { Ts } from './types';

ReactDOM.render(
  <Ts msgs="TypeScript基本运行环境" />,
  document.getElementById('TS') // 在这个ID标签内渲染----innerHTML
);


const a = { aa: 1 }
const b = { bb: 1 }
type t = typeof a & typeof b
const c: t = { aa: 2, bb: 3 }
