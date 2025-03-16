let has_account = false;
div_login = document.querySelector('div.login');
document.addEventListener("interactive", function(evt) {
    if (has_account) {
        div_login.style.display = "block";
    } else {
        div_login.style.display = "hidden";
    }
});