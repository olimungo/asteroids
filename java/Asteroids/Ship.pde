public class Ship extends Sprite {
    final static int SHIP_SIZE = 36;

    float heading = -PI / 2;
    Boolean isBoosting = false;
    Boolean fillShip;

    private ArrayList<Laser> lasers = new ArrayList<Laser>();
    private Interval boosterFlamesInterval;
    private Boolean switchFlames = false;

    Ship(PVector position, Boolean fillShip) {
        super(position, SHIP_SIZE, new PVector(0, 0), 0);

        this.fillShip = fillShip;
        this.heading = -PI / 2;
        this.boosterFlamesInterval = new Interval(150);
    }
  
    @Override
    void update() {
        this.heading += this.rotation;

        if (this.isBoosting) {
            PVector force = PVector.fromAngle(this.heading);
            force.limit(0.15);

            this.velocity.add(force);
        }

        this.velocity.limit(10);

        super.update();

        this.velocity.mult(0.995);

        for (int laserIndex = lasers.size() - 1; laserIndex >= 0; laserIndex--) {
            Laser laser = lasers.get(laserIndex);
            laser.update();

            if (laser.isOffScreen) {
                lasers.remove(laserIndex);
            }
        }
    }

    @Override
    void draw() {
        for (Laser laser : this.lasers) {
            laser.draw();
        }

        push();

        translate(this.position.x, this.position.y);
        rotate(this.heading + PI / 2);

        float smallerRadius = (this.diameter / 10) * 3.5;

        beginShape();

        if (this.fillShip) {
            fill(Colors.EDGE);
        } else {
            fill(Colors.BACKGROUND);
        }

        stroke(Colors.EDGE);
        strokeWeight(1.4);

        vertex(smallerRadius, this.diameter / 2);
        vertex(0, -this.diameter / 2);
        vertex(-smallerRadius, this.diameter / 2);

        bezierVertex(
            0,
            smallerRadius,
            0,
            smallerRadius,
            smallerRadius,
            this.diameter / 2
        );

        endShape();

        if (this.isBoosting) {
            if (this.boosterFlamesInterval.isElapsed()) {
                this.switchFlames = !this.switchFlames;
            }

            this.drawBoosterFlames();
        }

        pop();
    }

    void setRotation(float angle) {
        this.rotation = angle;
    }

    void setBoost(Boolean value) {
        this.isBoosting = value;
    }

    void shoot() {
        this.lasers.add(new Laser(this.position.copy(), this.heading));
    }

    private void drawBoosterFlames() {
        float smallerRadius = (this.diameter / 10) * 3.5;
        float marginX = this.diameter / 9;
        float marginY = this.diameter / 3;

        if (this.switchFlames) {
            line(
                smallerRadius - marginX,
                this.diameter / 2,
                0,
                this.diameter
            );

            line(
                -smallerRadius + marginX,
                this.diameter / 2,
                0,
                this.diameter
            );
        } else {
            line(
                smallerRadius - marginX,
                this.diameter / 2,
                0,
                this.diameter - marginY
            );

            line(
                -smallerRadius + marginX,
                this.diameter / 2,
                0,
                this.diameter - marginY
            );
        }
    }
}
