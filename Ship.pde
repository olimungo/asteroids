public class Ship extends Sprite {
    private float heading = 0;
    private float rotation = 0;
    private Boolean isBoosting = false;
    private ArrayList<Laser> lasers = new ArrayList<Laser>();
    private ArrayList<Asteroid> asteroids;

    Ship(ArrayList<Asteroid> asteroids) {
        super(width / 2, height / 2, 20);

        this.asteroids = asteroids;
    }
  
    @Override
    void update() {
        this.turn();

        if (this.isBoosting) {
            this.boost();
        }

        super.update();
        this.velocity.mult(0.99);
    }

    @Override
    void draw() {
        for (int i = this.lasers.size() - 1; i >= 0; i--) {
            Laser laser = this.lasers.get(i);
            laser.update();
            laser.draw();

            if (laser.hits(this.asteroids) || laser.isOffScreen) {
                this.lasers.remove(laser);
            }
        }

        pushStyle();
        pushMatrix();
            strokeWeight(1.4);

            translate(this.position.x, this.position.y);

            rotate(this.heading + PI / 2);

            float color1 = random(150, 255);
            float color2 = random(150, 255);
            float color3 = random(150, 255);

            stroke(color1);
            line(-this.radius / 5 * 4, this.radius, this.radius / 5 * 4, this.radius);
            stroke(255, 50);
            ellipse(this.radius / 5 * 4, this.radius, 2, 2);
            stroke(color2);
            line(this.radius / 5 * 4, this.radius, 0, -this.radius);
            stroke(255, 50);
            ellipse(0, -this.radius, 2, 2);
            stroke(color3);
            line(0, -this.radius, -this.radius / 5 * 4, this.radius);
            stroke(255, 50);
            ellipse(-this.radius / 5 * 4, this.radius, 2, 2);

        popMatrix();
        popStyle();
    }

    void setRotation(float angle) {
        this.rotation = angle;
    }

    void setBoost(Boolean value) {
        this.isBoosting = value;
    }

    void shoot() {
        this.lasers.add(new Laser(this.position, this.heading));
    }

    private void boost() {
        PVector force = PVector.fromAngle(this.heading);
        force.limit(.15);
        this.velocity.add(force);
    }

    private void turn() {
        this.heading += this.rotation;
    }
}
