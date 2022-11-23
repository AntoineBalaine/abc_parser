mod csv_parse;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let parsed = csv_parse::parse_csv("-20, 12.5\n42, 0")?;
    let mut sum = 0.;
    for record in parsed {
        for field in record {
            if let csv_parse::CSVField::Number(x) = field {
                sum += x;
            }
        }
    }
    assert_eq!(sum, 34.5);

    let unsuccessful_parse = csv_parse::parse_csv("4, 0.1.1");
    println!("failure: {}", unsuccessful_parse.unwrap_err());

    let successful_parse = csv_parse::parse_csv("-273.15 , ' a string '\n\n42, 0")?;
    println!("success: {:?}", successful_parse);

    Ok(())
}
