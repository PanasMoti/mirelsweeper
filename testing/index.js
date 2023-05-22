import init,{getState,checkWin,isGameOver,openField,toggleFlag,createNewBoard} from './pkg/mirelsweeper.js'


async function main() {
    await init();
    // adding the  functions to an object in the globalThis so 
    // we can test them throw the console
    globalThis.minesweeper = Object;
    globalThis.minesweeper.checkWin = checkWin;
    globalThis.minesweeper.getState = getState;
    globalThis.minesweeper.isGameOver = isGameOver;
    globalThis.minesweeper.openField = openField;
    globalThis.minesweeper.toggleFlag = toggleFlag;
    globalThis.minesweeper.createNewBoard = createNewBoard;
}
//this will work
main();