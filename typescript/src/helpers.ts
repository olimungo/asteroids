import P5 from 'p5';
import { GameState } from './game-states';

export default class Helpers {
    p5: P5;
    centerX: number;
    centerY: number;

    // String frameRateMessage = "";

    // private ShipShell[] shipShells;

    constructor(p5: P5) {
        this.p5 = p5;
        this.centerX = p5.width / 2;
        this.centerY = p5.height / 2;

        // shipShells = new ShipShell[10];
        // int x = 20;

        // for (int i = 0; i < shipShells.length; i++) {
        //     x += 50;
        //     ShipShell shipShell = new ShipShell(x, 60);
        //     shipShell.heading = -PI / 2;
        //     this.shipShells[i] = shipShell;
        // }
    }

    // void showFrameRate() {
    //     if (frameCount % 5 == 0 || frameCount < 5) {
    //         this.frameRateMessage = String.format("%2.0f / %d / %d / %d + %d", frameRate, frameCount, millis(), mouseX, mouseY);
    //     }

    //     pushStyle();
    //         fill(255, 255, 255, 150);
    //         textSize(30);
    //         textAlign(LEFT);
    //         text(this.frameRateMessage, 30, height - 40);
    //     popStyle();
    // }

    showTitles(
        gameState: GameState,
        // lifes: number,
        level: number
        // score: number,
        // topScore: number,
        // isPaused: boolean
    ) {
        switch (gameState) {
            case GameState.HOMESCREEN:
                break;
            case GameState.PLAYING:
                break;
            case GameState.NEXT_LEVEL:
                break;
            case GameState.NEXT_LIFE:
                break;
            case GameState.GAME_OVER:
                break;
        }

        if (gameState == GameState.HOMESCREEN) {
            // this.showTopScore(topScore);
        }
        // else if (gameState == GameState.PLAYING) {
        //     this.showScore(score);
        //     this.showRemainingLifes(lifes - 1);
        //     this.showLevel(level);

        //     if (isPaused) {
        //         this.showPause();
        //     }
        // } else if (gameState == GameState.NEXT_LEVEL) {
        //     this.showScore(score);
        //     this.showRemainingLifes(lifes - 1);
        //     this.showNextLevel(level);
        //     this.showTopScore(topScore);
        // } else if (gameState == GameState.NEXT_LIFE) {
        //     this.showScore(score);
        //     this.showRemainingLifes(lifes);
        //     this.showLevel(level);
        //     this.showNextLife(lifes);
        // } else if (gameState == GameState.GAME_OVER) {
        //     this.showScore(score);
        //     this.showGameOver();
        //     this.showTopScore(topScore);
        // }
    }

    // void showPause() {
    //     pushStyle();
    //         textAlign(CENTER);
    //         textSize(80);
    //         text("PAUSE", width / 2, height / 2);
    //     popStyle();
    // }
}
