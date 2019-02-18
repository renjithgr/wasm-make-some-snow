// import("../crate/pkg").then(module => {
//   module.run();
// });

class Snowflake {
  constructor(opts) {
    this.x = 0;
    this.y = 0;
    this.velocityX = 0;
    this.velocityY = 0;
    this.radius = 0;
    this.alpha = 0;
    this.reset();
  }

  reset() {
    this.x = this.randBetween(0, window.innerWidth);
    this.y = this.randBetween(0, -window.innerHeight);
    this.velocityX = this.randBetween(-3, 3);
    this.velocityY = this.randBetween(2, 5);
    this.radius = this.randBetween(1, 4);
    this.alpha = this.randBetween(0.1, 0.9);
  }

  randBetween(min, max) {
    return min + Math.random() * (max - min);
  }

  update() {
    this.x += this.velocityX;
    this.y += this.velocityY;
    
    if(this.y + this.radius > window.innerHeight) {
      this.reset();
    }
  }
}

class Snow {
  constructor() {
    this.canvas = document.createElement('canvas');
    this.context = this.canvas.getContext('2d');
    document.body.appendChild(this.canvas);

    this.createSnowflakes();

    window.addEventListener('resize', () => this.onResize());
    this.onResize();

    this.update = this.update.bind(this);
    requestAnimationFrame(this.update);
  }

  onResize() {
    this.width = window.innerWidth;
    this.height = window.innerHeight;
    this.canvas.width = window.innerWidth;
    this.canvas.height = window.innerHeight;
  }

  createSnowflakes() {
    const flakes = window.innerWidth / 4;
    this.snowflakes = [];

    for(let s = 0; s < flakes; s++) {
      this.snowflakes.push(new Snowflake());
    }
  }

  update() {
    this.context.clearRect(0, 0, this.width, this.height);

    for(const flake of this.snowflakes) {
      flake.update();

      this.context.save();
      this.context.fillStyle = '#FFF';
      this.context.beginPath();
      this.context.arc(flake.x, flake.y, flake.radius, 0, Math.PI * 2);
      this.context.closePath();
      this.context.globalAlpha = flake.alpha;
      this.context.fill();
      this.context.restore();
    }

    requestAnimationFrame(this.update);
  }
}

new Snow();