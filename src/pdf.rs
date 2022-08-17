use genpdf::Alignment;
use genpdf::Element as _;
use genpdf::{elements, fonts, style};
use image::Luma;
use qrcode::QrCode;
use std::env;
use std::fs::remove_file;
use std::path::Path;

const FONT_DIRS: &[&str] = &["fonts"];
const DEFAULT_FONT_NAME: &'static str = "LiberationSans";

pub fn generate_pdf(preimage: &str) {
    // First we need to know if the file already exists we don't do anything
    let pdf_path_string = format!("./files/{preimage}.pdf");
    let pdf_path: &str = &pdf_path_string;
    let pdf_path = Path::new(pdf_path);
    if pdf_path.exists() {
        return ();
    }
    let title = env::var("EVENT_NAME").expect("EVENT_NAME must be set");
    let description = env::var("EVENT_DESCRIPTION").expect("EVENT_DESCRIPTION must be set");
    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
    let address = env::var("EVENT_ADDRESS").expect("EVENT_ADDRESS must be set");
    let datetime_str = env::var("EVENT_DATETIME").expect("EVENT_DATETIME must be set");
    let note = env::var("PDF_NOTE").expect("PDF_NOTE must be set");
    let font_dir = FONT_DIRS
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");
    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Helvetica))
            .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);
    doc.set_title(title.clone());
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    decorator.set_header(|_| {
        let layout = elements::LinearLayout::vertical();
        layout.styled(style::Style::new().with_font_size(10))
    });
    doc.set_page_decorator(decorator);

    doc.push(
        elements::Paragraph::new(&title).styled(style::Style::new().bold().with_font_size(20)),
    );
    doc.push(elements::Break::new(1.5));
    doc.push(elements::Paragraph::new(&description).styled(style::Style::new().with_font_size(10)));

    doc.push(
        elements::Paragraph::new("Present this ticket on the event entrance")
            .styled(style::Style::new().with_font_size(10)),
    );
    // We create the URL to be QRencoded
    let url = format!("{server_url}/verify/{preimage}");

    // Encode some data into bits.
    let code = QrCode::new(url.as_bytes()).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    let image_path = format!("/tmp/{preimage}.png");
    let image_path: &str = &image_path;
    let image_path = Path::new(image_path);
    // Save the image.
    image.save(image_path).unwrap();

    doc.push(
        elements::Image::from_path(image_path)
            .expect("Unable to load alt image")
            .with_scale(genpdf::Scale::new(2, 2))
            .with_position(genpdf::Position::new(69, 12)), // far over to right and down
    );
    doc.push(elements::Break::new(13));
    doc.push(
        elements::Paragraph::new(&*preimage)
            .aligned(Alignment::Center)
            .styled(style::Style::new().with_font_size(6)),
    );
    doc.push(elements::Break::new(2));
    doc.push(elements::Paragraph::new(&address).aligned(Alignment::Center));
    doc.push(elements::Paragraph::new(&datetime_str).aligned(Alignment::Center));
    doc.push(elements::Break::new(5));

    doc.push(elements::Paragraph::new(&note).styled(style::Style::new().with_font_size(10)));

    doc.render_to_file(&pdf_path)
        .expect("Failed to write output file");
    // Now we delete the image
    if image_path.exists() {
        remove_file(image_path).unwrap();
    }
}
