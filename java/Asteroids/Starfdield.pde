public class Starfield {
    private final static int STARS_COUNT = 400;

    private ArrayList<Star> stars = new ArrayList<Star>();

    Starfield() {
        for (int counter = 0; counter < this.STARS_COUNT; counter++) {
            this.stars.add(new Star());
        }
    }

    void update() {
        for (Star star : this.stars) {
            star.update();
        }
    }

    void draw() {
        for (Star star : this.stars) {
            push();

            translate(width / 2, height / 2);

            star.draw();

            pop();
        }
    }
}