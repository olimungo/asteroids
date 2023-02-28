import P5 from 'p5';
import Colors from '../colors';
import Fonts from '../fonts';

const WIDTH = 200;
const HEIGHT = 30;

export default class Spacebar {
    private p5: P5;
    private position: P5.Vector;

    constructor(p5: P5, position: P5.Vector) {
        this.p5 = p5;
        this.position = position;
    }

    draw() {
        this.p5.push();

        this.p5.translate(this.position.x, this.position.y);

        this.p5.fill(Colors.BACKGROUND);
        this.p5.stroke(Colors.EDGE);
        this.p5.strokeWeight(1);

        this.p5.translate(-WIDTH / 2, -HEIGHT / 2);

        this.p5.rect(0, 0, WIDTH, HEIGHT, HEIGHT / 5);

        this.p5.fill(Colors.EDGE);
        this.p5.noStroke();
        this.p5.textFont(Fonts.getInstance().fontLight);
        this.p5.textAlign(this.p5.CENTER);
        this.p5.text('SPACEBAR', WIDTH / 2, HEIGHT / 2 + 5);

        this.p5.pop();
    }
}
