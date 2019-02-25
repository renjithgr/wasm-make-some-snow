#[macro_use]
extern crate cfg_if;
extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn rand_between(min: f64, max: f64) -> f64 {
    (js_sys::Math::random() * (max - min) + min) as f64
}

#[wasm_bindgen]
pub fn rand_int(n: i32) -> i32 {
    (js_sys::Math::random() * n as f64) as i32
}

#[wasm_bindgen]
pub fn rand_dir() -> i32 {
    let i = rand_int(1000);
    (i % 3) - 1
}

#[wasm_bindgen]
pub struct Snowflake {
    maxWidth: f64,
    maxHeight: f64,
    x: f64,
    y: f64,
    velocity_x: f64,
    velocity_y: f64,
    radius: f64,
    alpha: f64
}

#[wasm_bindgen]
impl Snowflake {

    pub fn tick(&mut self) {
        self.x = self.x + self.velocity_x;
        self.y = self.y + self.velocity_y;

        if(self.y + self.radius > self.maxHeight) {
            self.x = rand_between(0.0, self.maxWidth);
            self.y = rand_between(0.0, -self.maxHeight);
            self.velocity_x = rand_between(-3.0, 3.0);
            self.velocity_y = rand_between(2.0, 5.0);
            self.radius = rand_between(1.0, 2.0);
            self.alpha = rand_between(0.1, 0.9);
        }
    }

    pub fn new(maxWidth: f64, maxHeight: f64) -> Snowflake {
        Snowflake {
            maxWidth: maxWidth,
            maxHeight: maxHeight,
            x: rand_between(0.0, maxWidth),
            y: rand_between(0.0, maxHeight),
            velocity_x: rand_between(-3.0, 3.0),
            velocity_y: rand_between(2.0, 5.0),
            radius: rand_between(1.0, 4.0),
            alpha: rand_between(0.1, 0.9)
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn velocity_x(&self) -> f64 {
        self.velocity_x
    }

    pub fn velocity_y(&self) -> f64 {
        self.velocity_y
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn alpha(&self) -> f64 {
        self.alpha
    }
}