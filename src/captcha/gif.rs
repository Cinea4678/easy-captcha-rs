//! GIF Format Captcha
//!
//! GIF动态验证码
//!

use crate::base::captcha::{AbstractCaptcha, Captcha};
use crate::captcha::spec::SpecCaptcha;
use crate::utils::color::Color;
use crate::NewCaptcha;
use font_kit::canvas::RasterizationOptions;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use gif::Repeat;
use raqote::{BlendMode, DrawOptions, DrawTarget, Point, SolidSource, Source, StrokeStyle};
use std::io::Write;
use std::sync::Arc;

/// GIF动态验证码
pub struct GifCaptcha {
    pub(crate) captcha: Captcha,
}

type ImageBuffer = Vec<u8>;

impl GifCaptcha {
    /// 画随机码图
    ///
    /// fontColor 随机字体颜色
    // 	strs 字符数组
    // 	flag 透明度
    // 	besselXY 干扰线参数
    pub(crate) fn graphics_image(
        &mut self,
        color: &Vec<Color>,
        str: &Vec<char>,
        flag: usize,
        // bessel_xy: [[f32; 2]; 3],
    ) -> ImageBuffer {
        let width = self.captcha.width;
        let height = self.captcha.height;

        let mut dt = DrawTarget::new(width, height);
        let ref mut randoms = self.captcha.randoms;

        // 填充背景
        dt.fill_rect(
            0.,
            0.,
            width as f32,
            height as f32,
            &Source::Solid(SolidSource::from(raqote::Color::new(255, 255, 255, 255))),
            &DrawOptions::new(),
        );

        // 画干扰圆
        let alpha = 0.1 * randoms.num(10) as f32;
        self.captcha.draw_oval_with_option(
            2,
            &mut dt,
            None,
            DrawOptions {
                blend_mode: BlendMode::SrcOver,
                alpha,
                ..Default::default()
            },
        );

        // 画干扰线
        self.captcha.draw_bessel_line_with_all_option(
            1,
            &mut dt,
            None,
            StrokeStyle {
                width: 1.2,
                ..Default::default()
            },
            DrawOptions {
                blend_mode: BlendMode::SrcOver,
                alpha: 0.7,
                ..Default::default()
            },
        );

        // 画验证码
        let font = self.captcha.get_font();
        let font_size = self.captcha.get_font_size();
        let glyph = font.glyph_for_char('W').unwrap();
        let bounds = font
            .raster_bounds(
                glyph,
                font_size,
                Default::default(),
                HintingOptions::None,
                RasterizationOptions::GrayscaleAa,
            )
            .unwrap();

        let mut text_draw_options = DrawOptions {
            blend_mode: BlendMode::SrcOver,
            ..Default::default()
        };

        let f_w = width / str.len() as i32; // 每个字符所占宽度
        let f_sp = (f_w - bounds.width()) / 2; // 字符的左右边距
        for (i, ch) in str.iter().enumerate() {
            let mut color = color[i].clone();
            let alpha = self.get_alpha(flag, i);
            color.set_alpha(alpha as f64);

            let color: raqote::Color = color.into();
            let glyph = font.glyph_for_char(ch.clone());
            if glyph.is_none() {
                continue;
            }

            let bounds = font
                .raster_bounds(
                    glyph.unwrap(),
                    font_size,
                    Default::default(),
                    HintingOptions::None,
                    RasterizationOptions::GrayscaleAa,
                )
                .unwrap();

            let f_y = height - ((height - bounds.height() as i32) >> 1);
            text_draw_options.alpha = alpha;

            dt.draw_glyphs(
                &font,
                font_size,
                &[glyph.unwrap()],
                &[Point::new(
                    (i as i32 * f_w + f_sp + 3) as f32,
                    f_y as f32 - 3.,
                )],
                &Source::Solid(SolidSource::from(color)),
                &text_draw_options,
            )
        }

        Vec::from(dt.get_data_u8())
    }

    /// 获取透明度,从0到1,自动计算步长
    fn get_alpha(&self, i: usize, j: usize) -> f32 {
        let len = self.captcha.len;

        let num = i + j;
        let r = 1. / (len - 1) as f32;
        let s = len as f32 * r;
        if num >= len {
            num as f32 * r - s
        } else {
            num as f32 * r
        }
    }
}

impl NewCaptcha for GifCaptcha {
    fn new() -> Self {
        Self {
            captcha: Captcha::new(),
        }
    }

    fn with_size(width: i32, height: i32) -> Self {
        Self {
            captcha: Captcha::with_size(width, height),
        }
    }

    fn with_size_and_len(width: i32, height: i32, len: usize) -> Self {
        Self {
            captcha: Captcha::with_size_and_len(width, height, len),
        }
    }

    fn with_all(width: i32, height: i32, len: usize, font: &Arc<Font>, font_size: f32) -> Self {
        Self {
            captcha: Captcha::with_all(width, height, len, font, font_size),
        }
    }
}

impl AbstractCaptcha for GifCaptcha {
    type Error = gif::EncodingError;

    fn out(&mut self, out: impl Write) -> Result<(), Self::Error> {
        let str = self.captcha.text_char();

        // 随机生成每个文字的颜色
        let font_color: Vec<_> = str.iter().map(|_| self.captcha.color()).collect();

        // 开始画gif的每一帧
        let width = self.captcha.width as u16;
        let height = self.captcha.height as u16;
        let mut encoder = gif::Encoder::new(out, width, height, &[])?;
        encoder.set_repeat(Repeat::Infinite)?;
        for i in 0..self.captcha.len {
            let mut image = self.graphics_image(&font_color, &str, i);
            let mut frame = gif::Frame::from_rgba_speed(width, height, &mut image, 10);
            frame.delay = 10;
            encoder.write_frame(&frame)?;
        }

        Ok(())
    }

    fn get_chars(&mut self) -> Vec<char> {
        self.captcha.text_char()
    }

    fn base64(&mut self) -> Result<String, Self::Error> {
        self.base64_with_head("data:image/gif;base64,")
    }

    fn get_content_type(&mut self) -> String {
        "image/gif".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn it_works() {
        let mut file = File::create("test.gif").unwrap();
        let mut captcha = GifCaptcha::new();
        captcha.out(&mut file).unwrap();
        println!("{}", captcha.captcha.chars.unwrap())
    }
}
