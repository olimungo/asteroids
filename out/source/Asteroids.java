import processing.core.*; 
import processing.data.*; 
import processing.event.*; 
import processing.opengl.*; 

import java.util.HashMap; 
import java.util.ArrayList; 
import java.io.File; 
import java.io.BufferedReader; 
import java.io.PrintWriter; 
import java.io.InputStream; 
import java.io.OutputStream; 
import java.io.IOException; 

public class Asteroids extends PApplet {

ArrayList<Asteroid> asteroids;
PFont font;
Helpers helpers;
Boolean slowFrameRate = false;

float middleWidth = 1000 / 2;
float middleHeight = 800 / 2;

Starfield starfield;
Ship ship;

public void pre() {
}

public void setup() {
    
    //fullScreen();
    // frameRate(5);

    helpers = new Helpers();

    font = loadFont("HelveticaNeue-48.vlw");
    textFont(font, 48);

    starfield = new Starfield();
    asteroids = new ArrayList<Asteroid>();

    for (int i = 0; i < 10; i++) {
        asteroids.add(new Asteroid());
    }

    ship = new Ship(asteroids);
}

public void draw() {
    background(0);

    helpers.translateSketch(1.5f);
    helpers.drawPattern();

    starfield.draw();

    for (Asteroid asteroid : asteroids) {
        asteroid.update();
        asteroid.draw();
    }

    ship.update();
    ship.draw();

    helpers.showFrameRate();
}

public void mousePressed() {
    if (slowFrameRate) {
        frameRate(60);
    } else {
        frameRate(10);
    }

    slowFrameRate = !slowFrameRate;
    noLoop();
}

public void mouseReleased() {
    // frameRate(60);
    loop();
}

public void keyPressed() {
    if (keyCode == LEFT) {
        ship.setRotation(-0.1f);
    } else if (keyCode == RIGHT) {
        ship.setRotation(0.1f);
    } else if (keyCode == UP) { // SPACE 
        ship.setBoost(true);
    } else if (keyCode == 32) {
        ship.shoot();
    }
}

public void keyReleased() {
    if (keyCode == UP) {
        ship.setBoost(false);
    } else {
        ship.setRotation(0);
    }
}
public class Asteroid extends Sprite {
    private int sides = floor(random(5, 15));
    private float[] offsets = new float[sides];

    Asteroid(PVector position, float radius) {
        super(position.x, position.y, radius * 0.5f);
        this.velocity = PVector.random2D();
        this.calcOffsets();
    }

    Asteroid() {
        super(random(width), random(height), floor(random(15, 50)));
        this.velocity = PVector.random2D();
        this.calcOffsets();
    }

    public @Override
    void draw() {
        pushStyle();
        pushMatrix();
            noFill();
            
            translate(this.position.x, this.position.y); 

            beginShape();
                for (int i = 0; i < this.sides; i++) {
                    float radius = this.radius + this.offsets[i];
                    float angle = map(i, 0, this.sides, 0, TWO_PI);
                    float x = radius * cos(angle);
                    float y = radius * sin (angle);

                    float color1 = random(150, 255);
                    stroke(color1);
                    strokeWeight(1.4f);
                    
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

    public ArrayList<Asteroid> breakup() {
        ArrayList<Asteroid> asteroids = new ArrayList<Asteroid>();

        asteroids.add(new Asteroid(this.position, this.radius));
        asteroids.add(new Asteroid(this.position, this.radius));

        return asteroids;
    }

    private void calcOffsets() {
        for (int i = 0; i < this.sides; i++) {
            this.offsets[i] = random(-this.radius / 4, this.radius / 4);
        }
    }
}
public class Helpers {
    float middleWidth = width / 2;
    float middleHeight = height / 2;
    String frameRateMessage = "";

    public void translateSketch(float ratio) {
        scale(1/ratio);
        translate((width * ratio - width) / 2, (height * ratio - height) / 2);
    }

    public void drawPattern() {
        stroke(255);
        noFill();
        rect(0, 0, width, height);
        line(0, middleHeight, width, middleHeight);
        line(middleWidth, 0, middleWidth, height);
        ellipse(middleWidth, middleHeight, 400, 400);
    }

    public void showFrameRate() {
        if (frameCount % 5 == 0 || frameCount < 5) {
            this.frameRateMessage = String.format("%2.0f / %d / %d / %d + %d", frameRate, frameCount, millis(), mouseX, mouseY);
        }

        fill(255, 255, 255, 100);
        textSize(30);
        textAlign(LEFT);
        text(this.frameRateMessage, 30, height - 40);
    }
}
public class Laser extends Sprite {
    Boolean isOffScreen = false;

    Laser(PVector position, float angle) {
        super(position.x, position.y, 0);
        this.velocity = PVector.fromAngle(angle).mult(10);
    }

    public @Override
    void draw() {
        pushStyle();
        pushMatrix();
            stroke(255);
            strokeWeight(4);
            point(this.position.x, this.position.y);
        popMatrix();
        popStyle();
    }

    public @Override
    void checkEdges() {
        if (this.position.x > width + this.radius) {
            this.isOffScreen = true; 
        } else if (this.position.x < -this.radius) {
            this.isOffScreen = true; 
        }

        if (this.position.y > height + this.radius) {
            this.isOffScreen = true; 
        } else if (this.position.y < -this.radius) {
            this.isOffScreen = true; 
        }
    }

    public Boolean hits(ArrayList<Asteroid> asteroids) {
        Boolean hit = false;

        for (int i = asteroids.size() - 1; i >= 0; i--) {
            Asteroid asteroid = asteroids.get(i);
            float distance = dist(this.position.x, this.position.y, asteroid.position.x, asteroid.position.y);

            if (distance < asteroid.radius) {
                if (asteroid.radius > 15) {
                    asteroids.addAll(asteroid.breakup());
                }

                asteroids.remove(asteroid);
                hit = true;
            }
        }

        return hit;
    }
}
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
  
    public @Override
    void update() {
        this.turn();

        if (this.isBoosting) {
            this.boost();
        }

        super.update();
        this.velocity.mult(0.99f);
    }

    public @Override
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
            strokeWeight(1.4f);

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

    public void setRotation(float angle) {
        this.rotation = angle;
    }

    public void setBoost(Boolean value) {
        this.isBoosting = value;
    }

    public void shoot() {
        this.lasers.add(new Laser(this.position, this.heading));
    }

    private void boost() {
        PVector force = PVector.fromAngle(this.heading);
        force.limit(.15f);
        this.velocity.add(force);
    }

    private void turn() {
        this.heading += this.rotation;
    }
}
public class Sprite {
    PVector position;
    PVector velocity;
    float radius;

