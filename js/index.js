import('../pkg/').then((module) => {
  var mazeWidth = 10;
  var mazeHeight = 10;

  function createBlankMaze() {

      var rowIndex, colIndex;

      var table = document.createElement("table");
      var tbody = document.createElement("tbody");

      for (rowIndex = 0; rowIndex < mazeHeight; rowIndex++) {

          var row = document.createElement("tr");

          for (colIndex = 0; colIndex < mazeWidth; colIndex++) {

              var col = document.createElement("td");
              col.style.backgroundColor = "rgb(255,255,255)";
              col.setAttribute("id", "cell_" + colIndex + "_" + rowIndex);

              col.style["border-right"] = "none";
              col.style["border-bottom"] = "none";

              if (rowIndex == mazeHeight - 1) {
                col.style["border-bottom"] = "1px solid black";
              }

              if (colIndex == mazeWidth - 1) {
                col.style["border-right"] = "1px solid black";
              }

              row.appendChild(col);

          }

          tbody.appendChild(row);

      }

      table.appendChild(tbody);

      document.getElementById("maze_container").appendChild(table);

  }

  createBlankMaze()

  var maze = module.Maze.new(mazeWidth, mazeHeight);
  var edge = maze.tick()

  function myLoop() {
    setTimeout(function() {
      var edge = maze.tick();
      if (edge) {
        var currentCell = document.getElementById("cell_" + edge.x + "_" + edge.y);
        currentCell.style["border-" + edge.direction.toLowerCase() + "-style"] = "hidden";
        myLoop();
      } else {
        myLoop();
      }
    }, 250)
  }

  myLoop();
});
