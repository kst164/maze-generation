import React, { useEffect, useState } from 'react';
import Maze from './Maze';

const App: React.FC = () => {

  // We can get away without explicitly specifing a type but then the linter wouldn't know the type of {wasm}
  // This will initialize {wasm} as undefined
  const [wasm, setWasm] = useState();

  // Asyncronously load webassembly
  const loadWasm = async () => {
    try {
      const wasm = await import('maze-wasm');
      // Update state
      setWasm(wasm);
    } catch(err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  }

  // Same as component did mount, called when the component is first initialized
  useEffect(() => {
    loadWasm();
  }, []);

  const [rows, setRows] = useState(10);
  const [cols, setCols] = useState(15)

  // Since we need to load web assembly asynchronously, the {wasm} object won't be available right away
  // So we conditionally render a loading text until {wasm} is ready
  return (
    <div className="App">
      Warning: if you change rows or columns maze will reset. <br/>
      Rows: <input value={rows} type="number" onChange={event => setRows(+event.target.value)}/>
      Columns: <input value={cols} type="number" onChange={event => setCols(+event.target.value)}/>
      {wasm ? (
            <Maze wasm={wasm} rows={rows} cols={cols}/>
          ) : (
            <header className="App-header">
              <h1>
                Loading WASM...
              </h1>
            </header>
          )
      }
    </div>
  );
}

export default App;
