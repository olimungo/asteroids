public class Hub extends Overlay {
    @Override
    void draw() {
        push();

        stroke(Colors.DARK);
        noFill();

        rect(0, 0, width, height);

        line(0, this.centerY, width, this.centerY);
        line(this.centerX, 0, this.centerX, height);

        ellipse(this.centerX, this.centerY, 400, 400);

        pop();
    }
}