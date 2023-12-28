import * as wasm from "chess";
import s1 from './assets/1.svg'
import s2 from './assets/2.svg'
import s3 from './assets/3.svg'
import s4 from './assets/4.svg'
import s5 from './assets/5.svg'
import s6 from './assets/6.svg'
import s9 from './assets/9.svg'
import s10 from './assets/10.svg'
import s11 from './assets/11.svg'
import s12 from './assets/12.svg'
import s13 from './assets/13.svg'
import s14 from './assets/14.svg'

let chess_ptr;
let array_ptr, array;
let canvas;
let canvasWrapper;
let fen;
let pgn;
let historyMoves;

const animationDelay = 200;
const isRobot = [false, true];
let rotated = isRobot[0] && !(isRobot[1]);
const movers = [null, null];

const pieceSet = [null, s1, s2, s3, s4, s5, s6, null, null, s9, s10, s11, s12, s13, s14]
const pieceName = [null, 'P', 'R', 'N', 'B', 'Q', 'K', null, null, 'p', 'r', 'n', 'b', 'q', 'k']
let boardSize = 640
let cellSize = boardSize / 8;
const darkBg = "#8c8c8c"
const lightBg = "#d9d9d9"

let status = 'normal';
let possibleMoves = {}
let prevPossibleMoves;


let mask;

const selectionWindow = document.createElement("div");
selectionWindow.id = "selection-window";
const buttonQueen = document.createElement("button");
const buttonRook = document.createElement("button");
const buttonBishop = document.createElement("button");
const buttonKnight = document.createElement("button");

selectionWindow.appendChild(buttonQueen);
selectionWindow.appendChild(buttonRook);
selectionWindow.appendChild(buttonBishop);
selectionWindow.appendChild(buttonKnight);


/**
 * @argument array {Uint8Array}
 * @argument i {number}
 * @argument j {number}
 * @returns {number}
 */
function getPiece(array, i, j) {
    const offset = (i * 8 + j) >> 1;
    const isHigh = j & 1;
    if (isHigh) {
        return array[offset] >> 4;
    } else {
        return array[offset] & 0xf;
    }
}
/**
 * @argument array {Uint8Array}
 * @returns {int}
 */
function getPlayer(array) {
    return array[32] & 1;
}
function getHalfMove(array) {
    return array[33] & 0x7f;
}
function getFullMove(array) {
    return array[34] | (array[35] << 8);
}
function getCastleRights(array) {
    let ans = ""
    if (array[32] & 0x2) ans += 'K';
    if (array[32] & 0x4) ans += 'Q';
    if (array[32] & 0x8) ans += 'k';
    if (array[32] & 0x10) ans += 'q';
    if (ans === "") return "-";
    return ans;
}
function getEnPassant(array) {
    if (array[33] & 0x80) {
        let ans = String.fromCharCode('a'.charCodeAt(0) + (array[32] >> 5));
        if (getPlayer(array) === 0) {
            ans += '6';
        } else {
            ans += '3'
        }
        return ans;
    } else return "-"
}
/**
 * @param {Uint8Array} array 
 */
function toFEN(array) {
    let ans = ""
    let counter = 0
    for (let i = 7; i >= 0; i--) {
        for (let j = 0; j < 8; j++) {
            const piece = getPiece(array, i, j);
            if (piece !== 0) {
                if (counter > 0) {
                    ans += '' + counter;
                    counter = 0;
                }
                ans += pieceName[piece];
            } else {
                counter++;
            }
        }
        if (counter > 0) ans += '' + counter;
        counter = 0;
        if (i !== 0)
            ans += "/"
    }
    ans += getPlayer(array) === 0 ? ' w ' : ' b '
    ans += getCastleRights(array) + " "
    ans += getEnPassant(array) + " "
    ans += getHalfMove(array) + " "
    ans += getFullMove(array)
    return ans
}
function posToString(pos) {
    const i = Math.floor(pos / 8);
    const j = pos % 8;
    return String.fromCharCode('a'.charCodeAt(0) + j, '1'.charCodeAt(0) + i)
}
/**
 * @param {number} piece 
 * @param {number} move 
 */
