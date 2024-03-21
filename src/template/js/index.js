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
    const data = await fetch("https://qrsvg-gen.my.id/api/svg", {
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
    if (payload === "") {
      div.innerHTML = "Data Kosong!";
      div.style.color = "red";
    } else {
      div.innerHTML = data.svg;
    }
  };
})();
