import Colors from '../colors';
import Fonts from '../fonts';
import Overlay from './overlay';

export default class NextLife extends Overlay {
    private pastMillis: number = 0;
    private displayRetry = true;

    draw(lifes: number) {
        this.p5.push();

        this.p5.fill(Colors.EDGE);
        this.p5.textAlign(this.p5.CENTER);
        this.p5.textSize(60);

        let message = lifes + ' LIFE';

        if (lifes > 1) {
            message += 'S LEFT';
        } else {
            message += ' LEFT';
        }

        this.p5.text(message, this.p5.width / 2, this.p5.height / 2);

        const now = this.p5.millis();

        if (now - this.pastMillis > 1000) {
            this.displayRetry = !this.displayRetry;
            this.pastMillis = now;
        }

        if (this.displayRetry) {
            this.p5.textFont(Fonts.getInstance().fontLight);
            this.p5.textSize(25);

            this.p5.text(
                'PRESS "S" TO RE-TRY',
                this.p5.width / 2,
                this.p5.height / 2 + 50
            );
        }

        this.p5.pop();
    }
}
