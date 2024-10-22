fn main() {
  let bytes = std::fs::read("docs/pdf2.0.pdf").unwrap();
  let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();

  // write output to txt file
  std::fs::write("output.txt", out.as_bytes()).unwrap();
  println!("{}", out);
}
