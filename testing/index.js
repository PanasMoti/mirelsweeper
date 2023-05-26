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
    render();
    globalThis.mirel.render = render;
}

function render() {
    let root = document.getElementById("root"); // the root
    let board = document.getElementById("board");
    board.innerText = "";
    let data = getState().split("\n").map(row => row.trim().split(/\s+/)); // break the state to a 2d array
    data.pop(); // removing the last empty row that is added by accident
    board.style.display = "inline-grid";
    board.style.gridTemplate = `repeat(${data.length},auto) / repeat(${data[0].length},auto)`;
    if(!isGameOver() || true) {
        if(checkWin()) {
            board.innerText = "YOU WON";
        } else {
            for(let y = 0; y < data.length; y++) {
                for(let x = 0; x < data[y].length; x++) {
                    const cell = document.createElement("div");
                    cell.classList.add('cell');
                    if(data[y][x] == '🚩') {
                        cell.style.backgroundImage = "url('assets/images/flag.png')";
                    } else if (data[y][x] == '💣') {
                        cell.style.backgroundImage = "url('assets/images/bomb.png')";
                    } else if (data[y][x] == '🟩') {
                        cell.style.backgroundImage = "url('assets/images/unopened.png')";
                    } else if (data[y][x] == '0') {
                        cell.style.backgroundImage = "url('assets/images/empty.png')";
                    } else {
                        let i = parseInt(data[y][x]);
                        let img = `url('assets/images/${i}.png')`;
                        cell.style.backgroundImage = img;
                    }
                    // cell.innerText = data[y][x];
                    cell.addEventListener("click",evt => {
                        evt.preventDefault();
                        console.log(x,y);
                        openField(x,y);
                        render();
                    })
                    cell.addEventListener("contextmenu",evt => {
                        evt.preventDefault();
                        console.log(x,y);
                        toggleFlag(x,y);
                        render(); 
                    })
                    board.appendChild(cell);
                }
            }    
        }
            
    } else {
        board.innerText = "YOU LOST"
    }
        console.log(data);
}
//this will work
main();