use crate::base::randoms::Randoms;

use crate::utils::color::Color;
use crate::utils::font;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use font_kit::font::Font;

use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source, StrokeStyle};
use std::fmt::Debug;
use std::io::Write;
use std::sync::Arc;

/// 验证码抽象类
pub(crate) struct Captcha {
    /// 随机数工具类
    pub(crate) randoms: Randoms,

    /// 常用颜色
    color: Vec<Color>,

    /// 字体名称
    font_names: [&'static str; 10],

    /// 验证码的字体
    font_name: String,

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
    pub(crate) chars: Option<String>,
}

/// 验证码文本类型 The character type of the captcha
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

/// 内置字体 Fonts shipped with the library
pub enum CaptchaFont {
    /// actionj
    Font1,
    /// epilog
    Font2,
    /// fresnel
    Font3,
    /// headache
    Font4,
    /// lexo
    Font5,
    /// prefix
    Font6,
    /// progbot
    Font7,
    /// ransom
    Font8,
    /// robot
    Font9,
    /// scandal
    Font10,
}

impl Captcha {
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
        self.draw_oval_with_option(num, g, color, DrawOptions::new());
    }

    /// 随机画干扰圆（包含选项）
    pub fn draw_oval_with_option(
        &mut self,
        num: usize,
        g: &mut DrawTarget,
        color: Option<Color>,
        options: DrawOptions,
    ) {
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
                &options,
            );
        }
    }

    /// 随机画干扰贝塞尔曲线
    pub fn draw_bessel_line(&mut self, num: usize, g: &mut DrawTarget, color: Option<Color>) {
        self.draw_bessel_line_with_all_option(
            num,
            g,
            color,
            StrokeStyle {
                width: 2.,
                ..Default::default()
            },
            DrawOptions::new(),
        )
    }

    /// 随机画干扰贝塞尔曲线（包含所有选项）
    pub fn draw_bessel_line_with_all_option(
        &mut self,
        num: usize,
        g: &mut DrawTarget,
        color: Option<Color>,
        stroke_style: StrokeStyle,
        draw_options: DrawOptions,
    ) {
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
                &stroke_style,
                &draw_options,
            );
        }
    }

    pub fn get_font(&mut self) -> Arc<Font> {
        if let Some(font) = font::get_font(&self.font_name) {
            font
        } else {
            font::get_font(self.font_names[0]).unwrap()
        }
    }

    pub fn get_font_size(&mut self) -> f32 {
        self.font_size
    }

    pub fn set_font_by_enum(&mut self, font: CaptchaFont, size: Option<f32>) {
        let font_name = self.font_names[font as usize];
        self.font_name = font_name.into();
        self.font_size = size.unwrap_or(32.);
    }
}

/// 初始化验证码的抽象方法 Traits for initialize a Captcha instance.
pub trait NewCaptcha
where
    Self: Sized,
{
    /// 用默认参数初始化
    ///
    /// Initialize the Captcha with the default properties.
    fn new() -> Self;

    /// 使用输出图像大小初始化
    ///
    /// Initialize the Captcha with the size of output image.
    fn with_size(width: i32, height: i32) -> Self;

    /// 使用输出图像大小和验证码字符长度初始化
    ///
    /// Initialize the Captcha with the size of output image and the character length of the Captcha.
    ///
    /// <br/>
    ///
    /// 特别地/In particular:
    ///
    /// - 对算术验证码[ArithmeticCaptcha](crate::captcha::arithmetic::ArithmeticCaptcha)而言，这里的`len`是验证码中数字的数量。
    /// For [ArithmeticCaptcha](crate::captcha::arithmetic::ArithmeticCaptcha), the `len` presents the count of the digits
    /// in the Captcha.
    fn with_size_and_len(width: i32, height: i32, len: usize) -> Self;

    /// 使用完整的参数来初始化，包括输出图像大小、验证码字符长度和输出字体及其大小
    ///
    /// Initialize the Captcha with full properties, including the size of output image, the character length of the Captcha,
    /// and the font used in Captcha with the font size.
    ///
    /// 关于`len`字段的注意事项，请参见[with_size_and_len](Self::with_size_and_len)中的说明。Refer to the document of
    /// [with_size_and_len](Self::with_size_and_len) for the precautions of the `len` property.
    fn with_all(width: i32, height: i32, len: usize, font: CaptchaFont, font_size: f32) -> Self;
}

impl NewCaptcha for Captcha {
    fn new() -> Self {
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

        let font_name = font_names[0].into();
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
            font_name,
            font_size,
            len,
            width,
            height,
            char_type,
            chars,
        }
    }

    fn with_size(width: i32, height: i32) -> Self {
        let mut _self = Self::new();
        _self.width = width;
        _self.height = height;
        _self
    }

    fn with_size_and_len(width: i32, height: i32, len: usize) -> Self {
        let mut _self = Self::new();
        _self.width = width;
        _self.height = height;
        _self.len = len;
        _self
    }

    fn with_all(width: i32, height: i32, len: usize, font: CaptchaFont, font_size: f32) -> Self {
        let mut _self = Self::new();
        _self.width = width;
        _self.height = height;
        _self.len = len;
        _self.set_font_by_enum(font, None);
        _self.font_size = font_size;
        _self
    }
}

/// 验证码的抽象方法  Traits which a Captcha must implements.
pub trait AbstractCaptcha: NewCaptcha {
    /// 错误类型
    type Error: std::error::Error + Debug + Send + Sync + 'static;

    /// 输出验证码到指定位置
    ///
    /// Write the Captcha image to the specified place.
    fn out(&mut self, out: impl Write) -> Result<(), Self::Error>;

    /// 获取验证码中的字符（即正确答案）
    ///
    /// Get the characters (i.e. the correct answer) of the Captcha
    fn get_chars(&mut self) -> Vec<char>;

    /// 输出Base64编码。注意，返回值会带编码头（例如`data:image/png;base64,`），可以直接在浏览器中显示；如不需要编码头，
    /// 请使用[base64_with_head](Self::base64_with_head)方法并传入空参数以去除编码头。
    ///
    /// Get the Base64 encoded image. Reminds: the returned Base64 strings will begin with an encoding head like
    /// `data:image/png;base64,`, which make it possible to display in browsers directly. If you don't need it, you may
    /// use [base64_with_head](Self::base64_with_head) and pass a null string.
    fn base64(&mut self) -> Result<String, Self::Error>;

    /// 获取验证码的MIME类型
    ///
    /// Get the MIME Content type of the Captcha.
    fn get_content_type(&mut self) -> String;

    /// 输出Base64编码（指定编码头）
    ///
    /// Get the Base64 encoded image, with specified encoding head.
    fn base64_with_head(&mut self, head: &str) -> Result<String, Self::Error> {
        let mut output_stream = Vec::new();
        self.out(&mut output_stream)?;
        Ok(String::from(head) + &BASE64_STANDARD.encode(&output_stream))
    }
}
