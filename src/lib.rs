use chumsky::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum Expr {
    Var(String),
    App(Box<Expr>, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
}

fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let ident = text::ident().padded();

    recursive(|expr| {
        // let <ident> = <expr> in <expr>
        let let_in = text::keyword("let")
            .padded()
            .ignore_then(ident)
            .then_ignore(just('='))
            .then(expr.clone())
            .then_ignore(text::keyword("in").padded())
            .then(expr.clone())
            .map(|((name, let_body), in_body)| {
                Expr::Let(name, Box::new(let_body), Box::new(in_body))
            });
        let var = ident.map(Expr::Var);
        let atom = let_in.or(var);
        // <expr> <expr>
        atom.clone()
            .then(atom.repeated())
            .foldl(|e1, e2| Expr::App(e1.into(), e2.into()))
    })
}

#[test]
fn parses_let_in() {
    assert_eq!(
        parser().parse("let hello = world in hello world"),
        Ok(Expr::Let(
            "hello".into(),
            Box::new(Expr::Var("world".into())),
            Box::new(Expr::App(
                Box::new(Expr::Var("hello".into())),
                Box::new(Expr::Var("world".into()))
            ))
        ))
    );
}

#[test]
fn parses_application() {
    let app = Ok(Expr::App(
        Box::new(Expr::App(
            Box::new(Expr::Var("hello".into())),
            Box::new(Expr::Var("beautiful".into())),
        )),
        Box::new(Expr::Var("world".into())),
    ));

    assert_eq!(parser().parse("hello beautiful world"), app);
}
