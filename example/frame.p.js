
Draw2d()
Clear([1, 1, 1, 1])
Rect([1, 0, 0, 1], [D.playerx, 10, 100, 100])


Set({ playerx: D.playerx + D.direction })

if (D.playerx > 400) {
    Set({ direction: -1 })
}

if (D.playerx < 0) {
    Set({ direction: 1 })
}
