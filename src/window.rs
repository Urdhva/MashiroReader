/* window.rs
 *
 * Copyright 2025 urdhva
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};       //braces for multiple imports from the same crate
use gtk::{Button, FileDialog, Window};



mod imp {
    //connects the xml blueprint and logic (rust code)
    use super::*;

    // '#' defines a macro attribute. Basically adds metadata to the code below it
    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/Urdhva/MashiroReader/window.ui")]
    pub struct MashiroreaderWindow {
        //we add UI elements here
        // Template widgets. pub <id>: TemplateChild<gtk::<gtk class>>,
        #[template_child]
        pub open_button: TemplateChild<gtk::Button>,

        #[template_child]
        pub textview: TemplateChild<gtk::TextView>,

    }

    //all the user defined functions go here
    impl MashiroreaderWindow {
        fn setup_file_picker(&self) {
            let file_dialog = FileDialog::builder()
                .title("Select a file")
                .accept_label("Open")
                .modal(true)
                .build();

            // Cast to adw::ApplicationWindow first, then to gtk::Window
            let app_window: adw::ApplicationWindow = self.obj().clone().upcast();
            let window: Window = app_window.upcast();

            glib::spawn_future_local(async move {
                let result = file_dialog.open_future(Some(&window)).await;

                match result {
                    Ok(file) => {
                        if let Some(path) = file.path() {
                            println!("User selected file {:?}", path);
                        }
                    }
                    Err(error) => {
                        println!("Error or cancellation occurred: {:?}", error);
                    }
                }
            });
        }
    }

    //to add a UI button, add another template class

    //object subclass trait binds the struct to the gobject system
    //it defines 'what' our object is
    //"imp for" basically means we're defining a trait behavior
    #[glib::object_subclass]
    impl ObjectSubclass for MashiroreaderWindow {

        const NAME: &'static str = "MashiroreaderWindow";
        type Type = super::MashiroreaderWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    //methods for struct MashiroreaderWindow
    //ObjectImpl is a trait
    impl ObjectImpl for MashiroreaderWindow {
        //so what I need to do -> Capture the signal.
        //-> Open the file explorer
        //-> Connect both these actions

        fn constructed(&self){
            self.parent_constructed();

            self.open_button.connect_clicked(glib::clone!(@weak self as window => move |_| {
                println!("button clicked");

                //now call method on the window instance
                window.setup_file_picker();
            }));
        }
    }

    impl WidgetImpl for MashiroreaderWindow {}
    impl WindowImpl for MashiroreaderWindow {}
    impl ApplicationWindowImpl for MashiroreaderWindow {}
    impl AdwApplicationWindowImpl for MashiroreaderWindow {}
    //impl IsA<gtk::Window> for MashiroreaderWindow {}

}

glib::wrapper! {
    pub struct MashiroreaderWindow(ObjectSubclass<imp::MashiroreaderWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl MashiroreaderWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
