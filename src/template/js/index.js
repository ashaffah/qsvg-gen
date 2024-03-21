"use strict";

(async () => {
  let inputEl = document.getElementById("text");
  let div = document.getElementById("qr");
  let buttonGen = document.getElementById("generate");
  let payload = "";
  inputEl.onchange = async (e) => {
    payload = e.target.value;
  };
  buttonGen.onclick = async () => {
    const data = await fetch("http://127.0.0.1:8000/api/svg", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
      },
      body: JSON.stringify({
        content: payload,
      }),
    })
      .then((response) => {
        return response.json();
      })
      .catch((error) => {
        console.log(error);
      });
    div.innerHTML = data.svg;
  };
})();
