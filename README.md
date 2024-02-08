# easy-captcha✌️

Rust图形验证码，由Java同名开源库[whvcse/EasyCaptcha](https://github.com/ele-admin/EasyCaptcha)移植而来👏，100%纯Rust实现，支持gif、算术等类型。

目前已适配框架：

- `axum` + `tower-sessions`

更多框架欢迎您提交PR，参与适配🙏

## 效果展示

**普通验证码**

![](https://s.c.accr.cc/picgo/1707038120-1f01ca.png)
&emsp;&emsp;
![](https://s.c.accr.cc/picgo/1707038197-f6cc2f.png)
&emsp;&emsp;
![](https://s.c.accr.cc/picgo/1707038216-0f456d.png)
<br/>

**动态验证码**

![](https://s.c.accr.cc/picgo/1707038251-e5c2ea.gif)
&emsp;&emsp;
![](https://s.c.accr.cc/picgo/1707038285-db5430.gif)
&emsp;&emsp;
![](https://s.c.accr.cc/picgo/1707038298-7742a9.gif)
<br/>

**算术验证码**

![](https://s.c.accr.cc/picgo/1707038412-6e1f68.png)
&emsp;&emsp;
![](https://s.c.accr.cc/picgo/1707038393-c03afc.png)
&emsp;&emsp;
![](https://s.c.accr.cc/picgo/1707038483-387a8a.png)
<br/>

## 安装和使用

**在Linux系统上需要安装`pkg-config`和`fontconfig`**，关于`fontconfig`的选择请参考[fontconfig-rs](https://github.com/yeslogic/fontconfig-rs)
中的提示；具体依赖对应如下：

* Alpine Linux: `pkg-config fontconfig-dev`
* Arch Linux: `pkg-config fontconfig`
* Debian-based systems: `pkg-config libfontconfig1-dev`
* FreeBSD: `pkg-config fontconfig`
* Void Linux: `pkg-config fontconfig-devel`

```shell
cargo add easy-captcha
```

若您正在使用的框架已适配，您可直接通过`CaptchaUtil`类（并导入相应框架的trait）来使用验证码：

```rust
use easy_captcha::extension::axum_tower_sessions::{
    CaptchaAxumTowerSessionExt, CaptchaAxumTowerSessionStaticExt,
};

/// 接口：获取验证码
async fn get_captcha(session: Session) -> Result<Response, StatusCode> {
    let mut captcha: CaptchaUtil<GifCaptcha> = CaptchaUtil::new();
    match captcha.out(&session).await {
        Ok(response) => Ok(response),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// 接口：验证验证码
async fn verify_captcha(
    session: Session,
    Query(query): Query<HashMap<String, String>>,
) -> Response {
    // 从请求中获取验证码
    if let Some(code) = query.get("code") {
        // 调用CaptchaUtil的静态方法验证验证码是否正确
        if CaptchaUtil::ver(code, &session).await {
            CaptchaUtil::clear(&session).await; // 如果愿意的话，你可以从Session中清理掉验证码
            "Your code is valid, thank you.".into_response()
        } else {
            "Your code is not valid, I'm sorry.".into_response()
        }
    } else {
        "You didn't provide the code.".into_response()
    }
}
```

您也可以自定义验证码的各项属性：

```rust
async fn get_captcha(session: Session) -> Result<Response, StatusCode> {
    let mut captcha: CaptchaUtil<GifCaptcha> = CaptchaUtil::with_size_and_len(127, 48, 4);
    match captcha.out(&session).await {
        Ok(response) => Ok(response),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
```

项目当前提供了三种验证码实现：`SpecCaptcha`（静态PNG）、`GifCaptcha`（动态GIF）、`ArithmeticCaptcha`（算术PNG），您可按需使用。

项目内置字体：

| 字体                  | 效果                                             |
|---------------------|------------------------------------------------|
| CaptchaFont::Font1  | ![](https://s2.ax1x.com/2019/08/23/msMe6U.png) |
| CaptchaFont::Font2  | ![](https://s2.ax1x.com/2019/08/23/msMAf0.png) |
| CaptchaFont::Font3  | ![](https://s2.ax1x.com/2019/08/23/msMCwj.png) |
| CaptchaFont::Font4  | ![](https://s2.ax1x.com/2019/08/23/msM9mQ.png) |
| CaptchaFont::Font5  | ![](https://s2.ax1x.com/2019/08/23/msKz6S.png) |
| CaptchaFont::Font6  | ![](https://s2.ax1x.com/2019/08/23/msKxl8.png) |
| CaptchaFont::Font7  | ![](https://s2.ax1x.com/2019/08/23/msMPTs.png) |
| CaptchaFont::Font8  | ![](https://s2.ax1x.com/2019/08/23/msMmXF.png) |
| CaptchaFont::Font9  | ![](https://s2.ax1x.com/2019/08/23/msMVpV.png) |
| CaptchaFont::Font10 | ![](https://s2.ax1x.com/2019/08/23/msMZlT.png) |

## 未来工作计划

- 改进API设计，补充一些setter
- 移植原库的中文验证码功能
- 适配更多框架
- 编写单元测试和集成测试
