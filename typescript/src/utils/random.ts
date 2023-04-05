import { Vector } from 'p5';

export function random_limit(limit: number, min_value: number): Vector {
    const constrain_limit = limit - min_value;
    let x = Math.random() * constrain_limit * 2 - constrain_limit;
    let y = Math.random() * constrain_limit * 2 - constrain_limit;

    if (x > 0.0) {
        x += min_value;
    } else {
        x -= min_value;
    }

    if (y > 0.0) {
        y += min_value;
    } else {
        y -= min_value;
    }

    return new Vector(x, y);
}
