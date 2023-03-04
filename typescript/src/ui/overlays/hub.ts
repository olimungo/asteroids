import Colors from '../colors';
import Fonts from '../fonts';
import Overlay from './overlay';

export default class Hub extends Overlay {
    private frame = '';

    draw() {
        this.p5.push();

        this.p5.stroke(Colors.DARK);
        this.p5.noFill();
        this.p5.rect(0, 0, this.p5.width, this.p5.height);
        this.p5.line(0, this.centerY, this.p5.width, this.centerY);
        this.p5.line(this.centerX, 0, this.centerX, this.p5.height);
        this.p5.ellipse(this.centerX, this.centerY, 400, 400);

        if (this.p5.frameCount % 10 == 0 || this.p5.frameCount < 5) {
            this.frame = this.p5.frameRate().toFixed(0) + '';
        }

        this.p5.fill(Colors.DARK);
        this.p5.textFont(Fonts.fontLight);
        this.p5.textSize(30);
        this.p5.textAlign(this.p5.LEFT);
        this.p5.text(this.frame, 30, this.p5.height - 45);
        this.p5.text('FPS', 70, this.p5.height - 45);

        this.p5.pop();
    }
}
