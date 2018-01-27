public class Asteroid extends Sprite {
    private int sides = floor(random(5, 15));
    private float[] offsets = new float[sides];
    private float rotation = 0;
    private float rotationStep = map(random(1), 0, 1, -0.01, 0.01);

    Asteroid(PVector position, float radius) {
        super(position.x, position.y, radius * 0.5);
        this.init();
    }

    Asteroid() {
        super(random(width), random(height), floor(random(15, 50)));

        // As the position is randomly chosen, make sure that the asteroid is not
        // placed to close from the center of the screen, where the ship will be.
        PVector middle = new PVector(width / 2, height / 2);
        PVector position = PVector.sub(this.position, middle);

        if (position.mag() < 250) {
            position.setMag(250);
        }

        position.add(middle);

        this.position = position;

        this.init();
    }

    @Override
    void update() {
        super.update();
        this.rotation += this.rotationStep;
    }

    @Override
    void draw() {
        pushStyle();
        pushMatrix();
            noFill();
            
            translate(this.position.x, this.position.y); 

            rotate(this.rotation);

            beginShape();
                for (int i = 0; i < this.sides; i++) {
                    float radius = this.radius + this.offsets[i];
                    float angle = map(i, 0, this.sides, 0, TWO_PI);
                    float x = radius * cos(angle);
                    float y = radius * sin (angle);

                    float color1 = random(150, 255);
                    stroke(color1);
                    strokeWeight(1.4);
                    
                    vertex(x, y);
                }
            endShape(CLOSE);

            for (int i = 0; i < this.sides; i++) {
                float radius = this.radius + this.offsets[i];
                float angle = map(i, 0, this.sides, 0, TWO_PI);
                float x = radius * cos(angle);
                float y = radius * sin (angle);

                stroke(255, 35);

                ellipse(x, y, 2, 2);
            }
        popMatrix();
        popStyle();
    }

    ArrayList<Asteroid> breakup() {
        ArrayList<Asteroid> asteroids = new ArrayList<Asteroid>();

        asteroids.add(new Asteroid(this.position, this.radius));
        asteroids.add(new Asteroid(this.position, this.radius));

        return asteroids;
    }

    private void init() {
        this.velocity = PVector.random2D();

        for (int i = 0; i < this.sides; i++) {
            this.offsets[i] = random(-this.radius / 4, this.radius / 4);
        }
    }
}