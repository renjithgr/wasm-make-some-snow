import("../crate/pkg").then(module => {

  const canvas = document.createElement('canvas');
  const context = canvas.getContext('2d');
  document.body.appendChild(canvas);

  const flakes = 500;
  const snowflakes = [];

  for(let s = 0; s < flakes; s++) {
    snowflakes.push(module.Snowflake.new(window.innerWidth, window.innerHeight));
  }

  function update() {
    
    context.clearRect(0, 0, window.innerWidth, window.innerHeight);

    for(const flake of snowflakes) {
      flake.tick()
      context.save();
      context.fillStyle = '#FFF';
      context.beginPath();
      context.arc(flake.x(), flake.y(), flake.radius(), 0, Math.PI * 2);
      context.closePath();
      context.globalAlpha = flake.alpha();
      context.fill();
      context.restore();
    }

    requestAnimationFrame(update);
  }

  requestAnimationFrame(update);

});

