mod enum_components;
mod enum_screen;
mod st_queue_components;
mod st_queue_screen;
mod trait_objects_components;
mod trait_objects_screen;

fn main() {
    {
        println!("\n# using statically typed queue");
        use crate::st_queue_components::Draw;
        st_queue_screen::new_screen().draw();
    }

    {
        println!("\n# using trait objects");
        let screen = trait_objects_screen::new_screen();
        for component in &screen {
            component.draw();
        }
    }

    {
        use crate::enum_components::Draw;

        println!("\n# using enums");
        let screen = enum_screen::new_screen();
        for component in &screen {
            component.draw();
        }
    }
}
