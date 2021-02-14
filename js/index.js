// Setup global variables
var canvas;
var ctx;

import("../pkg").then(module => {
  var startTime;
  var endTime;

  document.getElementById('img_uploader').addEventListener('change', readURL, true);
  document.getElementById('run_button').addEventListener('click', function(){processImage()}, false);

  function readURL() {
    let file = document.getElementById("img_uploader").files[0];

    let reader = new FileReader();
    reader.onloadend = function () {
      newimg.src = reader.result;  // Set the global image to the path of the file on the client's PC.
    }
    if (file) {
      reader.readAsDataURL(file);
    } else {
      // Error message TODO
      console.log("Could not read file. :(")
    }
  }

  // Setup images
  const newimg = new Image();
  newimg.style.display = "none";
  newimg.onload = () => {
    setUpCanvas();
  }

  function processImage() {
    startTime = performance.now();
    ctx.drawImage(newimg, 0, 0);
    let filter_name = event.target.id;

    console.time("wasm_time");

    // Convert the ImageData to a PhotonImage (so that it can communicate with the core Rust library)
    let rust_image = module.to_black_and_white_from_js(canvas, ctx);

    endTime = performance.now();
    updateBenchmarks();
    console.timeEnd("wasm_time");
  }

  function setUpCanvas() {
    canvas = document.getElementById("canvas");
    canvas.width = newimg.width;
    canvas.height = newimg.height;

    ctx = canvas.getContext("2d");
    ctx.drawImage(newimg, 0, 0);
  }

  function updateBenchmarks() {
    console.log("update benchmarks");
    let time_taken = endTime - startTime;
    let time_elem = document.getElementById("time");
    time_elem.innerHTML = `Time: ${time_taken}ms`;
  }
});