function toPGN(array, move, prevPossibleMoves, checked) {
    const pos0 = move & 0x3f;
    const pos1 = (move & 0xfc0) >> 6;
    const capture = (move & 0xf0000) != 0;
    const promotion = (move & 0xf000) >> 12;
    const isCastle = (move & 0x200000) !== 0;

    if (isCastle) {
        if (pos0 === 4 && pos1 === 2) {
            return "O-O-O"
        }
        if (pos0 === 4 && pos1 === 6) {
            return "O-O"
        }
        if (pos0 === 60 && pos1 === 58) {
            return "o-o-o"
        }
        if (pos0 === 60 && pos1 === 62) {
            return "o-o"
        }
    }

    let piece;
    if (!promotion) {
        piece = getPiece(array, Math.floor(pos1 / 8), pos1 % 8);
    } else {
        piece = getPlayer(array) === 0 ? 9 : 1;
    }

    let collisionPositions = [];
    for (let src in prevPossibleMoves) {
        const i = Math.floor(src / 8);
        const j = src % 8;
        const anotherPiece = getPiece(array, i, j);
        if (anotherPiece !== piece) continue;
        if (prevPossibleMoves[src].find((v) => {
            return (v & 0xfc0) >> 6 === pos1;
        })) {
            collisionPositions.push(src);
        }
    }
    let uniqueI = true;
    let uniqueJ = true;
    const pos0i = Math.floor(pos0 / 8);
    const pos0j = pos0 % 8;
    for (let x of collisionPositions) {
        const xi = Math.floor(x / 8);
        const xj = x % 8;
        if (xi === pos0i) uniqueI = false;
        if (xj === pos0j) uniqueJ = false;
    }

    let ans = "";
    if (piece !== 1 && piece !== 9) {
        ans += pieceName[piece].toUpperCase()
    }
    if ((collisionPositions.length > 0 && uniqueJ) || ((piece === 1 || piece === 9) && capture)) {
        ans += String.fromCharCode('a'.charCodeAt(0) + pos0j);
    }
    if (!uniqueJ) {
        ans += String.fromCharCode('1'.charCodeAt(0) + pos0i);
    }
    if (capture) {
        ans += 'x'
    }
    ans += posToString(pos1)
    if (promotion !== 0) {
        ans += "=" + pieceName[promotion].toUpperCase()
    }
    if (checked === 1) {
        ans += "+";
    }
    if (checked === 2) {
        ans += "#"
    }
    return ans;
}
async function am_make_move() {
    await new Promise((res, _) => {
        const ans = wasm.am_make_move(movers[getPlayer(array)]);
        const move = Number(ans & 0xffffn); // 16 bit move request
        const evaluation = Number(ans >> 32n); // 32 bit value
        console.log(`Robot ${getPlayer(array) === 1 ? "Black" : "White"} evaluates the situation as ${evaluation / 1000}`)
        const pos0 = move & 0x3f;
        const pos1 = (move & 0xfc0) >> 6;

        // play animation
        const pos0i = Math.floor(pos0 / 8);
        const pos0j = pos0 % 8;
        const pos1i = Math.floor(pos1 / 8);
        const pos1j = pos1 % 8;
        const id = `piece-${pos0i}-${pos0j}`
        const piece = document.getElementById(id);
        if (animationDelay === 0) {
            movePiece(move, true);
            res();
            return;
        }
        piece.style.transform = `translate(${(rotated ? -1 : 1) * (pos1j - pos0j) * cellSize}px, ${(rotated ? 1 : -1) * (pos1i - pos0i) * cellSize}px)`
        piece.style.transition = `transform ${animationDelay}ms ease-out`;
        setTimeout(() => {
            movePiece(move, true);
            res();
        }, animationDelay);
    })
}
/**
 * @param {number} moveReq
 * @param {boolean} isRobot
 */
