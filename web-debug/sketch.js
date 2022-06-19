import init from '../dankgine/pkg/dankgine.js'
import * as Dankgine from '../dankgine/pkg/dankgine.js'

init().then(() => {
    new p5(function (p5) {

        let state
        let freeze = false

        p5.setup = function () {
            p5.createCanvas(600, 600)
        }

        p5.draw = function () {
            p5.background(0)
            if (!freeze) {
                state = JSON.parse(Dankgine.update())
            }
            for (const body of state.bodies) {
                p5.circle(body.current_position.x, body.current_position.y, body.radius * 2)
            }

            if (p5.frameCount % 25 === 0 && !freeze) {
                Dankgine.add_body(250, 50, Math.random() * (10 - 5) + 5)
                Dankgine.add_body(350, 50, Math.random() * (10 - 5) + 5)
            }
            p5.push()
            p5.noFill()
            p5.stroke(255)
            p5.circle(300, 300, 600)
            p5.pop()
        }

        p5.mousePressed = function () {
            Dankgine.add_body(p5.mouseX, p5.mouseY, 10)
        }

        p5.keyPressed = function () {
            if (p5.keyCode === 13) {
                freeze = !freeze
            }
        }
    })
})