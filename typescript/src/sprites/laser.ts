import P5 from 'p5';
import Asteroid from './asteroid';
import Colors from '../ui/colors';
import Sprite from './sprite';

export default class Laser extends Sprite {
    isOffScreen = false;

    constructor(p5: P5, position: P5.Vector, angle: number) {
        super(p5, position, 4);
        this.velocity = P5.Vector.fromAngle(angle).mult(10);
    }

    draw() {
        this.p5.push();

        this.p5.stroke(Colors.EDGE);
        this.p5.strokeWeight(4);
        this.p5.point(this.position.x, this.position.y);

        this.p5.pop();
    }

    checkWindowEdges(): boolean {
        this.isOffScreen = super.checkWindowEdges();

        return this.isOffScreen;
    }
}
