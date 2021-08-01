function set(key, value) {
    localStorage.setItem(key, value);
}

function get(key) {
    return localStorage.getItem(key);
}

function canvasState(state) {
    canvas = document.getElementsByTagName("canvas")[0];

    if (canvas != null) {
        document.getElementsByTagName("canvas")[0].style.visibility = state;
    }
}