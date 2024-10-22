use image::ImageReader;
use rusty_tesseract::Image;
use rusty_tesseract::Args;
use std::collections::HashMap;

fn main() {
    // or instantiate Image from a DynamicImage
    let dynamic_image = ImageReader::open("docs/pdf2.0.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let img = Image::from_dynamic_image(&dynamic_image).unwrap();
    let default_args = Args::default();

    // the default parameters are
    /*
    Args {
        lang: "eng",
        dpi: Some(150),
        psm: Some(3),
        oem: Some(3),
    }
    */

    // fill your own argument struct if needed
    // Optional arguments are ignored if set to `None`
    let mut my_args = Args {
        //model language (tesseract default = 'eng')
        //available languages can be found by running 'rusty_tesseract::get_tesseract_langs()'
        lang: "eng".to_string(),

        //map of config variables
        //this example shows a whitelist for the normal alphabet. Multiple arguments are allowed.
        //available arguments can be found by running 'rusty_tesseract::get_tesseract_config_parameters()'
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
        )]),
        dpi: Some(150), // specify DPI for input image
        psm: Some(6), // define page segmentation mode 6 (i.e. "Assume a single uniform block of text")
        oem: Some(3), // define optical character recognition mode 3 (i.e. "Default, based on what is available")
    };
    // define parameters
    let mut my_args = Args {
        lang: "eng".to_string(),
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
        )]),
        dpi: Some(150),
        psm: Some(6),
        oem: Some(3),
    };

    // string output
    let output = rusty_tesseract::image_to_string(&img, &my_args).unwrap();
    println!("The String output is: {:?}", output);

    // image_to_boxes creates a BoxOutput containing the parsed output from Tesseract when using the "makebox" Parameter
    let box_output = rusty_tesseract::image_to_boxes(&img, &my_args).unwrap();
    println!(
        "The first boxfile symbol is: {}",
        box_output.boxes[0].symbol
    );
    println!("The full boxfile output is:\n{}", box_output.output);

    // image_to_data creates a DataOutput containing the parsed output from Tesseract when using the "TSV" Parameter
    let data_output = rusty_tesseract::image_to_data(&img, &my_args).unwrap();
    let first_text_line = &data_output.data[4];
    println!(
        "The first text is '{}' with confidence {}",
        first_text_line.text, first_text_line.conf
    );
    println!("The full data output is:\n{}", data_output.output);
}
