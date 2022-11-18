extern crate piston_window;

use piston_window::*;

mod non_zero;
use non_zero::*;
mod animal;
use animal::*;

fn main() {
    let mut x = NonZeroU8::new(3).unwrap();
    x.clone().mutate_and_consume();
    x.clone().mutate_and_consume();
    x.clone().mutate_and_consume();
    x.clone().mutate_and_consume();
    x.mutate_and_consume();

    // let cachorro = Cachorro;
    // let gato = Gato;

    // gato.fazer_barulho();
    // cachorro.fazer_barulho();

    // let b = {
    //     let c = "Cagada";

    //     c.len()
    // };
    // dbg!(b);
    // let a = unsafe {
    //     NonZeroU8::new_unchecked(0)
    // };
    // dbg!(a);

    // let a = NonZeroU8::new(3).unwrap();
    // dbg!(&a);
    // println!("{a}");
    // let mut window: PistonWindow =
    //     WindowSettings::new("Snake", (512, 256))
    //         .build().unwrap();
    // while let Some(e) = window.next() {
    //     window.draw_2d(&e, |c, g, _| {
    //         clear([0.5, 0.5, 0.5, 1.0], g);
    //         rectangle([1.0, 0.0, 0.0, 1.0], // red
    //                   [0.0, 0.0, 100.0, 100.0], // rectangle
    //                   c.transform, g);
    //     });
    // }
}

fn foo(x: bool) -> u8 {
    if x {
        return 0; // Retorna 0
    }

    4 // Retorna 4
}