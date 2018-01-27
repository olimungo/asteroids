
public static enum State {
    HOMESCREEN, PLAYING, GAME_OVER 
}

public class GameManager {
    int asteroidsHit = 0;
    State state = State.HOMESCREEN;

    private Helpers helpers;

    GameManager() {
        this.helpers = new Helpers();
    }

    void draw() {
        pushStyle();
            fill(219, 233, 255, random(200, 255));

            if (this.state == State.HOMESCREEN) {
                this.helpers.showHomescreen();
            } else if (this.state == State.PLAYING) {
                this.helpers.showScore(this.getScore());
            } else if (this.state == State.GAME_OVER) {
                this.helpers.showScore(this.getScore());
                this.helpers.showGameOver();
            } 
        popStyle();
    }

    void startGame() {
        this.asteroidsHit = 0;
        this.state = State.PLAYING;
    }

    void endGame() {
        this.state = State.GAME_OVER;
    }

    void asteroidHit() {
        this.asteroidsHit++;
    }

    private int getScore() {
        return asteroidsHit * 10;
    }
}