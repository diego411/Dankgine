import init from '../pkg/dankgine.js'
import * as Dankgine from '../pkg/dankgine.js'

init().then(() => {
    new p5(function (p5) {

        let state
        let n = 0

        p5.setup = function () {
            p5.createCanvas(600, 600)
        }

        p5.draw = function () {
            p5.background(0)

            state = JSON.parse(Dankgine.update())
            for (const body of state.bodies) {
                p5.circle(body.current_position.x, body.current_position.y, body.radius * 2)//body.radius * 2)
            }

            if (p5.frameCount % 16 === 0) {
                Dankgine.add_body(200, 50, 5)
                Dankgine.add_body(300, 50, 5)
                Dankgine.add_body(400, 50, 5)
                n = n + 3
            }
            p5.push()
            p5.noFill()
            p5.stroke(255)
            p5.circle(300, 300, 600)
            p5.pop()

            p5.push()
            p5.fill(255)
            p5.textSize(16)
            p5.text(n, 50, 40)
            p5.pop()
        }

        p5.mousePressed = function () {
            Dankgine.add_body(p5.mouseX, p5.mouseY, 10)
        }
    })
})