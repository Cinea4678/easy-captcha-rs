//! Arithmetic PNG Captcha
//!
//! PNG格式算术验证码
//!

use crate::base::captcha::{AbstractCaptcha, Captcha};
use crate::captcha::gif::GifCaptcha;
use crate::captcha::spec::SpecCaptcha;
use crate::NewCaptcha;
use font_kit::font::Font;
use std::io::Write;
use std::ops::Add;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub enum Symbol {
    /// 标识符
    NUM { value: &'static str, priority: bool },
    /// 加法
    ADD { value: &'static str, priority: bool },
    /// 减法
    SUB { value: &'static str, priority: bool },
    /// 乘法
    MUL { value: &'static str, priority: bool },
    /// 除法
    DIV { value: &'static str, priority: bool },
}

impl Symbol {
    fn of(c: &str) -> Result<Self, &'static str> {
        match c {
            "n" => Ok(Symbol::NUM {
                value: "n",
                priority: false,
            }),
            "+" => Ok(Symbol::ADD {
                value: "+",
                priority: false,
            }),
            "-" => Ok(Symbol::SUB {
                value: "-",
                priority: false,
            }),
            "x" => Ok(Symbol::MUL {
                value: "x",
                priority: true,
            }),
            "÷" => Ok(Symbol::DIV {
                value: "÷",
                priority: true,
            }),
            _ => Err("不支持的标识符，仅支持(+、-、×、÷)"),
        }
    }
}

impl From<usize> for Symbol {
    fn from(value: usize) -> Symbol {
        match value {
            0 => Symbol::NUM {
                value: "n",
                priority: false,
            },
            1 => Symbol::ADD {
                value: "+",
                priority: false,
            },
            2 => Symbol::SUB {
                value: "-",
                priority: false,
            },
            3 => Symbol::MUL {
                value: "x",
                priority: true,
            },
            4 => Symbol::DIV {
                value: "÷",
                priority: true,
            },
            _ => panic!("不支持的序号，仅支持0~4；收到了：{}", value),
        }
    }
}

/// 算数验证码
pub struct ArithmeticCaptcha {
    pub(crate) spec: SpecCaptcha,

    /// 计算公式
    arithmetic_string: Option<String>,

    /// 难度
    difficulty: usize,

    /// 表达式复杂度
    algorithm_sign: usize,
}

impl ArithmeticCaptcha {
    pub fn alphas(&mut self) -> Vec<char> {
        let len = self.spec.captcha.len;
        let ref mut randoms = self.spec.captcha.randoms;

        let mut arithmetic_list = Vec::new();
        arithmetic_list.reserve(len + len - 1);

        let mut last_symbol = None;
        let mut div_amount = 0;

        for i in 0..len {
            let mut number = randoms.num(self.difficulty);

            // 如果上一步生成的为除号，要重新设置除数和被除数，确保难度满足设定要求且可以整除
            if let Some(Symbol::DIV { .. }) = last_symbol {
                number = (number as f64).sqrt() as usize;
                // 避免被除数为 0
                number = if number == 0 { 1 } else { number };
                arithmetic_list[2 * (i - 1)] =
                    (number * randoms.num((self.difficulty as f64).sqrt() as usize)).to_string();
            }

            // 如果是减法则获取一个比第一个小的数据
            if let Some(Symbol::SUB { .. }) = last_symbol {
                let ref first_num = arithmetic_list[0];
                number = randoms.num(first_num.parse::<usize>().unwrap() + 1);
            }

            arithmetic_list.push(number.to_string());

            if i < len - 1 {
                let _type: Symbol;

                // 除法只出现一次，否则还需要递归更新除数，第一个除数将会很大
                if div_amount == 1 {
                    _type =
                        (randoms.num_between(1, self.algorithm_sign as i32 - 1) as usize).into();
                } else {
                    _type = (randoms.num_between(1, self.algorithm_sign as i32) as usize).into();
                }

                match _type {
                    Symbol::NUM { .. } => { /* 不可达 */ }
                    Symbol::ADD { value, .. }
                    | Symbol::MUL { value, .. }
                    | Symbol::SUB { value, .. } => {
                        last_symbol = Some(_type);
                        arithmetic_list.push(value.into())
                    }
                    Symbol::DIV { value, .. } => {
                        last_symbol = Some(_type);
                        arithmetic_list.push(value.into());
                        div_amount += 1;
                    }
                }
            }
        }

        self.arithmetic_string = Some(arithmetic_list.join(""));
        self.spec.captcha.chars = Some(
            evalexpr::eval(
                self.arithmetic_string
                    .clone()
                    .unwrap()
                    .replace("x", "*")
                    .replace("÷", "/")
                    .as_str(),
            )
            .unwrap()
            .to_string(),
        );
        self.arithmetic_string = Some(self.arithmetic_string.clone().unwrap().add("=?"));

        self.spec.captcha.chars.clone().unwrap().chars().collect()
    }

    pub fn get_arithmetic_string(&mut self) -> String {
        if self.arithmetic_string.is_none() {
            self.alphas();
        }

        self.arithmetic_string.clone().unwrap()
    }

    pub fn set_difficulty(&mut self, difficulty: usize) {
        // 做上下界检测，避免越界
        if difficulty <= 0 {
            self.difficulty = 10
        } else {
            self.difficulty = difficulty
        }
    }

    pub fn support_algorithm_sign(&mut self, algorithm_sign: usize) {
        // 做上下界检测，避免越界
        self.algorithm_sign = if algorithm_sign < 2 {
            2
        } else if algorithm_sign > 5 {
            5
        } else {
            algorithm_sign
        }
    }
}

impl NewCaptcha for ArithmeticCaptcha {
    fn new() -> Self {
        let mut spec = SpecCaptcha::new();
        spec.captcha.len = 2;

        let arithmetic_string = None;
        let difficulty = 10;
        let algorithm_sign = 4;

        Self {
            spec,
            arithmetic_string,
            difficulty,
            algorithm_sign,
        }
    }

    fn with_size(width: i32, height: i32) -> Self {
        let mut _self = Self::new();
        _self.spec.captcha.width = width;
        _self.spec.captcha.height = height;
        _self
    }

    fn with_size_and_len(width: i32, height: i32, len: usize) -> Self {
        let mut sf = Self::new();
        sf.spec.captcha.width = width;
        sf.spec.captcha.height = height;
        sf.spec.captcha.len = len;
        sf
    }

    fn with_all(width: i32, height: i32, len: usize, font: &Arc<Font>, font_size: f32) -> Self {
        let mut sf = Self::new();
        sf.spec.captcha.width = width;
        sf.spec.captcha.height = height;
        sf.spec.captcha.len = len;
        sf.spec
            .captcha
            .set_font_by_font(Arc::clone(font), Some(font_size));
        sf
    }
}

impl AbstractCaptcha for ArithmeticCaptcha {
    type Error = png::EncodingError;

    fn out(&mut self, out: impl Write) -> Result<(), Self::Error> {
        if self.arithmetic_string.is_none() {
            self.alphas();
        }

        self.spec.graphics_image(
            &self.arithmetic_string.clone().unwrap().chars().collect(),
            out,
        )
    }

    fn get_chars(&mut self) -> Vec<char> {
        if self.arithmetic_string.is_none() {
            self.alphas();
        }
        self.spec.captcha.chars.clone().unwrap().chars().collect()
    }

    fn base64(&mut self) -> Result<String, Self::Error> {
        self.spec.base64_with_head("data:image/png;base64,")
    }

    fn get_content_type(&mut self) -> String {
        "image/png".into()
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use std::fs::File;
//
//     #[test]
//     fn it_works() {
//         let mut file = File::create("test2.png").unwrap();
//         let mut captcha = ArithmeticCaptcha::new();
//         captcha.out(&mut file);
//         println!("{}", captcha.spec.captcha.chars.unwrap())
//     }
// }
