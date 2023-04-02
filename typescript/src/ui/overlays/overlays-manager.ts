import P5 from 'p5';
import { GameState } from '../../game-states';
import {
    Hub,
    GameOver,
    Help,
    Homescreen,
    Lifes,
    NextLevel,
    NextLife,
    Score,
    TopScore,
    Level,
    NewLife,
    Starfield,
    GamePaused,
} from '.';
import Interval from '../../interval';

const DISPLAY_NEW_LIFE_TIMEOUT = 5000;

export default class OverlaysManager {
    private p5: P5;

    private starfield: Starfield;

    private overlayHomeScreen: Homescreen;
    private overlayHub: Hub;
    private overlayNextLevel: NextLevel;
    private overlayNextLife: NextLife;
    private overlayGameOver: GameOver;
    private overlayScore: Score;
    private overlayHelp: Help;
    private overlayLifes: Lifes;
    private overlayTopScore: TopScore;
    private overlayLevel: Level;
    private overlayNewLife: NewLife;
    private overlayGamePaused: GamePaused;

    private showStarfield = false;
    private showHub = false;
    private showHelp = false;
    private scaleStage = false;
    private showNewLife = false;
    private newLifeInterval: Interval | null;

    constructor(p5: P5) {
        this.p5 = p5;

        this.starfield = new Starfield(p5);

        this.overlayHomeScreen = new Homescreen(p5);
        this.overlayHub = new Hub(p5);
        this.overlayNextLevel = new NextLevel(p5);
        this.overlayNextLife = new NextLife(p5);
        this.overlayGameOver = new GameOver(p5);
        this.overlayScore = new Score(p5);
        this.overlayHelp = new Help(p5);
        this.overlayLifes = new Lifes(p5);
        this.overlayTopScore = new TopScore(p5);
        this.overlayLevel = new Level(p5);
        this.overlayNewLife = new NewLife(p5);
        this.overlayGamePaused = new GamePaused(p5);
    }

    update() {
        if (this.newLifeInterval && this.newLifeInterval.isElapsed()) {
            this.showNewLife = false;
            this.newLifeInterval = null;
        }

        if (this.showStarfield) {
            this.starfield.update();
        }
    }

    drawForeground(
        gameState: GameState,
        gamePaused: boolean,
        topScore: number,
        score: number,
        level: number,
        lifes: number
    ) {
        this.overlayTopScore.draw(topScore);

        switch (gameState) {
            case GameState.HOMESCREEN:
                this.overlayHomeScreen.draw();
                break;
            case GameState.NEXT_LEVEL:
                this.overlayNextLevel.draw(level);
                break;
            case GameState.NEXT_LIFE:
                this.overlayNextLife.draw(lifes);
                break;
            case GameState.GAME_OVER:
                this.overlayGameOver.draw();
                break;
        }

        if (gameState == GameState.PLAYING && gamePaused) {
            this.overlayGamePaused.draw();
        }

        if (
            gameState == GameState.PLAYING ||
            gameState == GameState.NEXT_LEVEL
        ) {
            this.overlayLifes.draw();
        }

        if (
            gameState == GameState.PLAYING ||
            gameState == GameState.GAME_OVER ||
            gameState == GameState.NEXT_LEVEL ||
            gameState == GameState.NEXT_LIFE
        ) {
            this.overlayScore.draw(score);
            this.overlayLevel.draw(level);
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

    drawBackground() {
        if (this.scaleStage) {
            this.setScale(1.3);
        } else {
            this.setScale(1);
        }

        if (this.showStarfield) {
            this.starfield.draw();
        }
    }

    keyPressed(keyCode: number) {
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

    setLifeCount(count: number) {
        this.overlayLifes.setLifeCount(count);
    }

    displayNewLife() {
        this.showNewLife = true;
        this.newLifeInterval = new Interval(DISPLAY_NEW_LIFE_TIMEOUT);
    }

    pause() {
        if (this.newLifeInterval != null) {
            this.newLifeInterval.pause();
        }
    }
    unpause() {
        if (this.newLifeInterval != null) {
            this.newLifeInterval.unpause();
        }
    }

    private setScale(ratio: number) {
        this.p5.scale(1 / ratio);

        this.p5.translate(
            (this.p5.width * ratio - this.p5.width) / 2,
            (this.p5.height * ratio - this.p5.height) / 2
        );
    }
}
