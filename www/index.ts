import init, { World } from "snake_game";

init().then(_ => {
    const cellSize = 20;    // 20px
    // World
    const world = World.new();
    const worldWidth = world.get_width();
    // Canvas
    const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
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

    function drawSnake() {
        const snakeIdx = world.get_snake_head();
        const col = snakeIdx % worldWidth;
        const row = Math.floor(snakeIdx / worldWidth);

        ctx.beginPath();
        ctx.fillRect(
            cellSize * col, 
            cellSize * row,
            cellSize,
            cellSize);

        ctx.moveTo(col, row);

        ctx.stroke();
    }

    function paint() {
        drawWorld();
        drawSnake();
    }

    // every 100 milliseconds callback function is called
    function update() {
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            drawWorld();
            drawSnake();
            world.update();
            requestAnimationFrame(update)
        }, 100);
    }

    paint();
    update();
})

