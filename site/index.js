const js = import("./node_modules/impossibletictactoe/impossibletictactoe");

js.then((js) => {
  const cells = document.querySelectorAll(".cell");
  const onGameFinish = (reason) => {
    alert(reason);
    location.reload();
    return true;
  }

  const checkGameStatus = () => {
    const gameStatus = js.get_game_status();
    if(gameStatus === "WON") return onGameFinish("WON!")
    else if(gameStatus === "LOST") return onGameFinish("LOST!")
    else if (gameStatus === "PENDING") console.log("Game pending!")
  }


  const makeMove = () => {
    const nextMove = js.make_move();
    console.log(`Next move: ${nextMove}`);
    if(nextMove === "EOG") throw new Error("End of the game");
    cells[nextMove].innerHTML = "O";
    checkGameStatus();

  };

  cells.forEach((cell, index) => {
    cell.addEventListener("click", () => {
      const res = js.on_cell_click(index.toString());
      console.log(`Returned from rust ${res}`);
      if (res === "OK") {
        cell.innerHTML = "X";
        const gs = checkGameStatus();
        if (gs === true) return;
        makeMove();

      } else {
        alert(res);
      }
    });
  });
});
