import("../wasm/pkg")
  .then((wasm) => {
    console.log(wasm.greeting());
    const container = document.getElementById("container");
    console.log(wasm.render(container));
  })
  .catch(console.error);
