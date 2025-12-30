use zpl_parser::{
    Parser,
    command::{
        BlockJustification, Code128Mode, Command, CompressionType, DiagonalDirection, Encoding,
        FontName, Justification, LineColor, Orientation, QrErrorCorrection, QrModel,
    },
};

#[test]
fn test_start_format() {
    let parser = Parser::new("^XA");
    let result = parser.parse().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Command::StartFormat);
}

#[test]
fn test_end_format() {
    let parser = Parser::new("^XZ");
    let result = parser.parse().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Command::EndFormat);
}

#[test]
fn test_field_separator() {
    let parser = Parser::new("^FS");
    let result = parser.parse().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], Command::FieldSeperator);
}

#[test]
fn test_field_data() {
    let parser = Parser::new("^FDHello World^FS");
    let result = parser.parse().unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(
        result[0],
        Command::FielData {
            data: "Hello World".to_string()
        }
    );
}

#[test]
fn test_field_origin_minimal() {
    let parser = Parser::new("^FO100,200");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::FieldOrigin {
            x: 100,
            y: 200,
            justification: None
        }
    );
}

#[test]
fn test_field_origin_with_justification() {
    let parser = Parser::new("^FO100,200,1");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::FieldOrigin {
            x: 100,
            y: 200,
            justification: Some(Justification::Right)
        }
    );
}

#[test]
fn test_label_home() {
    let parser = Parser::new("^LH50,100");
    let result = parser.parse().unwrap();
    assert_eq!(result[0], Command::LabelHome { x: 50, y: 100 });
}

#[test]
fn test_label_top() {
    let parser = Parser::new("^LT10");
    let result = parser.parse().unwrap();
    assert_eq!(result[0], Command::LabelTop { offset: 10 });
}

#[test]
fn test_label_top_negative() {
    let parser = Parser::new("^LT-5");
    let result = parser.parse().unwrap();
    assert_eq!(result[0], Command::LabelTop { offset: -5 });
}

#[test]
fn test_print_width() {
    let parser = Parser::new("^PW800");
    let result = parser.parse().unwrap();
    assert_eq!(result[0], Command::PrintWidth { width: 800 });
}

#[test]
fn test_label_length_minimal() {
    let parser = Parser::new("^LL600");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::LabelLength {
            length: 600,
            apply_to_all_media: None
        }
    );
}

#[test]
fn test_label_length_with_flag() {
    let parser = Parser::new("^LL600,Y");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::LabelLength {
            length: 600,
            apply_to_all_media: Some(true)
        }
    );
}

#[test]
fn test_font_minimal() {
    let parser = Parser::new("^A0");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::Font {
            font: FontName::Font0,
            orientation: None,
            height: None,
            width: None
        }
    );
}

#[test]
fn test_font_with_all_params() {
    let parser = Parser::new("^A0N,50,40");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::Font {
            font: FontName::Font0,
            orientation: Some(Orientation::Normal),
            height: Some(50),
            width: Some(40)
        }
    );
}

#[test]
fn test_change_font() {
    let parser = Parser::new("^CFA,30");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::ChangeFont {
            font: FontName::FontA,
            height: Some(30),
            width: None
        }
    );
}

#[test]
fn test_field_typeset() {
    let parser = Parser::new("^FT100,200");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::FieldTypeset {
            x: 100,
            y: 200,
            justification: None
        }
    );
}

#[test]
fn test_field_block() {
    let parser = Parser::new("^FB300,5,0,L,0");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::FieldBlock {
            width: 300,
            max_lines: 5,
            line_spacing: 0,
            justification: BlockJustification::Left,
            hanging_indent: 0
        }
    );
}

#[test]
fn test_graphic_box_minimal() {
    let parser = Parser::new("^GB100,50,5");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::GraphicBox {
            width: 100,
            height: 50,
            thickness: 5,
            line_color: LineColor::Black,
            rounding: 0
        }
    );
}

#[test]
fn test_graphic_box_full() {
    let parser = Parser::new("^GB100,50,5,W,10");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::GraphicBox {
            width: 100,
            height: 50,
            thickness: 5,
            line_color: LineColor::White,
            rounding: 10
        }
    );
}

