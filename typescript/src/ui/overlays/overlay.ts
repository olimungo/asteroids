import P5 from 'p5';
import Colors from '../colors';

export default class Overlay {
    p5: P5;
    centerX: number;
    centerY: number;

    constructor(p5: P5) {
        this.p5 = p5;
        this.centerX = p5.width / 2;
        this.centerY = p5.height / 2;
    }

    draw(...args: any) {
        this.p5.push();

        this.p5.fill(Colors.WARNING);
        this.p5.textAlign(this.p5.CENTER);

        this.p5.textSize(50);
        this.p5.text('OVERLAY NOT YET DEFINED', this.centerX, this.centerY);

        this.p5.pop();
    }
}
