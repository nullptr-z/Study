import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
import * as wasm from "wasm-game-of-life/wasm_game_of_life_bg.wasm";
import fps from './fps'

const windowHeight = document.body.clientHeight
const gridDimens = 256         // 网格高宽X,y(正方形)

// 画布网格大小：px
const CELL_SIZE = Number.parseInt((windowHeight / gridDimens).toFixed(0)) - 1; // px
const GRID_COLOR = "#F5DA70";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#80E694";

const universe = Universe.new(gridDimens, gridDimens, true);
const width = universe.get_width()
const height = universe.get_height()
const cellsPtr = universe.get_cells_ptr()

// 设置画布
const canvas = document.getElementById("game-of-life-canvas") // as
canvas.width = (CELL_SIZE + 1) * width + 1  // +1是为了边框线
canvas.height = (CELL_SIZE + 1) * height + 1  // +1是为了边框线
const ctx = canvas.getContext('2d')

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cells = new Uint8Array(wasm.memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    ctx.fillStyle = ALIVE_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] !== Cell.Alive) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    // Dead cells.
    ctx.fillStyle = DEAD_COLOR;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] !== Cell.Dead) {
                continue;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const showCanvas = () => {
    drawGrid();     // 网格边框线
    drawCells();    // 网格
}

canvas.addEventListener('click', e => {
    // 获取画布信息。W,H  X,Y  L,R,T,B
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;
    console.log('scale', scaleX, scaleY)

    // 获取点击位置的Rect(方格)信息。W,H  X,Y  L,R,T,B
    const canvasLeft = (e.clientX - boundingRect.left) * scaleX;
    const canvasTop = (e.clientY - boundingRect.top) * scaleY;

    console.log('canvas', canvasLeft, canvasTop)

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    // 反转状态
    universe.toggle_cell(row, col);
    // 渲染画布
    showCanvas();    // 网格
})


let actionGameId = null

const playButton = document.getElementById('play-pause')
const play = () => {
    playButton.textContent = "⏸";
    renderLoop();
};

const suspend = () => {
    playButton.textContent = "▶"
    cancelAnimationFrame(actionGameId);
    actionGameId = null
}

const isPaused = () => {
    return actionGameId === null;
};

playButton.addEventListener('click', (e) => {
    if (isPaused()) {
        play()
    } else {
        suspend()
    }
})

const renderLoop = () => {
    fps.render()
    // 下一帧
    universe.tick();

    showCanvas()

    actionGameId = requestAnimationFrame(renderLoop);
};

// 初始化状态
showCanvas()
