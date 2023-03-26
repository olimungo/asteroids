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
