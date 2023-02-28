import Colors from '../colors';
import Fonts from '../fonts';
import Overlay from './overlay';

export default class Score extends Overlay {
    draw(score: number) {
        this.p5.push();

        this.p5.fill(Colors.EDGE);
        this.p5.textFont(Fonts.getInstance().fontLight);
        this.p5.textAlign(this.p5.RIGHT);
        this.p5.textSize(25);

        this.p5.text(score, this.p5.width - 50, 55);

        this.p5.pop();
    }
}
