use parking_lot::Mutex;
use std::{cell::RefCell, rc::Rc, sync::Arc};

use futures::channel::oneshot;

use crate::{
    px, AnyWindowHandle, MacPlatform, Pixels, Platform, PlatformAtlas, PlatformDisplay,
    PlatformWindow, Point, Scene, Size, WindowAppearance, WindowBounds, WindowOptions,
};

pub struct TestWindowState {
    pub(crate) options: WindowOptions,
    pub(crate) handle: AnyWindowHandle,
    pub(crate) current_scene: Option<Scene>,
    pub(crate) display: Rc<dyn PlatformDisplay>,
    pub(crate) sprite_atlas: Arc<dyn PlatformAtlas>,

    on_input: Option<Box<dyn FnMut(crate::InputEvent) -> bool>>,
    on_moved: Option<Box<dyn FnMut()>>,
    on_resize: Option<Box<dyn FnMut(Size<Pixels>, f32)>>,
    on_active_status_change: Option<Box<dyn FnMut(bool)>>,
}

#[derive(Clone)]
pub struct TestWindow(pub(crate) Arc<Mutex<TestWindowState>>);

impl TestWindow {
    pub fn new(
        options: WindowOptions,
        handle: AnyWindowHandle,
        display: Rc<dyn PlatformDisplay>,
    ) -> Self {
        TestWindow(Arc::new(Mutex::new(TestWindowState {
            options,
            current_scene: Default::default(),
            display,
            handle,

            sprite_atlas: Arc::new(TestAtlas),
            on_input: Default::default(),
            on_moved: Default::default(),
            on_resize: Default::default(),
            on_active_status_change: Default::default(),
        })))
    }

    pub async fn reveal(&self) {
        let mac_platform = Rc::new(MacPlatform::new());
        let this = self;

        let handle = this.0.lock().handle.clone();
        let options = this.0.lock().options.clone();
        let scene = this.0.lock().current_scene.take().unwrap();
        let mp2 = mac_platform.clone();

        let (sender, receiver) = oneshot::channel();

        dbg!("ohai");
        mac_platform.run(Box::new(move || {
            dbg!("oha2");
            let window = mp2.clone().open_window(handle, options);
            dbg!("oha3");
            window.draw(scene);
            dbg!("oha4");
            sender.send(()).unwrap();
        }));

        receiver.await.unwrap()
    }
}

impl PlatformWindow for TestWindow {
    fn bounds(&self) -> WindowBounds {
        self.0.lock().options.bounds
    }

    fn content_size(&self) -> Size<Pixels> {
        let bounds = match self.bounds() {
            WindowBounds::Fixed(bounds) => bounds,
            WindowBounds::Maximized | WindowBounds::Fullscreen => self.display().bounds(),
        };
        bounds.size.map(|p| px(p.0))
    }

    fn scale_factor(&self) -> f32 {
        2.0
    }

    fn titlebar_height(&self) -> Pixels {
        todo!()
    }

    fn appearance(&self) -> WindowAppearance {
        todo!()
    }

    fn display(&self) -> std::rc::Rc<dyn crate::PlatformDisplay> {
        self.0.lock().display.clone()
    }

    fn mouse_position(&self) -> Point<Pixels> {
        Point::zero()
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        todo!()
    }

    fn set_input_handler(&mut self, _input_handler: Box<dyn crate::PlatformInputHandler>) {
        todo!()
    }

    fn prompt(
        &self,
        _level: crate::PromptLevel,
        _msg: &str,
        _answers: &[&str],
    ) -> futures::channel::oneshot::Receiver<usize> {
        todo!()
    }

    fn activate(&self) {
        todo!()
    }

    fn set_title(&mut self, _title: &str) {
        todo!()
    }

    fn set_edited(&mut self, _edited: bool) {
        todo!()
    }

    fn show_character_palette(&self) {
        todo!()
    }

    fn minimize(&self) {
        todo!()
    }

    fn zoom(&self) {
        todo!()
    }

    fn toggle_full_screen(&self) {
        todo!()
    }

    fn on_input(&self, callback: Box<dyn FnMut(crate::InputEvent) -> bool>) {
        self.0.lock().on_input.replace(callback);
    }

    fn on_active_status_change(&self, callback: Box<dyn FnMut(bool)>) {
        self.0.lock().on_active_status_change.replace(callback);
    }

    fn on_resize(&self, callback: Box<dyn FnMut(Size<Pixels>, f32)>) {
        self.0.lock().on_resize.replace(callback);
    }

    fn on_fullscreen(&self, _callback: Box<dyn FnMut(bool)>) {
        todo!()
    }

    fn on_moved(&self, callback: Box<dyn FnMut()>) {
        self.0.lock().on_moved.replace(callback);
    }

    fn on_should_close(&self, _callback: Box<dyn FnMut() -> bool>) {
        todo!()
    }

    fn on_close(&self, _callback: Box<dyn FnOnce()>) {
        todo!()
    }

    fn on_appearance_changed(&self, _callback: Box<dyn FnMut()>) {
        todo!()
    }

    fn is_topmost_for_position(&self, _position: crate::Point<Pixels>) -> bool {
        todo!()
    }

    fn draw(&self, scene: crate::Scene) {
        self.0.lock().current_scene.replace(scene);
    }

    fn sprite_atlas(&self) -> std::sync::Arc<dyn crate::PlatformAtlas> {
        self.0.lock().sprite_atlas.clone()
    }
}

pub struct TestAtlas;

impl PlatformAtlas for TestAtlas {
    fn get_or_insert_with<'a>(
        &self,
        _key: &crate::AtlasKey,
        _build: &mut dyn FnMut() -> anyhow::Result<(
            Size<crate::DevicePixels>,
            std::borrow::Cow<'a, [u8]>,
        )>,
    ) -> anyhow::Result<crate::AtlasTile> {
        todo!()
    }

    fn clear(&self) {
        todo!()
    }
}
