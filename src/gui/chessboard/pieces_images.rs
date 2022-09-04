pub(crate) struct PiecesImages {
    pub(crate) wp: egui_extras::RetainedImage,
    pub(crate) wn: egui_extras::RetainedImage,
    pub(crate) wb: egui_extras::RetainedImage,
    pub(crate) wr: egui_extras::RetainedImage,
    pub(crate) wq: egui_extras::RetainedImage,
    pub(crate) wk: egui_extras::RetainedImage,
    pub(crate) bp: egui_extras::RetainedImage,
    pub(crate) bn: egui_extras::RetainedImage,
    pub(crate) bb: egui_extras::RetainedImage,
    pub(crate) br: egui_extras::RetainedImage,
    pub(crate) bq: egui_extras::RetainedImage,
    pub(crate) bk: egui_extras::RetainedImage,
}

impl PiecesImages {
    pub(crate) fn new() -> Self {
        let wp = egui_extras::RetainedImage::from_svg_bytes(
            "wp",
            include_bytes!("./vectors/Chess_plt45.svg"),
        )
        .unwrap();
        let wn = egui_extras::RetainedImage::from_svg_bytes(
            "wn",
            include_bytes!("./vectors/Chess_nlt45.svg"),
        )
        .unwrap();
        let wb = egui_extras::RetainedImage::from_svg_bytes(
            "wb",
            include_bytes!("./vectors/Chess_blt45.svg"),
        )
        .unwrap();
        let wr = egui_extras::RetainedImage::from_svg_bytes(
            "wr",
            include_bytes!("./vectors/Chess_rlt45.svg"),
        )
        .unwrap();
        let wq = egui_extras::RetainedImage::from_svg_bytes(
            "wq",
            include_bytes!("./vectors/Chess_qlt45.svg"),
        )
        .unwrap();
        let wk = egui_extras::RetainedImage::from_svg_bytes(
            "wk",
            include_bytes!("./vectors/Chess_klt45.svg"),
        )
        .unwrap();
        let bp = egui_extras::RetainedImage::from_svg_bytes(
            "bp",
            include_bytes!("./vectors/Chess_pdt45.svg"),
        )
        .unwrap();
        let bn = egui_extras::RetainedImage::from_svg_bytes(
            "bn",
            include_bytes!("./vectors/Chess_ndt45.svg"),
        )
        .unwrap();
        let bb = egui_extras::RetainedImage::from_svg_bytes(
            "bb",
            include_bytes!("./vectors/Chess_bdt45.svg"),
        )
        .unwrap();
        let br = egui_extras::RetainedImage::from_svg_bytes(
            "br",
            include_bytes!("./vectors/Chess_rdt45.svg"),
        )
        .unwrap();
        let bq = egui_extras::RetainedImage::from_svg_bytes(
            "bq",
            include_bytes!("./vectors/Chess_qdt45.svg"),
        )
        .unwrap();
        let bk = egui_extras::RetainedImage::from_svg_bytes(
            "bk",
            include_bytes!("./vectors/Chess_kdt45.svg"),
        )
        .unwrap();

        Self {
            wp,
            wn,
            wb,
            wr,
            wq,
            wk,
            bp,
            bn,
            bb,
            br,
            bq,
            bk,
        }
    }
}