#[test]
fn test_graphic_field_ascii() {
    let parser = Parser::new("^GFA,100,100,10,48656C6C6F");
    let result = parser.parse().unwrap();

    if let Command::GraphicField {
        compression,
        binary_byte_count,
        graphic_field_count,
        bytes_per_row,
        data,
    } = &result[0]
    {
        assert_eq!(compression, &CompressionType::Ascii);
        assert_eq!(binary_byte_count, &100);
        assert_eq!(graphic_field_count, &100);
        assert_eq!(bytes_per_row, &10);
        assert_eq!(data, &vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]);
    } else {
        panic!("Expected GraphicField command");
    }
}

#[test]
fn test_graphic_circle_minimal() {
    let parser = Parser::new("^GC50");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::GraphicCircle {
            diameter: 50,
            thickness: 1,
            color: LineColor::Black
        }
    );
}

#[test]
fn test_graphic_circle_full() {
    let parser = Parser::new("^GC100,5,W");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::GraphicCircle {
            diameter: 100,
            thickness: 5,
            color: LineColor::White
        }
    );
}

#[test]
fn test_graphic_diagonal_line_minimal() {
    let parser = Parser::new("^GD100,50,2");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::GraphicDiagonalLine {
            width: 100,
            height: 50,
            thickness: 2,
            color: LineColor::Black,
            direction: DiagonalDirection::RightToLeft
        }
    );
}

#[test]
fn test_graphic_diagonal_line_full() {
    let parser = Parser::new("^GD100,50,2,W,R");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::GraphicDiagonalLine {
            width: 100,
            height: 50,
            thickness: 2,
            color: LineColor::White,
            direction: DiagonalDirection::LeftToRight
        }
    );
}

#[test]
fn test_code128_minimal() {
    let parser = Parser::new("^BCN");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::Code128 {
            orientation: Some(Orientation::Normal),
            height: None,
            print_interpetation_line: true,
            print_above: false,
            check_digit: false,
            mode: Code128Mode::AutoMode
        }
    );
}

#[test]
fn test_code128_full() {
    let parser = Parser::new("^BCR,100,Y,Y,N,A");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::Code128 {
            orientation: Some(Orientation::Rotated90),
            height: Some(100),
            print_interpetation_line: true,
            print_above: true,
            check_digit: false,
            mode: Code128Mode::Auto
        }
    );
}

#[test]
fn test_barcode_field_defaults() {
    let parser = Parser::new("^BY2,3.0,10");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::BarcodeFieldDefaults {
            module_width: 2,
            wide_bar_ratio: 3.0,
            height: 10
        }
    );
}

#[test]
fn test_qr_code_minimal() {
    let parser = Parser::new("^BQN");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::QrCode {
            orientation: Orientation::Normal,
            model: QrModel::Model2,
            magnification: 1,
            error_correction: QrErrorCorrection::HighReliability,
            mask: Some(7)
        }
    );
}

#[test]
fn test_qr_code_full() {
    let parser = Parser::new("^BQR,1,5,H,3");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::QrCode {
            orientation: Orientation::Rotated90,
            model: QrModel::Model1,
            magnification: 5,
            error_correction: QrErrorCorrection::UltraHighReliability,
            mask: Some(3)
        }
    );
}

#[test]
fn test_code39_minimal() {
    let parser = Parser::new("^B3,N");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::Code39 {
            orientation: Orientation::Normal,
            height: None,
            print_interpetation_line: true,
            print_above: false,
            check_digit: false
        }
    );
}

#[test]
fn test_code39_full() {
    let parser = Parser::new("^B3R,Y,100,N,Y");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::Code39 {
            orientation: Orientation::Rotated90,
            height: Some(100),
            print_interpetation_line: false,
            print_above: true,
            check_digit: true
        }
    );
}

#[test]
fn test_character_encoding_default() {
    let parser = Parser::new("^CI");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::CharacterEncoding {
            encoding: Encoding::Usa1
        }
    );
}

#[test]
fn test_character_encoding_utf8() {
    let parser = Parser::new("^CI28");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::CharacterEncoding {
            encoding: Encoding::Utf8
        }
    );
}

