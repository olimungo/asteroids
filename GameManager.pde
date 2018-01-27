
public static enum State {
    HOMESCREEN, PLAYING, NEXT_LIFE, GAME_OVER 
}

public class GameManager {
    int asteroidsHit;
    int lifes;
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
                this.helpers.showRemainingLifes(this.lifes - 1);
            } else if (this.state == State.NEXT_LIFE) {
                this.helpers.showScore(this.getScore());
                this.helpers.showRemainingLifes(this.lifes);
                this.helpers.showNextLife(this.lifes);
            } else if (this.state == State.GAME_OVER) {
                this.helpers.showScore(this.getScore());
                this.helpers.showGameOver();
            }
        popStyle();
    }

    void startGame() {
        this.asteroidsHit = 0;
        this.lifes = 3;
        this.state = State.PLAYING;
    }

    void lifeLost() {
        this.lifes--;

        if (this.lifes == 0) {
            this.endGame();
        } else {
            this.state = State.NEXT_LIFE;
        }
    }

    void startNewLife() {
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