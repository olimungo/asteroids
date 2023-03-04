import Colors from '../colors';
import Fonts from '../fonts';
import Overlay from './overlay';

export default class NextLevel extends Overlay {
    private pastMillis: number = 0;
    private displayNextLevel = true;

    draw(level: number) {
        this.p5.push();

        this.p5.fill(Colors.EDGE);
        this.p5.textAlign(this.p5.CENTER);
        this.p5.textSize(60);

        this.p5.text(
            'LEVEL ' + level + ' COMPLETED',
            this.centerX,
            this.centerY
        );

        const now = this.p5.millis();

        if (now - this.pastMillis > 1000) {
            this.displayNextLevel = !this.displayNextLevel;
            this.pastMillis = now;
        }

        if (this.displayNextLevel) {
            this.p5.textFont(Fonts.fontLight);
            this.p5.textSize(25);

            this.p5.text(
                'PRESS "S" TO GO TO THE NEXT LEVEL',
                this.centerX,
                this.centerY + 50
            );
        }

        this.p5.pop();
    }
}
