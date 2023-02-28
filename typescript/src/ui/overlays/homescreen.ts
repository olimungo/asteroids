import Colors from '../colors';
import Fonts from '../fonts';
import Overlay from './overlay';

export default class Homescreen extends Overlay {
    private pastMillis: number = 0;
    private displayInsertCoin = true;

    draw() {
        this.p5.push();

        this.p5.fill(Colors.EDGE);
        this.p5.textAlign(this.p5.CENTER);

        this.p5.textSize(100);
        this.p5.text('ASTEROIDS', this.centerX, this.centerY - 100);

        this.p5.textFont(Fonts.getInstance().fontLight);
        this.p5.textSize(15);
        this.p5.text('by olimungo', this.centerX, this.centerY - 65);

        const now = this.p5.millis();

        if (now - this.pastMillis > 1000) {
            this.displayInsertCoin = !this.displayInsertCoin;
            this.pastMillis = now;
        }

        this.p5.textFont(Fonts.getInstance().fontThin);

        if (this.displayInsertCoin) {
            this.p5.textSize(40);
            this.p5.text('INSERT 1 COIN', this.centerX, this.centerY + 50);
        }

        this.p5.textFont(Fonts.getInstance().fontLight);
        this.p5.textSize(25);
        this.p5.text('PRESS "H" FOR HELP', this.centerX, this.centerY + 250);

        this.p5.pop();
    }
}
