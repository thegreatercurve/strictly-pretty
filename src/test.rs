#[cfg(test)]
mod tests {
    use crate::{
        break_with, break_with_space, doc_vec_to_doc, group, nest, text, Doc, PrettyPrinter,
    };

    #[test]
    fn it_should_correctly_break_based_on_max_width() {
        let doc = group(doc_vec_to_doc(vec![
            text("[begin"),
            nest(
                3,
                doc_vec_to_doc(vec![
                    break_with_space(),
                    group(doc_vec_to_doc(vec![
                        text("[stmt;"),
                        break_with_space(),
                        text("stmt;"),
                        break_with_space(),
                        text("stmt;]"),
                    ])),
                ]),
            ),
            break_with_space(),
            text("end]"),
        ]));

        let printer = PrettyPrinter();

        assert_eq!(
            printer.format(doc.clone(), 50),
            "[begin [stmt; stmt; stmt;] end]"
        );
        assert_eq!(
            printer.format(doc.clone(), 30),
            "[begin
   [stmt; stmt; stmt;]
end]"
        );
        assert_eq!(
            printer.format(doc.clone(), 10),
            "[begin
   [stmt;
   stmt;
   stmt;]
end]"
        );
    }

    #[test]
    fn it_should_correctly_break_based_with_nested_groups() {
        let doc = group(doc_vec_to_doc(vec![
            text("[begin"),
            nest(
                3,
                doc_vec_to_doc(vec![
                    break_with_space(),
                    group(doc_vec_to_doc(vec![
                        text("[stmt;"),
                        break_with_space(),
                        text("stmt;"),
                        nest(
                            3,
                            doc_vec_to_doc(vec![
                                break_with_space(),
                                group(doc_vec_to_doc(vec![
                                    text("[stmt;"),
                                    break_with_space(),
                                    text("stmt;"),
                                    break_with_space(),
                                    text("stmt;]"),
                                ])),
                            ]),
                        ),
                        break_with_space(),
                        text("stmt;]"),
                    ])),
                ]),
            ),
            break_with_space(),
            text("end]"),
        ]));

        let printer = PrettyPrinter();

        assert_eq!(
            printer.format(doc.clone(), 80),
            "[begin [stmt; stmt; [stmt; stmt; stmt;] stmt;] end]"
        );
        assert_eq!(
            printer.format(doc.clone(), 50),
            "[begin
   [stmt; stmt; [stmt; stmt; stmt;] stmt;]
end]"
        );
        assert_eq!(
            printer.format(doc.clone(), 30),
            "[begin
   [stmt;
   stmt;
      [stmt; stmt; stmt;]
   stmt;]
end]"
        );
        assert_eq!(
            printer.format(doc.clone(), 10),
            "[begin
   [stmt;
   stmt;
      [stmt;
      stmt;
      stmt;]
   stmt;]
end]"
        );
    }

    #[test]
    fn it_should_correctly_break_based_with_if_else_statement() {
        fn binop(left: &str, op: &str, right: &str) -> Doc {
            group(doc_vec_to_doc(vec![nest(
                2,
                doc_vec_to_doc(vec![
                    group(doc_vec_to_doc(vec![
                        text(left),
                        break_with_space(),
                        break_with(op),
                    ])),
                    break_with_space(),
                    text(right),
                ]),
            )]))
        }

        let cond = binop("a", "==", "b");
        let expr_1 = binop("a", "<<", "b");
        let expr_2 = binop("a", "+", "b");

        fn if_then(cond: Doc, expr_1: Doc, expr_2: Doc) -> Doc {
            group(doc_vec_to_doc(vec![
                group(doc_vec_to_doc(vec![nest(
                    2,
                    doc_vec_to_doc(vec![text("if"), break_with_space(), cond]),
                )])),
                break_with_space(),
                group(doc_vec_to_doc(vec![nest(
                    2,
                    doc_vec_to_doc(vec![text("then"), break_with_space(), expr_1]),
                )])),
                break_with_space(),
                group(doc_vec_to_doc(vec![nest(
                    2,
                    doc_vec_to_doc(vec![text("else"), break_with_space(), expr_2]),
                )])),
            ]))
        }

        let doc = if_then(cond, expr_1, expr_2);

        let printer = PrettyPrinter();

        assert_eq!(
            printer.format(doc.clone(), 32),
            "if a == b then a << b else a + b"
        );
        assert_eq!(
            printer.format(doc.clone(), 15),
            "if a == b
then a << b
else a + b"
        );
        assert_eq!(
            printer.format(doc.clone(), 10),
            "if a == b
then
  a << b
else a + b"
        );
        assert_eq!(
            printer.format(doc.clone(), 8),
            "if
  a == b
then
  a << b
else
  a + b"
        );
        assert_eq!(
            printer.format(doc.clone(), 7),
            "if
  a ==
    b
then
  a <<
    b
else
  a + b"
        );
        assert_eq!(
            printer.format(doc.clone(), 6),
            "if
  a ==
    b
then
  a <<
    b
else
  a +
    b"
        );
    }
}
