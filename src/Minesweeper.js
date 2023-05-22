import React,{Component} from "react";
import Board from "./components/Board";

class Minesweeper extends Component {

    state = {
        rows: 10,
        columns: 10,
        flags : 10,
        mines: 10
    };

    render() {
        return (
            <div className="minesweeper">
                <h1>MirelSweeper</h1>
                <Board rows={this.state.rows} columns={this.state.columns} mines={this.state.mines}/>
            </div>
        )
    }
}

export default Minesweeper;