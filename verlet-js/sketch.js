let state
let n

function setup() {
    createCanvas(600, 600)
    state = new State()
    n = 0
}

function draw() {
    background(0)

    let state = update_engine()

    for (const body of state.bodies) {
        circle(body.current_position.x, body.current_position.y, body.radius * 2)
    }

    if (frameCount % 16 === 0) {
        add_body(200, 50, 5)
        add_body(300, 50, 5)
        add_body(400, 50, 5)
        n = n + 3
    }
    push()
    noFill()
    stroke(255)
    circle(300, 300, 600)
    pop()

    push()
    fill(255)
    textSize(16)
    text(n, 50, 40)
    pop()
}

function update_engine() {
    let solver = new Solver()
    solver.update(0.016, state.bodies)
    return state
}

function add_body(x, y, radius) {
    state.bodies.push(new VerletObject(new Vec2(x, y), radius))
}

class State {
    constructor() {
        this.bodies = []
    }
}

class Solver {
    constructor() {
        this.gravity = new Vec2(0, 1000)
    }

    update(dt, bodies) {
        const sub_steps = 8
        const sub_dt = dt / sub_steps
        for (let i = 0; i < sub_steps; i++) {
            this.apply_gravity(bodies)
            this.apply_constraint(bodies)
            this.solve_collisions(bodies)
            this.update_position(sub_dt, bodies)
        }
    }

    update_position(dt, bodies) {
        for (const body of bodies) {
            body.update_position(dt)
        }
    }

    apply_gravity(bodies) {
        for (const body of bodies) {
            body.accelerate(this.gravity)
        }
    }

    apply_constraint(bodies) {
        let constraint_position = new Vec2(300, 300)
        let radius = 300
        for (const body of bodies) {
            let diff = body.current_position.sub(constraint_position)
            let dist = diff.length()
            if (dist > radius - body.radius) {
                let n = diff.div(dist)
                body.current_position = n.mul(radius - body.radius).add(constraint_position)
            }
        }
    }

    solve_collisions(bodies) {
        for (const b1 of bodies) {
            for (const b2 of bodies) {
                if (b1 === b2) continue
                let collision_axis = b1.current_position.sub(b2.current_position)
                let dist = collision_axis.length()
                let min_dist = b1.radius + b2.radius

                if (dist < min_dist) {
                    let n = collision_axis.div(dist)
                    let delta = min_dist - dist

                    b1.current_position = b1.current_position.add(n.mul(0.5 * delta))
                    b2.current_position = b2.current_position.sub(n.mul(0.5 * delta))
                }
            }
        }
    }
}

class VerletObject {
    constructor(pos, radius) {
        this.current_position = pos
        this.old_position = pos
        this.acceleration = new Vec2(0, 0)
        this.radius = radius
    }

    update_position(dt) {
        let velocity = this.current_position.sub(this.old_position)
        this.old_position = new Vec2(this.current_position.x, this.current_position.y)
        this.current_position = (this.acceleration.mul(Math.pow(dt, 2)).add(velocity).add(this.current_position))
        this.acceleration = new Vec2(0, 0)
    }

    accelerate(acc) {
        this.acceleration = this.acceleration.add(acc)
    }
}

class Vec2 {

    constructor(x, y) {
        this.x = x;
        this.y = y;
    }

    length() {
        return Math.sqrt(Math.pow(this.x, 2) + Math.pow(this.y, 2))
    }

    add(v) {
        return new Vec2(this.x + v.x, this.y + v.y)
    }

    sub(v) {
        return new Vec2(this.x - v.x, this.y - v.y)
    }

    mul(scalar) {
        return new Vec2(this.x * scalar, this.y * scalar)
    }

    div(demoniator) {
        return new Vec2(this.x / demoniator, this.y / demoniator)
    }
}