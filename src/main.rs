fn main() {
    let local_true_x = Local::new(10, "x");
    let local_true_y = Local::new(11, "y");
    let local_false_x = Local::new(20, "x");
    let local_false_y = Local::new(21, "y");
    let local_or_p = Local::new(30, "p");
    let local_or_q = Local::new(31, "q");

    let decl_true = Decl::new(
        "T",
        Abstr::new(
            local_true_x.clone(),
            Abstr::new(local_true_y.clone(), local_true_x.clone()),
        ),
    );
    print_declaration(&decl_true);

    let decl_false = Decl::new(
        "F",
        Abstr::new(
            local_false_x.clone(),
            Abstr::new(local_false_y.clone(), local_false_y.clone()),
        ),
    );
    print_declaration(&decl_false);

    let decl_or = Decl::new(
        "|",
        Abstr::new(
            local_or_p.clone(),
            Abstr::new(
                local_or_q.clone(),
                Appl::new(
                    Appl::new(local_or_p.clone(), local_or_q.clone()),
                    local_or_p.clone(),
                ),
            ),
        ),
    );
    print_declaration(&decl_or);

    let query = Term::new(Appl::new(Appl::new(decl_or, decl_true), decl_false));
    println!("Query:");
    print_term(&query, 0, "");

    let result = evaluate_query(query);
    println!("Result:");
    print_term(&result, 0, "");
}

fn evaluate_query(query: Term) -> Term {
    unimplemented!();
}

struct Term {
    value: TermKind,
}

enum TermKind {
    Local(Local),
    Global(Decl),
    Abstraction(Abstr),
    Application(Appl),
}

#[derive(Clone)]
struct Local {
    id: usize,
    name: String,
}

struct Decl {
    name: String,
    term: Box<Term>,
}

struct Abstr {
    parameter: Local,
    body: Box<Term>,
}

struct Appl {
    function: Box<Term>,
    argument: Box<Term>,
}

impl Into<TermKind> for Local {
    fn into(self) -> TermKind {
        TermKind::Local(self)
    }
}
impl Into<TermKind> for Decl {
    fn into(self) -> TermKind {
        TermKind::Global(self)
    }
}
impl Into<TermKind> for Abstr {
    fn into(self) -> TermKind {
        TermKind::Abstraction(self)
    }
}
impl Into<TermKind> for Appl {
    fn into(self) -> TermKind {
        TermKind::Application(self)
    }
}

impl Term {
    pub fn new(value: impl Into<TermKind>) -> Self {
        Self {
            value: value.into(),
        }
    }
}
impl Local {
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
        }
    }
}
impl Decl {
    pub fn new(name: impl Into<String>, term: impl Into<TermKind>) -> Self {
        Self {
            name: name.into(),
            term: Box::new(Term::new(term.into())),
        }
    }
}
impl Abstr {
    pub fn new(parameter: Local, body: impl Into<TermKind>) -> Self {
        Self {
            parameter,
            body: Box::new(Term::new(body)),
        }
    }
}
impl Appl {
    pub fn new(function: impl Into<TermKind>, argument: impl Into<TermKind>) -> Self {
        Self {
            function: Box::new(Term::new(function)),
            argument: Box::new(Term::new(argument)),
        }
    }
}

fn print_declaration(decl: &Decl) {
    println!("[_] {}", decl.name);
    print_term(&decl.term, 0, "");
}

fn print_term(term: &Term, depth: usize, prefix: &str) {
    match &term.value {
        TermKind::Local(local) => {
            print_label(depth, prefix, "local");
            print_local(local);
        }
        TermKind::Global(global) => {
            print_label(depth, prefix, "global");
            println!("[_] `{}` ...", global.name);
        }
        TermKind::Abstraction(abstr) => {
            print_label(depth, prefix, "abstraction");
            println!();
            print_label(depth + 1, "parameter", "");
            print_local(&abstr.parameter);
            print_term(&abstr.body, depth + 1, "body");
        }
        TermKind::Application(appl) => {
            print_label(depth, prefix, "application");
            println!();
            print_term(&appl.function, depth + 1, "function");
            print_term(&appl.argument, depth + 1, "argument");
        }
    }
}

fn print_local(local: &Local) {
    println!("{{{}}} `{}`", local.id, local.name);
}

fn print_label(depth: usize, prefix: &str, label: &str) {
    for _ in 0..depth {
        print!("|     ");
    }
    if !prefix.is_empty() {
        print!("{}", prefix);
    }
    if !prefix.is_empty() && !label.is_empty() {
        print!(".");
    }
    print!("{}: ", label);
}
