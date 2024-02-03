use crate::base::randoms::Randoms;
use crate::utils;
use crate::utils::color::Color;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use font_kit::font::Font;
use font_kit::loaders::default::NativeFont;
use font_kit::properties::Style;
use log::error;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source, StrokeStyle};
use std::io::Write;
use std::sync::Arc;

/// 验证码抽象类
pub(crate) struct Captcha {
    /// 随机数工具类
    randoms: Randoms,

    /// 常用颜色
    color: Vec<Color>,

    /// 字体名称
    font_names: [&'static str; 10],

    /// 验证码的字体
    font: Option<Arc<Font>>,

    /// 验证码的字体大小
    font_size: f32,

    /// 验证码随机字符长度
    pub len: usize,

    /// 验证码显示宽度
    pub width: i32,

    /// 验证码显示高度
    pub height: i32,

    /// 验证码类型
    char_type: CaptchaType,

    /// 当前验证码
    chars: Option<String>,
}

/// 验证码文本类型
pub enum CaptchaType {
    /// 字母数字混合
    TypeDefault = 1,

    /// 纯数字
    TypeOnlyNumber,

    /// 纯字母
    TypeOnlyChar,

    /// 纯大写字母
    TypeOnlyUpper,

    /// 纯小写字母
    TypeOnlyLower,

    /// 数字大写字母
    TypeNumAndUpper,
}

/// 内置字体
pub enum CaptchaFont {
    Font1,
    Font2,
    Font3,
    Font4,
    Font5,
    Font6,
    Font7,
    Font8,
    Font9,
    Font10,
}

impl Captcha {
    pub fn new() -> Self {
        let color = [
            (0, 135, 255),
            (51, 153, 51),
            (255, 102, 102),
            (255, 153, 0),
            (153, 102, 0),
            (153, 102, 153),
            (51, 153, 153),
            (102, 102, 255),
            (0, 102, 204),
            (204, 51, 51),
            (0, 153, 204),
            (0, 51, 102),
        ]
        .iter()
        .map(|v| (*v).into())
        .collect();

        let font_names = [
            "actionj.ttf",
            "epilog.ttf",
            "fresnel.ttf",
            "headache.ttf",
            "lexo.ttf",
            "prefix.ttf",
            "progbot.ttf",
            "ransom.ttf",
            "robot.ttf",
            "scandal.ttf",
        ];

        let font = None;
        let font_size = 32.;
        let len = 5;
        let width = 130;
        let height = 48;
        let char_type = CaptchaType::TypeDefault;
        let chars = None;

        Self {
            randoms: Randoms::new(),
            color,
            font_names,
            font,
            font_size,
            len,
            width,
            height,
            char_type,
            chars,
        }
    }

    /// 生成随机验证码
    pub fn alphas(&mut self) -> Vec<char> {
        let mut cs = vec!['\0'; self.len];
        for i in 0..self.len {
            match self.char_type {
                CaptchaType::TypeDefault => cs[i] = self.randoms.alpha(),
                CaptchaType::TypeOnlyNumber => {
                    cs[i] = self.randoms.alpha_under(self.randoms.num_max_index)
                }
                CaptchaType::TypeOnlyChar => {
                    cs[i] = self
                        .randoms
                        .alpha_between(self.randoms.char_min_index, self.randoms.char_max_index)
                }
                CaptchaType::TypeOnlyUpper => {
                    cs[i] = self
                        .randoms
                        .alpha_between(self.randoms.upper_min_index, self.randoms.upper_max_index)
                }
                CaptchaType::TypeOnlyLower => {
                    cs[i] = self
                        .randoms
                        .alpha_between(self.randoms.lower_min_index, self.randoms.lower_max_index)
                }
                CaptchaType::TypeNumAndUpper => {
                    cs[i] = self.randoms.alpha_under(self.randoms.upper_max_index)
                }
            }
        }

        self.chars = Some(cs.iter().collect());
        cs
    }

    /// 给定范围获得随机颜色
    pub fn color_range(&mut self, fc: u8, bc: u8) -> Color {
        let fc = if fc > 255 { 255 } else { fc };
        let bc = if bc > 255 { 255 } else { bc };

        let r = fc + self.randoms.num((bc - fc) as usize) as u8;
        let g = fc + self.randoms.num((bc - fc) as usize) as u8;
        let b = fc + self.randoms.num((bc - fc) as usize) as u8;
        return (r, g, b).into();
    }

    /// 获取随机常用颜色
    pub fn color(&mut self) -> Color {
        self.color[self.randoms.num(self.color.len())].clone()
    }

    /// 获取当前的验证码
    pub fn text(&mut self) -> String {
        self.check_alpha();
        self.chars.clone().unwrap()
    }

    /// 获取当前验证码的字符数组
    pub fn text_char(&mut self) -> Vec<char> {
        self.check_alpha();
        self.chars.clone().unwrap().chars().collect()
    }

