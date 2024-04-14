use crate::widgets::{
    custom_box::Box,
    custom_button::Button,
};

use crate::factory::{
    shell::exec_once,
    scripts::{
        JUMP_TO_DESKTOP,
        GET_NUMBER_OF_DESKTOPS,   
    },
};

use gtk::prelude::*;

pub fn desktop_box(
    name: &str,
    orientation: gtk::Orientation,
    label_vector: Vec<&str>
    ) -> Box {

    let mut buttons = vec![];

    for i in 1..=label_vector.len() {
        let button_name = String::from("desktop") + &i.to_string();
        buttons.push(Button::new(&button_name));
    }

    let nod = match exec_once(GET_NUMBER_OF_DESKTOPS).parse::<i32>() {
        Ok(n) => n,
        Err(_) => -1,
    };

    let mut i = 0;

    for btn in &buttons {
        btn.set_label(label_vector[i as usize]);


        if i < nod {
            btn.connect_clicked(move |_| {
                let desktop = &i.to_string();
                let cmd_head = String::from(JUMP_TO_DESKTOP);

                exec_once(&(cmd_head + desktop));
            });
        }

        i += 1;
    }

    Box::new(name, orientation).load(buttons)
}
