import P5 from 'p5';
import Colors from '../colors';

const SIZE = 30;

export default class Keycap {
    private p5: P5;
    private position: P5.Vector;
    private angle: number;

    constructor(p5: P5, position: P5.Vector, angle: number = 0) {
        this.p5 = p5;

        this.position = position;
        this.angle = angle;
    }

    draw() {
        this.p5.push();

        this.p5.translate(this.position.x, this.position.y);

        this.p5.angleMode(this.p5.DEGREES);
        this.p5.rotate(this.angle);
        this.p5.angleMode(this.p5.RADIANS);

        this.p5.fill(Colors.BACKGROUND);
        this.p5.stroke(Colors.EDGE);
        this.p5.strokeWeight(1);

        this.p5.translate(-SIZE / 2, -SIZE / 2);

        this.p5.rect(0, 0, SIZE, SIZE, SIZE / 5);

        this.p5.fill(Colors.EDGE);
        this.p5.triangle(
            SIZE / 2,
            (SIZE / 10) * 3,
            (SIZE / 10) * 7,
            (SIZE / 10) * 7,
            (SIZE / 10) * 3,
            (SIZE / 10) * 7
        );

        this.p5.pop();
    }
}