function movePiece(moveReq, robot) {
    if (robot !== isRobot[getPlayer(array)]) {
        alert("incorrect role");
        return;
    }
    // const pos0i = Math.floor(pos0 / 8);
    // const pos0j = pos0 % 8;
    // const pos1i = Math.floor(pos1 / 8);
    // const pos1j = pos1 % 8;
    // console.log(`Piece ${String.fromCharCode(65 + pos0j)}${pos0i + 1} moved to ${String.fromCharCode(65 + pos1j)}${pos1i + 1}`);
    const src = moveReq & 0x3f;
    const piece = getPiece(array, Math.floor(src / 8), src % 8);
    const res = wasm.cb_do_move(chess_ptr, moveReq);

    wasm.serialize(chess_ptr, array_ptr);
    fen.value = toFEN(array);
    
    const checked = fetch_status();
    let moveStr = toPGN(array, res, prevPossibleMoves, checked);
    const li = document.createElement('button');
    li.innerHTML = moveStr;
    if (getPlayer(array) === 0) {
        moveStr = " " + moveStr;
    } else {
        let idx = getFullMove(array);
        if (idx !== 1) moveStr = " " + getFullMove(array) + ". " + moveStr;
        else moveStr = getFullMove(array) + ". " + moveStr;
    }
    pgn.innerHTML += moveStr;
    historyMoves.appendChild(li);

    replacePieces(array);
    if (status !== 'normal') return;
    // start thinking
    if (isRobot[getPlayer(array)]) {
        setTimeout(am_make_move, 100);
    }
}

/**
 * @argument canvas {HTMLCanvasElement} 
 */
function canvasInit(canvas) {
    canvas.width = boardSize;
    canvas.height = boardSize;
    canvasClearHighlight(canvas);
    for (let i = 0; i < 8; i++) {
        const text = document.createElement('span')
        if (!rotated)
            text.innerHTML = String.fromCharCode('a'.charCodeAt(0) + i);
        else
            text.innerHTML = String.fromCharCode('h'.charCodeAt(0) - i);

        text.className = "board-label";
        text.style.bottom = `4px`;
        text.style.left = `${i * cellSize + 1}px`;
        text.style.color = (i & 1) ? darkBg : lightBg;
        canvasWrapper.appendChild(text);
    }
    
    for (let i = 0; i < 8; i++) {
        const text = document.createElement('span')
        if (!rotated)
            text.innerHTML = 8 - i;
        else
            text.innerHTML = i + 1;
        text.className = "board-label";
        text.style.top = `${i * cellSize + 1}px`;
        text.style.right = `1px`;
        text.style.color = (i & 1) ? darkBg : lightBg;
        canvasWrapper.appendChild(text);
    }
}
/**
 * @argument canvas {HTMLCanvasElement} 
 */
function canvasClearHighlight(canvas) {
    const context = canvas.getContext("2d");
    for (let i = 0; i < boardSize; i += cellSize) {
        for (let j = 0; j < boardSize; j += cellSize) {
            context.fillStyle = (((i + j) / cellSize) & 1) ? darkBg : lightBg;
            context.fillRect(j, i, cellSize, cellSize);
        }
    }
}
/**
 * @argument canvas {HTMLCanvasElement}
 * @argument highlightList {Uint16Array}
 */
function canvasHighlight(canvas, highlightList) {
    const context = canvas.getContext("2d");
    for (const move of highlightList) {
        const pos = (move & 0xfc0) >> 6;
        let i = Math.floor(pos / 8);
        let j = pos % 8;
        context.fillStyle = 'rgba(80, 180, 80, 0.8)';
        context.fillRect((rotated ? (7 - j) : j) * cellSize, (rotated ? i : (7 - i)) * cellSize, cellSize, cellSize);
    }
}



/**
 * @argument piece {int}
 * @returns {int}
 */
function getPieceColor(piece) {
    return piece >> 3;
}

const movement = {
    id: null,
    startMouseX: 0,
    startMouseY: 0,
    possibleMoves: []
}
function placePiece(piece, i, j) {
    const image = pieceSet[piece];
    const img = document.createElement("img");
    img.className = "piece";
    img.setAttribute("type", piece);
    const id = `piece-${i}-${j}`
    const pos = i * 8 + j;
    img.id = id
    img.src = image;
    img.width = cellSize;
    img.height = cellSize;
    img.draggable = false;
    img.style.left = `${(rotated ? (7 - j) : j) * cellSize}px`;
    img.style.top = `${(rotated ? i : (7 - i)) * cellSize}px`;
    img.addEventListener('mousedown', (e) => {
        if (isRobot[getPlayer(array)]) return;
        if (movement.id !== null) return;
        if (getPlayer(array) !== getPieceColor(piece)) return;
        if (!(pos in possibleMoves)) return;

        canvasHighlight(canvas, possibleMoves[pos]);

        movement.possibleMoves = possibleMoves[pos];

        movement.id = id;
        img.style.zIndex = 10;
        movement.startMouseX = e.clientX;
        movement.startMouseY = e.clientY;
    });
    canvasWrapper.appendChild(img);
}
/**
 * @argument array {Uint8Array}
 */