    Sprite(float x, float y, float radius) {
        this.position = new PVector(x, y);
        this.velocity = new PVector();
        this.radius = radius;
    }

    public void update() {
        this.position.add(this.velocity);
        this.checkEdges();
    }

    public void draw() {
        fill(255);
        noStroke();
        ellipse(this.position.x, this.position.y, this.radius, this.radius);
    }

    public void checkEdges() {
        if (this.position.x > width + this.radius) {
            this.position.x = -this.radius; 
        } else if (this.position.x < -this.radius) {
            this.position.x = width + this.radius;
        }

        if (this.position.y > height + this.radius) {
            this.position.y = -this.radius; 
        } else if (this.position.y < -this.radius) {
            this.position.y = height + this.radius;
        }
    }

    // Boolean hits(ArrayList<Sprite> sprites) {
    //     for (int i = sprites.size() - 1; i >= 0; i--) {
    //         Sprite sprite = sprites.get(i);
    //         float distance = dist(this.position.x, this.position.y, sprite.position.x, sprite.position.y);

    //         if (distance < this.radius + sprite.radius) {
    //             return true;
    //         }
    //     }

    //     return false;
    // }
}
public class Star {
    float x;
    float y;
    float z;
    float pz;

    Star() {
        this.x = random(-width, width);
        this.y = random(-height, height);
        this.z = random(width);
        this.pz = z;
    }

    public void update() {
        this.z = this.z - 4;

        if (this.z < 1) {
            this.x = random(-width, width);
            this.y = random(-height, height);
            this.z = width; 
            this.pz = z;
        }
    }

    public void draw() {
        float sx = map(x / z, 0, 1, 0, width);
        float sy = map(y / z, 0, 1, 0, height);
        float radius = map(z, 0, width, 8, 0);
        float px = map(x / pz, 0, 1, 0, width);
        float py = map(y / pz, 0, 1, 0, height);

        this.pz = z;

        fill(255);
        noStroke();
        stroke(255);
        line(px, py, sx, sy);
    }
}
public class Starfield {
    Star[] stars = new Star[400];

    Starfield() {
        for (int i = 0; i < stars.length; i++) {
            stars[i] = new Star();
        }
    }

    public void draw() {
        for (int i = 0; i < stars.length; i++) {
            stars[i].update();

            pushMatrix();
                translate(width / 2, height / 2);
                stars[i].draw();
            popMatrix();
        }
    }
}
  public void settings() {  size(1000, 800, P2D); }
  static public void main(String[] passedArgs) {
    String[] appletArgs = new String[] { "Asteroids" };
    if (passedArgs != null) {
      PApplet.main(concat(appletArgs, passedArgs));
    } else {
      PApplet.main(appletArgs);
    }
  }
}
