import P5 from 'p5';
import { GameState } from './game-states';
import SpritesManager from './sprites/sprites-manager';
import OverlaysManager from './ui/overlays/overlays-manager';
import Interval from './utils/interval';

const LIFES_WHEN_STARTING = 3;
const ASTEROIDS_START_COUNT = 12;
const ASTEROIDS_LEVEL_INCREMENT = 3;
const GAME_OVER_STATE_TIMEOUT = 8000; // ms
const ADD_LIFE_WHEN_SCORED = 3000;
const ASTEROID_HIT_SCORE = 10;
const UFO_HIT_SCORE = 50;
const UFO_CREATE_INIT_FREQUENCY = 15000; // ms
const UFO_CREATE_DECREMENT_FREQUENCY = 1000; // ms
const UFO_CREATE_MINIMAL_FREQUENCY = 10000; // ms
const UFO_SHOOT_INIT_FREQUENCY = 15000; // ms
const UFO_SHOOT_DECREMENT_FREQUENCY = 500; // ms
const UFO_SHOOT_MINIMAL_FREQUENCY = 5000; // ms

export default class GameManager {
    private p5: P5;

    private spritesManager: SpritesManager;
    private overlaysManager: OverlaysManager;

    private gameState = GameState.HOMESCREEN;

    private level: number;
    private lifes: number;
    private score: number;
    private topScore: number = 0;
    private maxAsteroids: number;
    private gameOverInterval = new Interval();
    private lifeAddedSoFar: number;
    private ufoCreateFrequency: number;
    private ufoShootFrequency: number;
    private gamePaused = false;

    constructor(p5: P5) {
        this.p5 = p5;

        this.spritesManager = new SpritesManager(p5);
        this.overlaysManager = new OverlaysManager(p5);

        this.spritesManager.createAsteroids(ASTEROIDS_START_COUNT);
        this.spritesManager.createUfo(0);

        // Get the cookie top score
        for (const cookie of document.cookie.replace(/ /g, '').split(';')) {
            const cookieSplit = cookie.split('=');

            if (cookieSplit[0] === 'top-score') {
                this.topScore = parseInt(cookieSplit[1]);
            }
        }
    }

    update() {
        this.overlaysManager.update();

        if (!this.gamePaused) {
            this.spritesManager.update();

            if (this.gameState === GameState.PLAYING) {
                this.checkLevel();
                this.checkNewLife();
            }
        }

        if (this.gameOverInterval.isElapsed()) {
            this.gameOverInterval.cancel();
            this.gameState = GameState.HOMESCREEN;

            this.spritesManager.reset();
            this.spritesManager.createUfo(0);
            this.spritesManager.createAsteroids(ASTEROIDS_START_COUNT);
        }
    }

    draw() {
        this.overlaysManager.drawBackground();

        this.spritesManager.draw();

        this.overlaysManager.drawForeground(
            this.gameState,
            this.gamePaused,
            this.topScore,
            this.getScore(),
            this.level,
            this.lifes
        );
    }

    keyPressed(keyCode) {
        this.overlaysManager.keyPressed(keyCode);

        switch (this.gameState) {
            case GameState.HOMESCREEN:
            case GameState.GAME_OVER:
                if (keyCode === 83) {
                    // s
                    this.startGame();
                }

                break;
            case GameState.PLAYING:
                this.spritesManager.keyPressed(keyCode);

                if (keyCode === 80) {
                    // p
                    this.gamePaused = !this.gamePaused;

                    if (this.gamePaused) {
                        this.spritesManager.pause();
                    } else {
                        this.spritesManager.unpause();
                    }
                }

                break;
            case GameState.NEXT_LEVEL:
                if (keyCode === 83) {
                    // s
                    this.nextLevel();
                }

                break;
            case GameState.NEXT_LIFE:
                if (keyCode === 83) {
                    // s
                    this.startLevel();
                }

                break;
        }
    }

    keyReleased(keyCode) {
        this.spritesManager.keyReleased(keyCode);
    }

    private getScore() {
        return (
            this.score +
            this.spritesManager.countAsteroidsHit * ASTEROID_HIT_SCORE +
            this.spritesManager.countUfosHit * UFO_HIT_SCORE
        );
    }

    private startGame() {
        this.gameOverInterval.cancel();

        this.score = 0;
        this.level = 1;
        this.maxAsteroids = ASTEROIDS_START_COUNT;
        this.lifes = LIFES_WHEN_STARTING;
        this.overlaysManager.setLifeCount(this.lifes - 1);
        this.lifeAddedSoFar = 0;
        this.ufoCreateFrequency = UFO_CREATE_INIT_FREQUENCY;
        this.ufoShootFrequency = UFO_SHOOT_INIT_FREQUENCY;

        this.spritesManager.reset();

        this.startLevel();
    }

    private startLevel() {
        this.gameState = GameState.PLAYING;

        this.spritesManager.startLevel(
            this.maxAsteroids,
            this.ufoCreateFrequency,
            this.ufoShootFrequency
        );
    }

    private nextLevel() {
        this.level++;
        this.maxAsteroids += ASTEROIDS_LEVEL_INCREMENT;
        this.ufoCreateFrequency -= UFO_CREATE_DECREMENT_FREQUENCY;
        this.ufoShootFrequency -= UFO_SHOOT_DECREMENT_FREQUENCY;

        if (this.ufoCreateFrequency < UFO_CREATE_MINIMAL_FREQUENCY) {
            this.ufoCreateFrequency = UFO_CREATE_MINIMAL_FREQUENCY;
        }

        if (this.ufoShootFrequency < UFO_SHOOT_MINIMAL_FREQUENCY) {
            this.ufoShootFrequency = UFO_SHOOT_MINIMAL_FREQUENCY;
        }

        this.startLevel();
    }

    private checkLevel() {
        if (!this.spritesManager.shipHit()) {
            if (this.spritesManager.getAsteroidsCount() === 0) {
                this.gameState = GameState.NEXT_LEVEL;
                this.score = this.getScore();
                this.spritesManager.stopLevel();
            }
        } else {
            this.lifes--;
            this.overlaysManager.setLifeCount(this.lifes - 1);
            this.score = this.getScore();
            this.spritesManager.stopLevel();

            if (this.lifes === 0) {
                this.gameState = GameState.GAME_OVER;

                this.topScore =
                    this.score > this.topScore ? this.score : this.topScore;

                // Save the top score in a cookie
                document.cookie = `top-score=${this.topScore}`;

                // Return to homescreen after some time...
                this.gameOverInterval.set(GAME_OVER_STATE_TIMEOUT);
            } else {
                this.gameState = GameState.NEXT_LIFE;
            }
        }
    }

    private checkNewLife() {
        if (
            this.getScore() - this.lifeAddedSoFar * ADD_LIFE_WHEN_SCORED >=
            ADD_LIFE_WHEN_SCORED
        ) {
            this.lifeAddedSoFar++;
            this.lifes++;
            this.overlaysManager.setLifeCount(this.lifes - 1);
            this.overlaysManager.displayNewLife();
        }
    }
}
