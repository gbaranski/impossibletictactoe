const js = import("./node_modules/impossibletictactoe/impossibletictactoe");

js.then((js) => {
  const cells = document.querySelectorAll(".cell");
  const makeMove = () => {
    const nextMove = js.make_move();
    console.log(`Next move: ${nextMove}`);
    if(nextMove === "EOG") throw new Error("End of the game");
    cells[nextMove].innerHTML = "O";

  };

  cells.forEach((cell, index) => {
    cell.addEventListener("click", () => {
      const res = js.on_cell_click(index.toString());
      console.log(`Returned from rust ${res}`);
      if (res === "OK") {
        cell.innerHTML = "X";
        makeMove();
      } else {
        alert(res);
      }
    });
  });
});
