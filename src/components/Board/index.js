import React, {Component} from "react";

class Board extends Component {
    constructor(props) {
        super(props);

        this.state = {
            rows: this.createBoard(props)
        }
    }


    createBoard = props => {
        let board = [];

        for(let i = 0; i < props.rows; i++) {
            board.push([]);
            for(let j = 0; j < props.columns; j++) {
                board[i].push({
                    x: j,
                    y: i,
                    count : 0,
                    isOpen: false,
                    hasMine: false,
                    hasFlag : false
                }); 
            }
        }

        // after creating the board we add the mines
        for(let i = 0; i < props.mines; i++) {
            let randomRow = Math.floor(Math.random()*props.rows);
            let randomCol = Math.floor(Math.random()*props.columns);


            let cell = board[randomRow][randomCol];
            if (cell.hasMine ) {
                i--;
            } else {
                cell.hasMine = true;
            }
            console.log(cell);
        }

    }

    render() {
        return(
            <div className="board">
                board
            </div>
        )
    }
}

export default Board;