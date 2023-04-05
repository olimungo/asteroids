import { Game } from 'asteroids';

let renderLoopTimeOutId = null;
let renderLoopId = null;
let game = Game.new();

renderLoop();

function renderLoop() {
    game.tick();

    renderLoopTimeOutId = setTimeout(() => {
        renderLoopId = requestAnimationFrame(renderLoop);
    }, 0);
}

document.addEventListener('keydown', (event) => {
    game.key_pressed(event.key);
});

document.addEventListener('keyup', (event) => {
    game.key_released(event.key);
});
