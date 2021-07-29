function set(key, value) {
    localStorage.setItem(key, value);
}

function get(key) {
    return localStorage.getItem(key);
}

// function clearLocalStorage() {
//     document.defaultView.onunload(function () {
//         localStorage.clear();
//     }
// }