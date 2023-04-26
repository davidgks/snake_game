import init, { World, Directions} from "snake_game";
import {rnd} from "./utils/rnd";

init().then(wasm => {
    // constants
    const CELL_SIZE = 20;    // 20px
    const WORLD_WIDTH = 4;
    const snakeSpawnIdx = rnd(WORLD_WIDTH * WORLD_WIDTH);

    // World
    const world = World.new(WORLD_WIDTH, snakeSpawnIdx);
    const worldWidth = world.get_width();
    // Canvas
    const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
    const ctx = canvas.getContext("2d");

    canvas.height = worldWidth * CELL_SIZE;  // = 16*10px ->16px
    canvas.width = worldWidth * CELL_SIZE;

    const snakeCellPtr = world.snake_cells();
    const snakeLength = world.snake_length();
    
    const snakeCells = new Uint32Array( 
        wasm.memory.buffer,
        snakeCellPtr,
        snakeLength,
    )


    // Specify events you wanna listen to and respective callback function that should be executed when events occurs
    document.addEventListener("keydown", event => {
        switch(event.code) {
            case "ArrowUp":
                world.change_snake_direction(Directions.Up);
                console.log("switch direction to up!");
                break;
            case "ArrowRight":
                world.change_snake_direction(Directions.Right);
                console.log("switch direction to right!");
                break;
            case "ArrowDown":
                world.change_snake_direction(Directions.Down);
                console.log("switch direction to down!");
                break;
            case "ArrowLeft":
                world.change_snake_direction(Directions.Left);
                console.log("switch direction to left!");
                break;
        }
    })

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

    function drawReward() {
        const rewardCell = world.reward_cell();
        const col = rewardCell % worldWidth;
        const row = Math.floor(rewardCell / worldWidth);

        ctx.beginPath();
        ctx.fillStyle = "#FF0000";
        ctx.fillRect(
            CELL_SIZE * col, 
            CELL_SIZE * row,
            CELL_SIZE,
            CELL_SIZE
        );
        ctx.stroke();

        if (rewardCell === 1000) {
            alert("You won the game!");
        };
    }

    function drawSnake() {
        // Get snake cells
        const snakeCells = new Uint32Array(
            wasm.memory.buffer,
            world.snake_cells(),
            world.snake_length(),
        );
        console.log(snakeCells);
        snakeCells.forEach((cell, i) => {
            const col = cell % worldWidth;
            const row = Math.floor(cell / worldWidth);

            ctx.beginPath();
            ctx.fillStyle = i === 0 ? "#7878db" : "#000000";
            ctx.fillRect(
                CELL_SIZE * col, 
                CELL_SIZE * row,
                CELL_SIZE,
                CELL_SIZE
            );
            ctx.moveTo(col, row);
            ctx.stroke();
        })

        const snakeIdx = world.get_snake_head();
        const col = snakeIdx % worldWidth;
        const row = Math.floor(snakeIdx / worldWidth);

        ctx.beginPath();
        ctx.fillStyle = "#7878db";
        ctx.fillRect(
            CELL_SIZE * col, 
            CELL_SIZE * row,
            CELL_SIZE,
            CELL_SIZE
        );
        ctx.moveTo(col, row);
        ctx.stroke();
    }

    function paint() {
        drawWorld();
        drawSnake();
        drawReward();
    }

    // every 100 milliseconds callback function is called
    function update() {
        const fps = 2;
        setTimeout(() => {
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            paint();
            world.step();
            requestAnimationFrame(update)
        }, 1000 / fps);
    }

    paint();
    update();
})

