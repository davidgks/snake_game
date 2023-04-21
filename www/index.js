import init, { World } from "snake_game";

init().then(_ => {
    const cellSize = 20;    // 20px
    // World
    const world = World.new();
    const worldWidth = world.get_width();
    // Canvas
    const canvas = document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");

    canvas.height = worldWidth * cellSize;  // = 16*10px ->16px
    canvas.width = worldWidth * cellSize;

    function drawWorld() {
        ctx.beginPath();

        for (let x=0; x < worldWidth+1; x++) {
            ctx.moveTo(cellSize * x, 0);
            ctx.lineTo(cellSize * x, worldWidth * cellSize);
        }

        for (let y=0; y < worldWidth+1; y++) {
            ctx.moveTo(0, cellSize * y);
            ctx.lineTo(worldWidth * cellSize, cellSize * y);
        }

        ctx.stroke();
    }

    drawWorld();
    console.log(world.get_snake_head());
})

