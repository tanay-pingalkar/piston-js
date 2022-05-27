
Clear([1, 1, 1, 1])
Rect([1, 0, 0, 1], [D.player_x, D.player_y, 50, 50])


// if (Input.state != "None") Log([Input])


if (D.player_direction_y == "Up") {

    Set({ player_y: D.player_y - 1 })
}
else if (D.player_direction_y == "Down") {
    Set({ player_y: D.player_y + 1 })
}

if (D.player_direction_x == "Left") {

    Set({ player_x: D.player_x - 1 })
}
else if (D.player_direction_x == "Right") {
    Set({ player_x: D.player_x + 1 })
}

if (Input.state == "Press" && Input.key == "Keyboard(Up)") {

    Set({ player_direction_y: "Up" })
}
if (Input.state == "Release" && Input.key == "Keyboard(Up)") {
    Set({ player_direction_y: "None" })
}
if (Input.state == "Press" && Input.key == "Keyboard(Down)") {
    Set({ player_direction_y: "Down" })
}
if (Input.state == "Release" && Input.key == "Keyboard(Down)") {
    Set({ player_direction_y: "None" })
}
if (Input.state == "Press" && Input.key == "Keyboard(Left)") {

    Set({ player_direction_x: "Left" })
}
if (Input.state == "Release" && Input.key == "Keyboard(Left)") {
    Set({ player_direction_x: "None" })
}
if (Input.state == "Press" && Input.key == "Keyboard(Right)") {
    Set({ player_direction_x: "Right" })
}
if (Input.state == "Release" && Input.key == "Keyboard(Right)") {
    Set({ player_direction_x: "None" })
}




