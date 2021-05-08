import React from 'react';
import './Maze.css';
import * as Wasm from 'maze-wasm'; // For types only

interface Paths {
  up: boolean,
  down: boolean,
  left: boolean,
  right: boolean,
}

interface SquareProps {
  onClick: () => void,
  isPath: boolean,
  isSelected: boolean,
  paths: Paths,
}

function Square(props: SquareProps) {
  let classes = "square";
  if (props.isPath) {
    classes += " path";
  }
  if (props.isSelected) {
    classes += " selected";
  }

  if (props.paths.up) {
    classes += " path-up";
  }
  if (props.paths.down) {
    classes += " path-down";
  }
  if (props.paths.left) {
    classes += " path-left";
  }
  if (props.paths.right) {
    classes += " path-right";
  }

  return(
    <button className={classes} onClick={props.onClick}>
    </button>
  )
}

interface MazeProps {
  wasm: typeof Wasm,
  rows: number,
  cols: number,
}

interface MazeState {
  maze: Wasm.Maze,
  theChosenOne: [number, number] | null,
}

class Maze extends React.Component<MazeProps, MazeState> {
  constructor(props: MazeProps) {
    super(props);
    this.state = {
      maze: this.props.wasm.Maze.new(props.rows, props.cols),
      theChosenOne: null,
    };
  }

  componentDidUpdate(prevProps: MazeProps) {
    if (prevProps.rows !== this.props.rows || prevProps.cols !== this.props.cols) {
      this.setState({
        maze: this.props.wasm.Maze.new(this.props.rows, this.props.cols),
        theChosenOne: null,
      })
    }
  }

  handleClick(row: number, col: number) {
    if (this.state.theChosenOne === null) {
      this.setState({
        theChosenOne: [row, col],
      });
      return;
    }

    const [prevRow, prevCol] = this.state.theChosenOne;

    let dir: Wasm.Direction | null = null;
    let squares = [];
/*
    if (prevRow === row) {
      if (prevCol === col - 1) {
        dir = this.props.wasm.Direction.Left;
      } else if (prevCol === col + 1) {
        dir = this.props.wasm.Direction.Right;
      }
    } else if (prevCol === col) {
      if (prevRow === row - 1) {
        dir = this.props.wasm.Direction.Up;
      } else if (prevRow === row + 1) {
        dir = this.props.wasm.Direction.Down;
      }
    }
*/
    if (prevRow === row) {
      dir = this.props.wasm.Direction.Right;

      const smaller = prevCol > col ? col : prevCol;
      const bigger = prevCol > col ? prevCol : col;

      for (let c = smaller; c < bigger; c++) {
        squares.push([row, c]);
      }
    } else if (prevCol === col) {
      dir = this.props.wasm.Direction.Down;

      const smaller = prevRow > row ? row : prevRow;
      const bigger = prevRow > row ? prevRow : row;

      for (let i = smaller; i < bigger; i++) {
        squares.push([i, col]);
      }
    }

    if (dir !== null) {
      const new_maze = this.state.maze.clone_maze();
      const d: Wasm.Direction = dir;
      squares.forEach(([r, c]) => {
        new_maze.toggle_wall(r, c, d);
      });
      this.setState({
        maze: new_maze,
      });
    }

    this.setState({
      theChosenOne: null,
    })
  }

  getPaths(row: number, col: number): Paths {
    return {
      up: this.state.maze.can_go(row, col, this.props.wasm.Direction.Up),
      down: this.state.maze.can_go(row, col, this.props.wasm.Direction.Down),
      left: this.state.maze.can_go(row, col, this.props.wasm.Direction.Left),
      right: this.state.maze.can_go(row, col, this.props.wasm.Direction.Right),
    }
  }

  renderSquare(row: number, col: number) {
    return <Square
      onClick={() => this.handleClick(row, col)}
      isPath={this.state.maze.is_path(row, col)}
      isSelected={this.state.theChosenOne !== null 
               && this.state.theChosenOne[0] === row
               && this.state.theChosenOne[1] === col}
      paths={this.getPaths(row, col)}
      key={row + ", " + col}
    />
  }

  renderRow(rowNumber: number) {
    const row = Array(this.props.cols);
    for (let colNumber = 0; colNumber < this.props.cols; colNumber++) {
      row.push(this.renderSquare(rowNumber, colNumber));
    }
    return(
      <div className="maze-row" key={rowNumber}>
        {row}
      </div>
    )
  }

  render() {
    const rows = Array(this.props.rows).fill(null).map((_, rowNumber) => this.renderRow(rowNumber));

    return(
      <div className="maze">
        {rows}
      </div>
    )
  }
}

export default Maze;
