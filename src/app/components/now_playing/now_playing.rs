use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use std::rc::Rc;

use super::NowPlayingModel;
use crate::app::components::{Component, EventListener, Playlist};
use crate::app::{state::PlaybackEvent, AppEvent, Worker};
use libadwaita::subclass::prelude::BinImpl;

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/dev/alextren/Spot/components/now_playing.ui")]
    pub struct NowPlayingWidget {
        #[template_child]
        pub song_list: TemplateChild<gtk::ListView>,

        #[template_child]
        pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NowPlayingWidget {
        const NAME: &'static str = "NowPlayingWidget";
        type Type = super::NowPlayingWidget;
        type ParentType = libadwaita::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for NowPlayingWidget {}
    impl WidgetImpl for NowPlayingWidget {}
    impl BinImpl for NowPlayingWidget {}
}

glib::wrapper! {
    pub struct NowPlayingWidget(ObjectSubclass<imp::NowPlayingWidget>) @extends gtk::Widget, libadwaita::Bin;
}

impl NowPlayingWidget {
    fn new() -> Self {
        glib::Object::new()
    }

    fn connect_bottom_edge<F>(&self, f: F)
    where
        F: Fn() + 'static,
    {
        self.imp()
            .scrolled_window
            .connect_edge_reached(move |_, pos| {
                if let gtk::PositionType::Bottom = pos {
                    f()
                }
            });
    }

    fn song_list_widget(&self) -> &gtk::ListView {
        self.imp().song_list.as_ref()
    }
}

pub struct NowPlaying {
    widget: NowPlayingWidget,
    model: Rc<NowPlayingModel>,
    children: Vec<Box<dyn EventListener>>,
}

impl NowPlaying {
    pub fn new(model: Rc<NowPlayingModel>, worker: Worker) -> Self {
        let widget = NowPlayingWidget::new();

        widget.connect_bottom_edge(clone!(@weak model => move || {
            model.load_more();
        }));

        let playlist = Playlist::new(widget.song_list_widget().clone(), model.clone(), worker);

        Self {
            widget,
            model,
            children: vec![Box::new(playlist)],
        }
    }
}

impl Component for NowPlaying {
    fn get_root_widget(&self) -> &gtk::Widget {
        self.widget.upcast_ref()
    }

    fn get_children(&mut self) -> Option<&mut Vec<Box<dyn EventListener>>> {
        Some(&mut self.children)
    }
}

impl EventListener for NowPlaying {
    fn on_event(&mut self, event: &AppEvent) {
        if let AppEvent::PlaybackEvent(PlaybackEvent::TrackChanged(_)) = event {
            self.model.load_more();
        }
        self.broadcast_event(event);
    }
}
