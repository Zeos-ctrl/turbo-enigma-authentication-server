document.querySelector("#login").addEventListener("click",() => {
    document.querySelector(".popupregister").classList.remove("active");
    document.querySelector(".popup").classList.add("active");
});
document.querySelector("#register").addEventListener("click", () => {
    httpGetAsync("http://127.0.0.1:8000/gen_captcha", function(result){
        console.log(result);
    });
    document.querySelector(".popup").classList.remove("active");
    document.querySelector(".popupregister").classList.add("active");
});
document.querySelector(".popup .close-btn").addEventListener("click", () => {
    document.querySelector(".popup").classList.remove("active");
});
document.querySelector(".popupregister .close-btn").addEventListener("click", () => {
    document.querySelector(".popupregister").classList.remove("active");
});

function httpGetAsync(theUrl, callback){
    var xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = function() { 
        if (xmlHttp.readyState == 4 && xmlHttp.status == 200)
            callback(xmlHttp.responseText);
    }
    xmlHttp.open("GET", theUrl, true); // true for asynchronous 
    xmlHttp.send(null);
}

function updateDiv()
{ 
    $( "#here" ).load(window.location.href + " #here" );
}
