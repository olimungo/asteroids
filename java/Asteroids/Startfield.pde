public class Starfield {
    Star[] stars = new Star[400];

    Starfield() {
        for (int i = 0; i < stars.length; i++) {
            stars[i] = new Star();
        }
    }

    void draw() {
        for (int i = 0; i < stars.length; i++) {
            stars[i].update();

            pushMatrix();
                translate(width / 2, height / 2);
                stars[i].draw();
            popMatrix();
        }
    }
}