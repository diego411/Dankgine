import init from '../dankgine/pkg/dankgine.js'
import * as Dankgine from '../dankgine/pkg/dankgine.js'

init().then(() => {
    new p5(function (p5) {
        let i = 0

        p5.setup = function () {
            p5.createCanvas(600, 600)
            console.log(Dankgine.add(3, 6))
        }

        p5.draw = function () {
            p5.background(0)
            p5.circle(50, 50 + i, 20)
            i++
        }
    })
})