    /// 检查验证码是否生成，没有则立即生成
    pub fn check_alpha(&mut self) {
        if self.chars.is_none() {
            self.alphas();
        }
    }

    /// 随机画干扰线
    pub fn draw_line(&mut self, num: usize, g: &mut DrawTarget, color: Option<Color>) {
        for _ in 0..num {
            let color = color.clone().unwrap_or_else(|| self.color());
            let color: raqote::Color = color.into();

            let x1 = self.randoms.num_between(-10, self.width - 10);
            let y1 = self.randoms.num_between(5, self.height - 5);
            let x2 = self.randoms.num_between(10, self.width + 10);
            let y2 = self.randoms.num_between(2, self.height - 2);

            let mut pb = PathBuilder::new();
            pb.move_to(x1 as f32, y1 as f32);
            pb.line_to(x2 as f32, y2 as f32);
            let path = pb.finish();

            g.stroke(
                &path,
                &Source::Solid(SolidSource::from(color)),
                &StrokeStyle {
                    width: 2.,
                    ..Default::default()
                },
                &DrawOptions::new(),
            );
        }
    }

    /// 随机画干扰圆
    pub fn draw_oval(&mut self, num: usize, g: &mut DrawTarget, color: Option<Color>) {
        for _ in 0..num {
            let color = color.clone().unwrap_or_else(|| self.color());
            let color: raqote::Color = color.into();

            let w = 5 + self.randoms.num(10);
            let x = self.randoms.num(self.width as usize - 25) + w;
            let y = self.randoms.num(self.height as usize - 15) + w;

            let mut pb = PathBuilder::new();
            pb.arc(x as f32, y as f32, w as f32, 0., 2. * std::f32::consts::PI);
            let path = pb.finish();

            g.stroke(
                &path,
                &Source::Solid(SolidSource::from(color)),
                &StrokeStyle {
                    width: 2.,
                    ..Default::default()
                },
                &DrawOptions::new(),
            );
        }
    }

    /// 随机画干扰贝塞尔曲线
    pub fn draw_bessel_line(&mut self, num: usize, g: &mut DrawTarget, color: Option<Color>) {
        for _ in 0..num {
            let color = color.clone().unwrap_or_else(|| self.color());
            let color: raqote::Color = color.into();

            let x1 = 5;
            let mut y1 = self.randoms.num_between(5, self.height / 2);
            let x2 = self.width - 5;
            let mut y2 = self.randoms.num_between(self.height / 2, self.height - 5);

            let cx = self.randoms.num_between(self.width / 4, self.width / 4 * 3);
            let cy = self.randoms.num_between(5, self.height - 5);

            if self.randoms.num(2) == 0 {
                (y2, y1) = (y1, y2)
            }

            let mut pb = PathBuilder::new();
            pb.move_to(x1 as f32, y1 as f32);

            if self.randoms.num(2) == 0 {
                // 二阶曲线
                pb.quad_to(cx as f32, cy as f32, x2 as f32, y2 as f32);
            } else {
                // 三阶曲线
                let cx1 = self.randoms.num_between(self.width / 4, self.width / 4 * 3);
                let cy1 = self.randoms.num_between(5, self.height - 5);
                pb.cubic_to(
                    cx as f32, cy as f32, cx1 as f32, cy1 as f32, x2 as f32, y2 as f32,
                );
            }

            let path = pb.finish();

            g.stroke(
                &path,
                &Source::Solid(SolidSource::from(color)),
                &StrokeStyle {
                    width: 2.,
                    ..Default::default()
                },
                &DrawOptions::new(),
            );
        }
    }

    pub fn get_font(&mut self) -> Arc<Font> {
        if self.font.is_none() {
            self.set_font_by_enum(CaptchaFont::Font1, None);
        }
        return self.font.clone().unwrap();
    }

    pub fn get_font_size(&mut self) -> f32 {
        self.font_size
    }

    pub fn set_font_by_font(&mut self, font: Arc<Font>, size: Option<f32>) {
        self.font = Some(font);
        self.font_size = size.unwrap_or(32.);
    }

    pub fn set_font_by_enum(&mut self, font: CaptchaFont, size: Option<f32>) {
        let font_name = self.font_names[font as usize];
        match utils::font::get_font(font_name) {
            Some(font) => {
                self.font = Some(font);
                self.font_size = size.unwrap_or(32.);
            }
            None => {
                error!("Set font by enum failed.")
            }
        }
    }
}

/// 验证码的抽象方法
pub trait AbstractCaptcha {
    /// 验证码输出,抽象方法，由子类实现
    fn out(&mut self, os: impl Write) -> bool;

    /// 输出Base64编码
    fn base64(&mut self) -> String;

    /// 获取图片类型
    fn get_content_type(&mut self) -> String;

    /// 输出Base64编码（包含编码头）
    fn base64_with_head(&mut self, _type: &str) -> String {
        let mut output_stream = Vec::new();
        self.out(&mut output_stream);
        String::from(_type) + &BASE64_STANDARD.encode(&output_stream)
    }
}
