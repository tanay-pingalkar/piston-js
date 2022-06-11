function Config(){
    return {
        height:200,
        width:400
    };
};

let x = 0.0;
let y = 0.0;
let speed = 1;
let direction = "None";

function Update(input) {
    if (input) {
        if (input.button.Keyboard == "Right") {
            if(input.state == "Press") {
                direction = "Right"
            }
            else {
                direction = "None"
            }
        }

        if (input.button.Keyboard == "Left") {
            if(input.state == "Press") {
                direction = "Left"
            }
            else {
                direction = "None"
            }
        }
        if (input.button.Keyboard == "Up") {
            if(input.state == "Press") {
                direction = "Up"
            }
            else {
                direction = "None"
            }
        }
        if (input.button.Keyboard == "Down") {
            if(input.state == "Press") {
                direction = "Down"
            }
            else {
                direction = "None"
            }
        }
    };
    
    if(direction === "Right" && x<350) {
        x = x+speed;
    }

    if(direction === "Left" && x >0 ) {
        x = x-speed;
    }

    if(direction === "Up" && y >0) {
        y = y-speed;
    }

    if(direction === "Down"&& y<150) {
        y = y+speed;
    }

    
};

function Draw() {
    Clear([1.0,1.0,1.0,1.0]);
    Rect([1.0, 0.0, 0.0, 1.0], [x, y, 50.0, 50.0]);
};
