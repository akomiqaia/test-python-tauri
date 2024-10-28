fn main() {
    let pdf = std::fs::read("docs/pdf2.0.pdf");
    let bytes = pdf.unwrap();
    println!("{:?}", bytes);
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();

    // write output to txt file
    let _ = std::fs::write("output.txt", &out);
}