#[test]
fn test_print_quantity_minimal() {
    let parser = Parser::new("^PQ10");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::PrintQuality {
            total: 10,
            pause_count: None,
            replicate_count: None,
            override_pause: false
        }
    );
}

#[test]
fn test_print_quantity_full() {
    let parser = Parser::new("^PQ50,10,2,Y");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::PrintQuality {
            total: 50,
            pause_count: Some(10),
            replicate_count: Some(2),
            override_pause: true
        }
    );
}

#[test]
fn test_media_darkness() {
    let parser = Parser::new("^MD-5");
    let result = parser.parse().unwrap();
    assert_eq!(result[0], Command::MediaDarkness { value: -5 });
}

#[test]
fn test_comment() {
    let parser = Parser::new("^FXThis is a comment^FS");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::Comment {
            text: "This is a comment".to_string()
        }
    );
}

#[test]
fn test_comment_empty() {
    let parser = Parser::new("^FX^FS");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::Comment {
            text: String::new()
        }
    );
}

#[test]
fn test_complete_label() {
    let zpl = r#"^XA^FO50,50^A0N,30,30^FDHello World^FS^FO50,100^BCN,100,Y,N,N^FD123456^FS^XZ"#;
    let parser = Parser::new(zpl);
    let result = parser.parse().unwrap();

    assert_eq!(result[0], Command::StartFormat);
    assert!(matches!(result[1], Command::FieldOrigin { .. }));
    assert!(matches!(result[2], Command::Font { .. }));
    assert!(matches!(result[3], Command::FielData { .. }));
    assert_eq!(result[4], Command::FieldSeperator);
    assert!(matches!(result[5], Command::FieldOrigin { .. }));
    assert!(matches!(result[6], Command::Code128 { .. }));
    assert!(matches!(result[7], Command::FielData { .. }));
    assert_eq!(result[8], Command::FieldSeperator);
    assert_eq!(result[9], Command::EndFormat);
}

#[test]
fn test_orientation_values() {
    let parser = Parser::new("^A0N^A0R^A0I^A0B");
    let result = parser.parse().unwrap();

    assert_eq!(
        result[0],
        Command::Font {
            font: FontName::Font0,
            orientation: Some(Orientation::Normal),
            height: None,
            width: None
        }
    );

    assert_eq!(
        result[1],
        Command::Font {
            font: FontName::Font0,
            orientation: Some(Orientation::Rotated90),
            height: None,
            width: None
        }
    );

    assert_eq!(
        result[2],
        Command::Font {
            font: FontName::Font0,
            orientation: Some(Orientation::Rotated180),
            height: None,
            width: None
        }
    );

    assert_eq!(
        result[3],
        Command::Font {
            font: FontName::Font0,
            orientation: Some(Orientation::Rotated270),
            height: None,
            width: None
        }
    );
}

#[test]
fn test_justification_values() {
    let parser = Parser::new("^FO0,0,0^FO0,0,1^FO0,0,2");
    let result = parser.parse().unwrap();

    assert_eq!(
        result[0],
        Command::FieldOrigin {
            x: 0,
            y: 0,
            justification: Some(Justification::Left)
        }
    );

    assert_eq!(
        result[1],
        Command::FieldOrigin {
            x: 0,
            y: 0,
            justification: Some(Justification::Right)
        }
    );

    assert_eq!(
        result[2],
        Command::FieldOrigin {
            x: 0,
            y: 0,
            justification: Some(Justification::Auto)
        }
    );
}

#[test]
fn test_block_justification_values() {
    let parser = Parser::new("^FB100,1,0,L^FB100,1,0,C^FB100,1,0,R^FB100,1,0,J");
    let result = parser.parse().unwrap();

    if let Command::FieldBlock { justification, .. } = result[0] {
        assert_eq!(justification, BlockJustification::Left);
    }

    if let Command::FieldBlock { justification, .. } = result[1] {
        assert_eq!(justification, BlockJustification::Center);
    }

    if let Command::FieldBlock { justification, .. } = result[2] {
        assert_eq!(justification, BlockJustification::Right);
    }

    if let Command::FieldBlock { justification, .. } = result[3] {
        assert_eq!(justification, BlockJustification::Justified);
    }
}

