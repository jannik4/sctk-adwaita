use smithay_client_toolkit::window::ButtonState;
use tiny_skia::{FillRule, PathBuilder, PixmapMut, Rect, Stroke, Transform};

use crate::{theme::ColorMap, utils, Location, SkiaResult};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonKind {
    Close,
    Maximize,
    Minimize,
}

#[derive(Default, Debug)]
pub(crate) struct Button {
    x: f32,
    y: f32,
    size: f32,
}

impl Button {
    pub fn radius(&self) -> f32 {
        self.size / 2.0
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn center_x(&self) -> f32 {
        self.x + self.radius()
    }

    pub fn center_y(&self) -> f32 {
        self.y + self.radius()
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        utils::HitBox::new_f32(self.x, self.y, self.size, self.size).contains(x, y)
    }
}

impl Buttons {
    pub fn draw_minimize(
        &self,
        scale: f32,
        colors: &ColorMap,
        mouses: &[Location],
        pixmap: &mut PixmapMut,
    ) -> SkiaResult {
        let button = &self.minimize;

        let btn_state = if mouses.contains(&Location::Button(ButtonKind::Minimize)) {
            ButtonState::Hovered
        } else {
            ButtonState::Idle
        };

        let radius = button.radius();

        let x = button.center_x();
        let y = button.center_y();

        let circle = PathBuilder::from_circle(x, y, radius)?;

        let button_bg = if btn_state == ButtonState::Hovered {
            colors.button_hover_paint()
        } else {
            colors.button_idle_paint()
        };

        pixmap.fill_path(
            &circle,
            &button_bg,
            FillRule::Winding,
            Transform::identity(),
            None,
        );

        let mut button_icon_paint = colors.button_icon_paint();
        button_icon_paint.anti_alias = false;

        let len = 8.0 * scale;
        let hlen = len / 2.0;
        pixmap.fill_rect(
            Rect::from_xywh(x - hlen, y + hlen, len, 1.0 * scale)?,
            &button_icon_paint,
            Transform::identity(),
            None,
        );

        Some(())
    }

    pub fn draw_maximize(
        &self,
        scale: f32,
        colors: &ColorMap,
        mouses: &[Location],
        maximizable: bool,
        is_maximized: bool,
        pixmap: &mut PixmapMut,
    ) -> SkiaResult {
        let button = &self.maximize;

        let btn_state = if !maximizable {
            ButtonState::Disabled
        } else if mouses
            .iter()
            .any(|&l| l == Location::Button(ButtonKind::Maximize))
        {
            ButtonState::Hovered
        } else {
            ButtonState::Idle
        };

        let radius = button.radius();

        let x = button.center_x();
        let y = button.center_y();

        let path1 = {
            let mut pb = PathBuilder::new();
            pb.push_circle(x, y, radius);
            pb.finish()?
        };

        let button_bg = if btn_state == ButtonState::Hovered {
            colors.button_hover_paint()
        } else {
            colors.button_idle_paint()
        };

        pixmap.fill_path(
            &path1,
            &button_bg,
            FillRule::Winding,
            Transform::identity(),
            None,
        );

        let path2 = {
            let size = 8.0 * scale;
            let hsize = size / 2.0;
            let mut pb = PathBuilder::new();

            let x = x - hsize;
            let y = y - hsize;
            pb.push_rect(x, y, size, size);

            if is_maximized {
                if let Some(rect) = Rect::from_xywh(x + 2.0, y - 2.0, size, size) {
                    pb.move_to(rect.left(), rect.top());
                    pb.line_to(rect.right(), rect.top());
                    pb.line_to(rect.right(), rect.bottom());
                }
            }

            pb.finish()?
        };

        let mut button_icon_paint = colors.button_icon_paint();
        button_icon_paint.anti_alias = false;
        pixmap.stroke_path(
            &path2,
            &button_icon_paint,
            &Stroke {
                width: 1.0 * scale,
                ..Default::default()
            },
            Transform::identity(),
            None,
        );

        Some(())
    }

    pub fn draw_close(
        &self,
        scale: f32,
        colors: &ColorMap,
        mouses: &[Location],
        pixmap: &mut PixmapMut,
    ) -> SkiaResult {
        let button = &self.close;

        // Draw the close button
        let btn_state = if mouses
            .iter()
            .any(|&l| l == Location::Button(ButtonKind::Close))
        {
            ButtonState::Hovered
        } else {
            ButtonState::Idle
        };

        let radius = button.radius();

        let x = button.center_x();
        let y = button.center_y();

        let path1 = {
            let mut pb = PathBuilder::new();
            pb.push_circle(x, y, radius);
            pb.finish()?
        };

        let button_bg = if btn_state == ButtonState::Hovered {
            colors.button_hover_paint()
        } else {
            colors.button_idle_paint()
        };

        pixmap.fill_path(
            &path1,
            &button_bg,
            FillRule::Winding,
            Transform::identity(),
            None,
        );

        let x_icon = {
            let size = 3.5 * scale;
            let mut pb = PathBuilder::new();

            {
                let sx = x - size;
                let sy = y - size;
                let ex = x + size;
                let ey = y + size;

                pb.move_to(sx, sy);
                pb.line_to(ex, ey);
                pb.close();
            }

            {
                let sx = x - size;
                let sy = y + size;
                let ex = x + size;
                let ey = y - size;

                pb.move_to(sx, sy);
                pb.line_to(ex, ey);
                pb.close();
            }

            pb.finish()?
        };

        let mut button_icon_paint = colors.button_icon_paint();
        button_icon_paint.anti_alias = true;
        pixmap.stroke_path(
            &x_icon,
            &button_icon_paint,
            &Stroke {
                width: 1.1 * scale,
                ..Default::default()
            },
            Transform::identity(),
            None,
        );

        Some(())
    }
}

#[derive(Debug)]
pub(crate) struct Buttons {
    pub close: Button,
    pub maximize: Button,
    pub minimize: Button,

    scale: u32,
}

impl Default for Buttons {
    fn default() -> Self {
        Self {
            close: Default::default(),
            maximize: Default::default(),
            minimize: Default::default(),
            scale: 1,
        }
    }
}

impl Buttons {
    pub fn arrange(&mut self, (margin_h, margin_v): (u32, u32), w: u32, scale: u32) {
        self.scale = scale;

        let scale = self.scale as f32;
        let margin_vertical = margin_v as f32;
        let margin_horizontal = margin_h as f32;

        let margin = 5.0 * scale;
        let spacing = 13.0 * scale;
        let size = 12.0 * 2.0 * scale;

        let mut x = w as f32 - margin - margin_horizontal;
        let y = margin + margin_vertical;

        x -= size;
        self.close.x = x;
        self.close.y = y;
        self.close.size = size;

        x -= size;
        x -= spacing;
        self.maximize.x = x;
        self.maximize.y = y;
        self.maximize.size = size;

        x -= size;
        x -= spacing;
        self.minimize.x = x;
        self.minimize.y = y;
        self.minimize.size = size;
    }

    pub fn find_button(&self, x: f64, y: f64) -> Option<ButtonKind> {
        let x = x * self.scale as f64;
        let y = y * self.scale as f64;
        if self.close.contains(x, y) {
            Some(ButtonKind::Close)
        } else if self.maximize.contains(x, y) {
            Some(ButtonKind::Maximize)
        } else if self.minimize.contains(x, y) {
            Some(ButtonKind::Minimize)
        } else {
            None
        }
    }
}
