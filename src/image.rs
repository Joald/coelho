use cairo::{self, Context, ImageSurface};
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub enum CoelhoError {
  CairoError(cairo::Error),
  CairoIoError(cairo::IoError),
  IoError(io::Error),
}

pub fn create_fake_quote(quote: String, output_filename: PathBuf) -> Result<(), CoelhoError> {
  let mut file = File::open("coelho_empty.png").map_err(CoelhoError::IoError)?;
  let surface = ImageSurface::create_from_png(&mut file).map_err(CoelhoError::CairoIoError)?;
  let ctx = Context::new(&surface);
  ctx.translate(500., 300.);
  ctx.set_font_size(50.);
  ctx.set_source_rgb(1., 1., 1.);
  ctx.text_path(&quote);
  ctx.fill();
  ctx.stroke();
  let mut out_file = File::create(output_filename).map_err(CoelhoError::IoError)?;
  surface
    .write_to_png(&mut out_file)
    .map_err(CoelhoError::CairoIoError)?;
  Ok(())
}
