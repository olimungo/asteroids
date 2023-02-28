import P5 from 'p5';
import Star from './star';

const STARS_COUNT = 400;

export default class Starfield {
    p5: P5;
    stars: Star[] = [];

    constructor(p5: P5) {
        this.p5 = p5;

        for (let counter = 0; counter < STARS_COUNT; counter++) {
            this.stars.push(new Star(p5));
        }
    }

    update() {
        for (let star of this.stars) {
            star.update();
        }
    }

    draw() {
        for (let star of this.stars) {
            this.p5.push();

            this.p5.translate(this.p5.width / 2, this.p5.height / 2);

            star.draw();

            this.p5.pop();
        }
    }
}
