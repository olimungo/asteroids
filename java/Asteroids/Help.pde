public class Help extends Overlay {
    private Keycap keycapLeft;
    private Keycap keycapRight;
    private Keycap keycapUp;
    private Spacebar spacebar;

    Help() {
        this.keycapLeft = new Keycap(new PVector(650, 285), -90);
        this.keycapRight = new Keycap(new PVector(770, 285), 90);
        this.keycapUp = new Keycap(new PVector(710, 215), 0);
        this.spacebar = new Spacebar(new PVector(710, 365));
    }

    void draw() {
        push();

        fill(Colors.BACKGROUND);
        stroke(Colors.EDGE);
        strokeWeight(2);
        rect(150, 150, width - 300, height - 300, 10);

        fill(Colors.EDGE);
        noStroke();
        textFont(Fonts.fontLight);
        textAlign(CENTER);
        textSize(35);
        text("HELP", this.centerX, 210);

        textAlign(LEFT);
        textSize(15);
        text("INSERT COIN", 220, 275);
        text("PAUSE", 220, 325);
        text("HUB", 220, 375);
        text("SHOW EGDES", 220, 425);
        text("STARFIELD", 220, 475);

        textAlign(RIGHT);
        text("S", 475, 275);
        text("P", 475, 325);
        text("U", 475, 375);
        text("E", 475, 425);
        text("X", 475, 475);

        this.keycapLeft.draw();
        this.keycapRight.draw();
        this.keycapUp.draw();

        textSize(15);
        textAlign(CENTER);
        text("BOOST", 710, 250);
        text("TURN LEFT", 650, 320);
        text("TURN RIGHT", 770, 320);
        text("SHOOT", 710, 400);

        textAlign(LEFT);
        text("ASTEROID", 600, 450);
        text("UFO", 600, 475);
        text("NEW LIFE", 600, 500);

        text("10 POINTS", 745, 450);
        text("50 POINTS", 745, 475);
        text("3000 POINTS", 745, 500);

        this.spacebar.draw();

        pop();
    }
}