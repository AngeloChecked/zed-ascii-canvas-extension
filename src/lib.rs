use zed_extension_api as zed;

struct ZedAsciiCanvas;

impl zed::Extension for ZedAsciiCanvas {
    fn new() -> Self {
        Self {}
    }
}

zed::register_extension!(ZedAsciiCanvas);
