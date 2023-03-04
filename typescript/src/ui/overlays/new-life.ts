import Colors from '../colors';
import Fonts from '../fonts';
import Overlay from './overlay';

export default class NewLife extends Overlay {
    private pastMillis: number = 0;
    private displayNewLife = true;

    draw() {
        const now = this.p5.millis();

        if (now - this.pastMillis > 500) {
            this.displayNewLife = !this.displayNewLife;
            this.pastMillis = now;
        }

        if (this.displayNewLife) {
            this.p5.push();

            this.p5.textFont(Fonts.fontThin);
            this.p5.fill(Colors.EDGE);
            this.p5.textFont(Fonts.fontLight);
            this.p5.textAlign(this.p5.CENTER);
            this.p5.textSize(20);

            this.p5.text('NEW LIFE!', this.centerX, 85);

            this.p5.pop();
        }
    }
}
