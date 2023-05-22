import init,{getState,checkWin,isGameOver,openField,toggleFlag,createNewBoard} from './pkg/mirelsweeper.js'


async function main() {
    await init();
    // adding the  functions to an object in the globalThis so 
    // we can test them throw the console
    globalThis.mirel = Object;
    globalThis.mirel.checkWin = checkWin;
    globalThis.mirel.getState = getState;
    globalThis.mirel.isGameOver = isGameOver;
    globalThis.mirel.openField = openField;
    globalThis.mirel.toggleFlag = toggleFlag;
    globalThis.mirel.createNewBoard = createNewBoard;
}
//this will work
main();