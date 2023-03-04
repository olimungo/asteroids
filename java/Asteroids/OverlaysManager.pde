public class OverlaysManager {
    private final static int DISPLAY_NEW_LIFE_TIMEOUT = 5000;

    private Starfield starfield;

    private Homescreen overlayHomeScreen;
    private Hub overlayHub;
    private NextLevel overlayNextLevel;
    private NextLife overlayNextLife;
    private GameOver overlayGameOver;
    private Score overlayScore;
    private Help overlayHelp;
    private Lifes overlayLifes;
    private Level overlayLevel;
    private GamePaused overlayGamePaused;
    private NewLife overlayNewLife;
    private TopScore overlayTopScore;

    private Boolean showStarfield = false;
    private Boolean showHub = false;
    private Boolean showHelp = false;
    private Boolean scaleStage = false;
    private Boolean showNewLife = false;
    private Interval newLifeInterval;

    OverlaysManager() {
        this.starfield = new Starfield();

        this.overlayHomeScreen = new Homescreen();
        this.overlayHub = new Hub();
        this.overlayNextLevel = new NextLevel();
        this.overlayNextLife = new NextLife();
        this.overlayGameOver = new GameOver();
        this.overlayScore = new Score();
        this.overlayHelp = new Help();
        this.overlayLifes = new Lifes();
        this.overlayLevel = new Level();
        this.overlayGamePaused = new GamePaused();
        this.overlayNewLife = new NewLife();
        this.overlayTopScore = new TopScore();
    }

    void update() {
        if (this.newLifeInterval != null && this.newLifeInterval.isElapsed()) {
            this.showNewLife = false;
            this.newLifeInterval = null;
        }

        if (this.showStarfield) {
            this.starfield.update();
        }
    }

    void drawForeground(
        GameState gameState,
        Boolean gamePaused,
        int topScore,
        int score,
        int level,
        int lifes) {
        this.overlayTopScore.draw(topScore);

        switch (gameState) {
            case HOMESCREEN:
                this.overlayHomeScreen.draw();
                break;
            case PLAYING:
                this.overlayScore.draw(score);
                this.overlayLifes.draw();
                this.overlayLevel.draw(level);

                if (gamePaused) {
                    this.overlayGamePaused.draw();
                }

                break;
            case NEXT_LEVEL:
                this.overlayNextLevel.draw(level);
                break;
            case NEXT_LIFE:
                this.overlayNextLife.draw(lifes);
                break;
            case GAME_OVER:
                this.overlayGameOver.draw();
                break;
        }

        if (this.showNewLife) {
            this.overlayNewLife.draw();
        }

        if (this.showHelp) {
            this.overlayHelp.draw();
        }

        if (this.showHub) {
            this.overlayHub.draw();
        }
    }

    void drawBackground(){
        if (this.scaleStage) {
            this.setScale(1.3);
        } else {
            this.setScale(1);
        }

        if (this.showStarfield) {
            this.starfield.draw();
        }
    }

    void keyPressed(int keyCode) {
        switch (keyCode) {
            case 69: // e
                this.scaleStage = !this.scaleStage;
                break;
            case 72: // h
                this.showHelp = !this.showHelp;
                break;
            case 85: // u
                this.showHub = !this.showHub;
                break;
            case 88: // x
                this.showStarfield = !this.showStarfield;
                break;
        }
    }

    void setLifeCount(int count) {
        this.overlayLifes.setLifeCount(count);
    }

    void displayNewLife() {
        this.showNewLife = true;
        this.newLifeInterval = new Interval(DISPLAY_NEW_LIFE_TIMEOUT);
    }

    void pause() {
        if (this.newLifeInterval != null) {
            this.newLifeInterval.pause();
        }
    }
    void unpause() {
        if (this.newLifeInterval != null) {
            this.newLifeInterval.unpause();
        }
    }

    private void setScale(float ratio) {
        scale(1 / ratio);

        translate(
            (width * ratio - width) / 2,
            (height * ratio - height) / 2
        );
    }
}