#[test]
fn test_line_color_values() {
    let parser = Parser::new("^GB100,50,1,B^GB100,50,1,W");
    let result = parser.parse().unwrap();

    if let Command::GraphicBox { line_color, .. } = result[0] {
        assert_eq!(line_color, LineColor::Black);
    }

    if let Command::GraphicBox { line_color, .. } = result[1] {
        assert_eq!(line_color, LineColor::White);
    }
}

#[test]
fn test_compression_types() {
    let parser = Parser::new("^GFA,10,10,1,FF^GFB,10,10,1,FF^GFC,10,10,1,FF");
    let result = parser.parse().unwrap();

    if let Command::GraphicField { compression, .. } = result[0] {
        assert_eq!(compression, CompressionType::Ascii);
    }

    if let Command::GraphicField { compression, .. } = result[1] {
        assert_eq!(compression, CompressionType::Binary);
    }

    if let Command::GraphicField { compression, .. } = result[2] {
        assert_eq!(compression, CompressionType::CompressedBinary);
    }
}

#[test]
fn test_font_names() {
    let fonts = vec![
        ("^A0", FontName::Font0),
        ("^AA", FontName::FontA),
        ("^AB", FontName::FontB),
        ("^AZ", FontName::Custom('Z')),
    ];

    for (zpl, expected_font) in fonts {
        let parser = Parser::new(zpl);
        let result = parser.parse().unwrap();

        if let Command::Font { font, .. } = result[0] {
            assert_eq!(font, expected_font);
        } else {
            panic!("Expected Font command");
        }
    }
}

#[test]
fn test_encoding_values() {
    let encodings = vec![
        ("^CI0", Encoding::Usa1),
        ("^CI1", Encoding::Usa2),
        ("^CI13", Encoding::Zebra),
        ("^CI28", Encoding::Utf8),
    ];

    for (zpl, expected_encoding) in encodings {
        let parser = Parser::new(zpl);
        let result = parser.parse().unwrap();

        if let Command::CharacterEncoding { encoding } = result[0] {
            assert_eq!(encoding, expected_encoding);
        } else {
            panic!("Expected CharacterEncoding command");
        }
    }
}

#[test]
fn test_qr_error_correction_values() {
    let parser = Parser::new("^BQN,2,1,H^BQN,2,1,Q^BQN,2,1,M^BQN,2,1,L");
    let result = parser.parse().unwrap();

    if let Command::QrCode {
        error_correction, ..
    } = result[0]
    {
        assert_eq!(error_correction, QrErrorCorrection::UltraHighReliability);
    }

    if let Command::QrCode {
        error_correction, ..
    } = result[1]
    {
        assert_eq!(error_correction, QrErrorCorrection::HighReliability);
    }

    if let Command::QrCode {
        error_correction, ..
    } = result[2]
    {
        assert_eq!(error_correction, QrErrorCorrection::Standard);
    }

    if let Command::QrCode {
        error_correction, ..
    } = result[3]
    {
        assert_eq!(error_correction, QrErrorCorrection::HighDensity);
    }
}

#[test]
fn test_diagonal_direction_values() {
    let parser = Parser::new("^GD100,50,2,B,R^GD100,50,2,B,L");
    let result = parser.parse().unwrap();

    if let Command::GraphicDiagonalLine { direction, .. } = result[0] {
        assert_eq!(direction, DiagonalDirection::LeftToRight);
    }

    if let Command::GraphicDiagonalLine { direction, .. } = result[1] {
        assert_eq!(direction, DiagonalDirection::RightToLeft);
    }
}

#[test]
fn test_empty_parameters_use_defaults() {
    let parser = Parser::new("^FO0,0");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::FieldOrigin {
            x: 0,
            y: 0,
            justification: None
        }
    );
}

#[test]
fn test_mixed_empty_parameters() {
    let parser = Parser::new("^A0,,50");
    let result = parser.parse().unwrap();
    assert_eq!(
        result[0],
        Command::Font {
            font: FontName::Font0,
            orientation: None,
            height: None,
            width: Some(50)
        }
    );
}
