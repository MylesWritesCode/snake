use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};

mod snake;
use snake::SnakeGame;
use web_sys::{console, window, Document, HtmlDivElement, HtmlElement};

thread_local! {

    static GAME: Rc<RefCell<SnakeGame>> = Rc::new(RefCell::new(SnakeGame::new(20, 20)));

    static TICK_CLOSURE: Closure<dyn FnMut()> = Closure::wrap(Box::new({
        let game = GAME.with(|game| game.clone());
        move || {
            game.borrow_mut().tick();
            render();
        }
    }) as Box<dyn FnMut()>);

}

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Starting web server...".into());

    TICK_CLOSURE.with(|closure| {
        window()
            .unwrap_throw()
            .set_interval_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                500,
            )
            .unwrap_throw()
    });

    render();
}

// @note This is kinda nuts. Doing this in JS just seems better
pub fn render() {
    let document = window().unwrap_throw().document().unwrap_throw();
    let root = document
        .get_element_by_id("root".into())
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();

    // Clear root element
    root.set_inner_html("");

    let width = GAME.with(|game| game.borrow().width);
    let height = GAME.with(|game| game.borrow().height);

    root.style()
        .set_property("display", "inline-grid")
        .unwrap_throw();
    root.style()
        .set_property(
            "grid-template",
            &format!("repeat({}, auto) / repeat({}, auto)", width, height),
        )
        .unwrap_throw();

    for y in 0..height {
        for x in 0..width {
            let position = (x, y);
            let el = document
                .create_element("div")
                .unwrap_throw()
                .dyn_into::<HtmlDivElement>()
                .unwrap_throw();

            el.set_inner_text({
                if position == GAME.with(|game| game.borrow().food) {
                    "🐀"
                } else if GAME.with(|game| game.borrow().snake.contains(&position)) {
                    "🟩"
                } else {
                    "⬜"
                }
            });

            root.append_child(&el).unwrap_throw();
        }
    }
}
