public class Overlay {
    float centerX;
    float centerY;

    Overlay() {
        this.centerX = width / 2;
        this.centerY = height / 2;
    }

    void draw() {
        push();

        fill(Colors.WARNING);
        textAlign(CENTER);

        textSize(50);
        text("OVERLAY NOT YET DEFINED", this.centerX, this.centerY);

        pop();
    }
}