const wasmImport = import("impossibletictactoe");
type wasmImportType = typeof import("impossibletictactoe");

enum CellState {
  X = -1,
  Empty = 0,
  O = 1,
}

enum GameState {
  Pending,
  Won,
  Lost,
}

let gameState = GameState.Pending;

const parseCellsToCellStateArray = (
  cells: NodeListOf<Element>
): CellState[] => {
  return Array.from(cells).map((cell) => {
    if (!cell.innerHTML) return CellState.Empty;
    if (cell.innerHTML === "X") return CellState.X;
    if (cell.innerHTML === "O") return CellState.O;
    throw new Error(`Unknown cell state ${cell.innerHTML}`);
  });
};

const hasWon = (cells: NodeListOf<Element>, player: "X" | "O"): boolean => {
  const arr = Array.from(cells).map((cell) => cell.innerHTML);
  // Diagonal
  if (arr[0] === player && arr[4] === player && arr[8] === player) return true;
  // Anti-Diagonal
  if (arr[2] === player && arr[4] === player && arr[6] === player) return true;
  // Row 1
  if (arr[0] === player && arr[1] === player && arr[2] === player) return true;
  // Row 2
  if (arr[3] === player && arr[4] === player && arr[5] === player) return true;
  // Row 3
  if (arr[6] === player && arr[7] === player && arr[8] === player) return true;
  // Column 1
  if (arr[0] === player && arr[3] === player && arr[6] === player) return true;
  // Column 2
  if (arr[1] === player && arr[4] === player && arr[7] === player) return true;
  // Column 3
  if (arr[2] === player && arr[5] === player && arr[8] === player) return true;

  return false;
};

const moveEnemy = (cells: NodeListOf<Element>, wasm: wasmImportType): void => {
  const res = wasm.move_enemy(
    new Int32Array(parseCellsToCellStateArray(cells))
  );
  cells[res].innerHTML = "O";
};

const onGameStateChange = (newGameState: GameState) => {
  console.log(Object.values(GameState)[newGameState]);
  gameState = newGameState;
};

const clickCallback = (cells: NodeListOf<Element>, wasm: wasmImportType) => {
  if (hasWon(cells, "X")) onGameStateChange(GameState.Won);

  moveEnemy(cells, wasm);
  if (hasWon(cells, "O")) onGameStateChange(GameState.Lost);
};

wasmImport.then((wasm) => {
  const cells = document.querySelectorAll(".cell");

  cells.forEach((cell, i) => {
    cell.addEventListener("click", (e) => {
      e.preventDefault();
      if (gameState !== GameState.Pending) {
        return console.log("Game already over!");
      }

      cells[i].innerHTML = "X";
      clickCallback(cells, wasm);
    });
  });

  console.log(wasm);
});
