use uml_test::diagram::*;

#[test]
fn convert_to_and_from_json() -> serde_json::Result<()> {
    let diagram = Diagram::from(vec![
        Element::Rectangle { x: 50, y: 50 },
        Element::Circle {
            color: Color::Red,
            size: 20.5,
        },
        Element::Text {
            text: String::from("MyClassTitle"),
        },
    ]);

    let serialized: String = serde_json::to_string(&diagram)?;
    let deserialized: Diagram = serde_json::from_str(&serialized)?;
    assert_eq!(deserialized, diagram);
    Ok(())
}
