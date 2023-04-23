import init, { World } from "snake_game";

init().then(_ => {
    // constants
    const CELL_SIZE = 20;    // 20px
    const WORLD_WIDTH = 8;

    const snakeSpawnIdx = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

    // World
    const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
    const worldWidth = world.get_width();
    // Canvas
    const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");

    canvas.height = worldWidth * CELL_SIZE;  // = 16*10px ->16px
    canvas.width = worldWidth * CELL_SIZE;

    function drawWorld() {
        ctx.beginPath();

        for (let x=0; x < worldWidth+1; x++) {
            ctx.moveTo(CELL_SIZE * x, 0);
            ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
        }

        for (let y=0; y < worldWidth+1; y++) {
            ctx.moveTo(0, CELL_SIZE * y);
            ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
        }

        ctx.stroke();
    }

    function drawSnake() {
        const snakeIdx = world.get_snake_head();
        const col = snakeIdx % worldWidth;
        const row = Math.floor(snakeIdx / worldWidth);

        ctx.beginPath();
        ctx.fillRect(
            CELL_SIZE * col, 
            CELL_SIZE * row,
            CELL_SIZE,
            CELL_SIZE);

        ctx.moveTo(col, row);

        ctx.stroke();
    }

    function paint() {
        drawWorld();
        drawSnake();
    }

    // every 100 milliseconds callback function is called
    function update() {
        const fps = 10;
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            drawWorld();
            drawSnake();
            world.update();
            requestAnimationFrame(update)
        }, 1000 / fps);
    }

    paint();
    update();
})

