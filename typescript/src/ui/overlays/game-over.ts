import Colors from '../colors';
import Fonts from '../fonts';
import Overlay from './overlay';

export default class GameOver extends Overlay {
    private pastMillis: number = 0;
    private displayPlayAgain = true;

    draw() {
        this.p5.push();

        this.p5.fill(Colors.EDGE);
        this.p5.textAlign(this.p5.CENTER);

        this.p5.textSize(80);
        this.p5.text('GAME OVER', this.centerX, this.centerY);

        const now = this.p5.millis();

        if (now - this.pastMillis > 1000) {
            this.displayPlayAgain = !this.displayPlayAgain;
            this.pastMillis = now;
        }

        if (this.displayPlayAgain) {
            this.p5.textFont(Fonts.fontLight);

            this.p5.textSize(25);
            this.p5.text(
                'PRESS "S" TO PLAY AGAIN',
                this.centerX,
                this.centerY + 50
            );
        }

        this.p5.pop();
    }
}
