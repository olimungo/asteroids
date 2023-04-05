public class Ship extends Sprite {
    private final static int SHIP_SIZE = 36;
    private final static int BOOSTER_INTERVAL = 150;

    float heading;
    Boolean isBoosting = false;
    Boolean fillShip;

    private ArrayList<Laser> lasers = new ArrayList<Laser>();
    private Interval boosterFlamesInterval;
    private Boolean switchFlames = false;

    Ship(PVector position, Boolean fillShip) {
        super(position, SHIP_SIZE, new PVector(0, 0), 0);

        this.fillShip = fillShip;
        this.heading = -PI / 2;
        this.boosterFlamesInterval = new Interval();
        this.boosterFlamesInterval.set(BOOSTER_INTERVAL);
    }
  
    Boolean update() {
        this.heading += this.rotation;

        if (this.isBoosting) {
            PVector force = PVector.fromAngle(this.heading);
            force.limit(0.15);

            this.velocity.add(force);
        }

        this.velocity.limit(10);
        this.velocity.mult(0.995);

        super.update();

        for (int laserIndex = this.lasers.size() - 1; laserIndex >= 0; laserIndex--) {
            Laser laser = this.lasers.get(laserIndex);
            laser.update();

            if (laser.isOffScreen) {
                lasers.remove(laserIndex);
            }
        }

        return true;
    }

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

    Boolean lasersHit(Sprite sprite) {
        for (int laserIndex = this.lasers.size() - 1; laserIndex >= 0; laserIndex--) {
            Laser laser = this.lasers.get(laserIndex);

            if (laser.collideWith(sprite)) {
                this.lasers.remove(laserIndex);
                return true;
            }
        }

        return false;
    }

    ArrayList<Potatoid> breakUp() {
        ArrayList<Potatoid> potatoids = new ArrayList<Potatoid>();
        float[][] data = {{0.9, 0.05, 5}, {0.9, -0.08, 6}, {0.9, 0.15, 3}, {0.7, -0.1, 5},};

        for(float[] element : data) {
            Potatoid potatoid = new Potatoid(
                this.position.copy(),
                this.diameter * element[0],
                PVector.random2D(),
                element[1],
                int(element[2])
            );

            potatoids.add(potatoid);
        }

        return potatoids;
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
