import Colors from '../colors';
import Overlay from './overlay';

export default class Hub extends Overlay {
    draw() {
        this.p5.push();

        this.p5.stroke(Colors.DARK);
        this.p5.noFill();
        this.p5.rect(0, 0, this.p5.width, this.p5.height);
        this.p5.line(0, this.centerY, this.p5.width, this.centerY);
        this.p5.line(this.centerX, 0, this.centerX, this.p5.height);
        this.p5.ellipse(this.centerX, this.centerY, 400, 400);

        this.p5.pop();
    }
}