function placePieces(array) {
    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            const piece = getPiece(array, i, j);
            if (piece == 0) continue;
            placePiece(piece, i, j);
        }
    }
}
function replacePieces(array) {
    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            const piece = getPiece(array, i, j);
            const oldImg = document.getElementById(`piece-${i}-${j}`);

            // did not change (0 => 0 or same type)
            if (piece === 0 && oldImg === null || oldImg && piece === parseInt(oldImg.getAttribute("type"))) continue;

            if (oldImg !== null) canvasWrapper.removeChild(oldImg);
            if (piece === 0) continue;
            placePiece(piece, i, j)
        }
    }
}

window.onload = async () => {
    const { memory } = await wasm.default();
    chess_ptr = wasm.cb_new();
    const searchParams = new URLSearchParams(window.location.search);
    if (window.visualViewport.width < boardSize) {
        boardSize = window.visualViewport.width;
        cellSize = boardSize / 8;
    }

    const w = searchParams.get('white');
    if (w == 'human') {
        isRobot[0] = false;
    } else if (w === 'naive') {
        isRobot[0] = true;
        movers[0] = wasm.am_naive(chess_ptr);
    } else if (w === 'random') {
        isRobot[0] = true;
        movers[0] = wasm.am_random(chess_ptr);
    }
    const b = searchParams.get('black');
    if (b == 'human') {
        isRobot[1] = false;
    } else if (b === 'naive') {
        isRobot[1] = true;
        movers[1] = wasm.am_naive(chess_ptr);
    } else if (b === 'random') {
        isRobot[1] = true;
        movers[1] = wasm.am_random(chess_ptr);
    }
    rotated = isRobot[0] && !(isRobot[1]);
    const r = searchParams.get('rotated');
    if (r === 'true') {
        rotated = true;
    } else if (r === 'false') {
        rotated = false;
    }
    if (isRobot[0] && movers[0] === null) {
        movers[0] = wasm.am_naive(chess_ptr);
    }
    if (isRobot[1] && movers[1] === null) {
        movers[1] = wasm.am_naive(chess_ptr);
    }
    document.title="White move";
    canvas = document.querySelector("canvas");
    canvasWrapper = document.getElementById("canvas-wrapper");
    fen = document.getElementById("fen");
    pgn = document.getElementById("pgn");
    historyMoves = document.getElementById("history-moves");
    mask = document.getElementById("mask");

    selectionWindow.style.height = `${cellSize * 4}px`;
    selectionWindow.style.width = `${cellSize}px`;
    canvasInit(canvas);
    array = new Uint8Array(memory.buffer, 0, 36);
    array_ptr = array.byteOffset;

    wasm.serialize(chess_ptr, array_ptr);
    fen.value = toFEN(array);
    placePieces(array);
    fetch_status();
    if (isRobot[0]) {
        setTimeout(() => am_make_move(movers[0]), 1)
    }
}
document.body.addEventListener('mousemove', (e) => {
    if (movement.id === null) return;
    const deltaX = e.clientX - movement.startMouseX;
    const deltaY = e.clientY - movement.startMouseY;
    document.getElementById(movement.id).style.transform = `translate(${deltaX}px, ${deltaY}px)`;
})


