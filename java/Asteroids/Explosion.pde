public class Explosion {
    ArrayList<Particle> particles = new ArrayList<Particle>();

    Explosion(PVector position) {
        for (int index = 0; index < 8; index++) {
            this.particles.add(
                new Particle(position.copy(), random(1, 4))
            );
        }
    }

    Boolean update() {
        for (int particleIndex = this.particles.size() - 1; particleIndex >= 0 ; particleIndex--) {
            Particle particle = this.particles.get(particleIndex);

            if (!particle.update()) {
                this.particles.remove(particleIndex);
            }
        }

        return this.particles.size() > 0;
    }

    void draw() {
        for (Particle particle : this.particles) {
            particle.draw();
        }
    }
}
