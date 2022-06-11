function Config(){
    return {
        height:200,
        width:400
    };
};

let x = 0.0;
let y = 0.0;

function Update() {
    x =  x+ 0.15;
    y = x;
};

function Draw() {
    Clear([1.0,1.0,1.0,1.0]);
    Rect([1.0, 0.0, 0.0, 1.0], [x, y, x, x]);
};