document.body.addEventListener('mouseup', async (e) => {
    return await new Promise((res) => {
        if (movement.id === null) res();
        canvasClearHighlight(canvas);
        const [_, i, j] = movement.id.split("-");
        const deltaX = (rotated ? -1 : 1) * Math.round((e.clientX - movement.startMouseX) / cellSize);
        const deltaY = (rotated ? 1 : -1) * Math.round((e.clientY - movement.startMouseY) / cellSize);
        const element = document.getElementById(movement.id);
        element.style.transform = "";
        element.style.zIndex = 1;
        movement.id = null;
        const pos0 = parseInt(i) * 8 + parseInt(j);
        const pos1i = parseInt(i) + deltaY;
        const pos1j = parseInt(j) + deltaX;
        if (pos1i < 0 || pos1i >= 8 || pos1j < 0 || pos1j >= 8) {
            res();
        }
        const pos1 = pos1i * 8 + pos1j;
        if (pos1 == pos0) res();
        let moves = movement.possibleMoves.filter((pred) => ((pred & 0xfc0) >> 6) === pos1);
        if (moves.length === 0) {
            res();
        }
        // promoting directly to queen
        if (moves.length === 1) {
            movePiece(moves[0], false);

        } else if (moves.length > 1) {
            canvasWrapper.removeChild(e.target);
            mask.classList.remove("invisible");
            canvasWrapper.appendChild(selectionWindow);
            selectionWindow.style.left = `${pos1j * cellSize}px`;
            if (getPlayer(array) === 0) {
                selectionWindow.style.bottom = "unset";
                selectionWindow.style.top = "0";
                buttonQueen.style.backgroundImage = `url(${pieceSet[5]})`;
                buttonRook.style.backgroundImage = `url(${pieceSet[2]})`;
                buttonBishop.style.backgroundImage = `url(${pieceSet[4]})`;
                buttonKnight.style.backgroundImage = `url(${pieceSet[3]})`;

                // depending on 0, 1, 2, 3 is always consecutive in order of queen, rook, bishop and knight.
                buttonQueen.onclick = () => {
                    mask.classList.add("invisible");
                    canvasWrapper.removeChild(selectionWindow);
                    movePiece(moves[0], false);
                    res();
                }

                buttonRook.onclick = () => {
                    mask.classList.add("invisible");
                    canvasWrapper.removeChild(selectionWindow);
                    movePiece(moves[1], false);
                    res();
                }

                buttonBishop.onclick = () => {
                    mask.classList.add("invisible");
                    canvasWrapper.removeChild(selectionWindow);
                    movePiece(moves[2], false);
                    res();
                }

                buttonKnight.onclick = () => {
                    mask.classList.add("invisible");
                    canvasWrapper.removeChild(selectionWindow);
                    movePiece(moves[3], false);
                    res();
                }
            } else {
                selectionWindow.style.top = "unset";
                selectionWindow.style.bottom = "0";
                buttonQueen.style.backgroundImage = `url(${pieceSet[13]})`;
                buttonRook.style.backgroundImage = `url(${pieceSet[10]})`;
                buttonBishop.style.backgroundImage = `url(${pieceSet[12]})`;
                buttonKnight.style.backgroundImage = `url(${pieceSet[11]})`;

                // depending on 0, 1, 2, 3 is always consecutive in order of queen, rook, bishop and knight.
                buttonQueen.onclick = () => {
                    mask.classList.add("invisible");
                    canvasWrapper.removeChild(selectionWindow);
                    movePiece(moves[0], false);
                    res();
                }

                buttonRook.onclick = () => {
                    mask.classList.add("invisible");
                    canvasWrapper.removeChild(selectionWindow);
                    movePiece(moves[1], false);
                    res();
                }

                buttonBishop.onclick = () => {
                    mask.classList.add("invisible");
                    canvasWrapper.removeChild(selectionWindow);
                    movePiece(moves[2], false);
                    res();
                }

                buttonKnight.onclick = () => {
                    mask.classList.add("invisible");
                    canvasWrapper.removeChild(selectionWindow);
                    movePiece(moves[3], false);
                    res();
                }
            }
        }
        res();
    })
});


function fetch_status() {
    const ans = wasm.cb_get_possible_moves(chess_ptr);
    document.title = getPlayer(array) === 0 ? "White move" : "Black move";
    prevPossibleMoves = possibleMoves;
    if (ans[0] === 1) {
        if (ans.length === 1) {
            status = "checkmate";
            document.title = getPlayer(array) === 0 ? "Black wins!" : "White wins!";
            return 2; // 2 is checkmate, for displaying move
        } else {
            document.title = getPlayer(array) === 0 ? "White move, check" : "Black move, check";
        }
    }
    if (ans[0] === 0 && ans.length === 1) {
        status = "draw"
        document.title = "Draw!"
    }
    if (getHalfMove(array) >= 100) {
        status = "draw"
        document.title = "Draw!"
    }
    const moves = ans.slice(1);
    prevPossibleMoves = possibleMoves;
    possibleMoves = {}
    for (const move of moves) {
        const src = move & 0x3f;
        if (!(src in possibleMoves)) {
            possibleMoves[src] = [move]
        } else {
            possibleMoves[src].push(move);
        }
    }
    return ans[0]; // returning if checked.
}