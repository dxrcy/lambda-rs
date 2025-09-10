fn main() {
    let decl_true = {
        let local_x = Local::new(10, "x");
        let local_y = Local::new(11, "y");
        Decl::new(
            "T",
            Abstr::new(
                local_x.clone(),
                Abstr::new(local_y.clone(), local_x.clone()),
            ),
        )
    };
    decl_true.print(0, "");

    let decl_false = {
        let local_x = Local::new(20, "x");
        let local_y = Local::new(21, "y");
        Decl::new(
            "F",
            Abstr::new(
                local_x.clone(),
                Abstr::new(local_y.clone(), local_y.clone()),
            ),
        )
    };
    decl_false.print(0, "");

    let decl_or = {
        let local_p = Local::new(30, "p");
        let local_q = Local::new(31, "q");
        Decl::new(
            "|",
            Abstr::new(
                local_p.clone(),
                Abstr::new(
                    local_q.clone(),
                    Appl::new(Appl::new(local_p.clone(), local_p.clone()), local_q.clone()),
                ),
            ),
        )
    };
    decl_or.print(0, "");

    let query = FreeTerm::from(Appl::new(
        Appl::new(
            GlobalRef::new(decl_or.clone()),
            GlobalRef::new(decl_true.clone()),
        ),
        GlobalRef::new(decl_false.clone()),
    ));
    println!("\nQuery:");
    query.print(0, "");

    let product = resolve(query);
    println!("\nProduct:");
    product.print(0, "");
}

fn resolve(query: FreeTerm) -> ProductTerm {
    let appl = match query {
        FreeTerm::Global(global) => return ProductTerm::Global(global),
        FreeTerm::Abstraction(abstr) => return ProductTerm::Abstraction(abstr),
        FreeTerm::Application(appl) => appl,
    };

    let Some(function_free) = FreeTerm::from_any(*appl.function) else {
        unreachable!(
            "application function (left-hand side) should have been beta-reduced to a free term"
        );
    };

    let function_product = resolve(function_free);
    let mut function_abstr = expand_global_recursive(function_product);

    beta_reduce(
        &function_abstr.parameter,
        &mut *function_abstr.body,
        &*appl.argument,
    );

    let Some(body_free) = FreeTerm::from_any(*function_abstr.body) else {
        unreachable!("application body should have been beta-reduced to a free term");
    };

    resolve(body_free)
}

fn beta_reduce(parameter: &Local, body: &mut AnyTerm, argument: &AnyTerm) {
    match body {
        AnyTerm::Global(_) => (),
        AnyTerm::Abstraction(abstr) => {
            beta_reduce(parameter, &mut abstr.body, argument);
        }
        AnyTerm::Application(appl) => {
            beta_reduce(parameter, &mut appl.function, argument);
            beta_reduce(parameter, &mut appl.argument, argument);
        }
        AnyTerm::Local(local) => {
            if local.id == parameter.id {
                *body = argument.clone();
            }
        }
    }
}

fn expand_global_recursive(mut product: ProductTerm) -> Abstr {
    for _ in 0..100 {
        match product {
            ProductTerm::Global(global) => {
                let Some(decl_product) = ProductTerm::from_free(global.value.term) else {
                    unimplemented!("application as declaration: `T = A x`");
                };
                product = decl_product;
            }
            ProductTerm::Abstraction(abstr) => return abstr,
        }
    }
    // TODO: Handle this without a loop limit
    panic!("expansion limit reached. declaration reference cycle?");
}

type LocalRef = Local;
type AnyTermRef = Box<AnyTerm>;

#[derive(Clone)]
struct GlobalRef {
    value: Box<Decl>,
}

#[derive(Clone)]
struct Decl {
    name: String,
    term: FreeTerm,
}

enum ProductTerm {
    Global(GlobalRef),
    Abstraction(Abstr),
}

#[derive(Clone)]
enum FreeTerm {
    Global(GlobalRef),
    Abstraction(Abstr),
    Application(Appl),
}

#[derive(Clone)]
enum AnyTerm {
    Global(GlobalRef),
    Abstraction(Abstr),
    Application(Appl),
    Local(LocalRef),
}

#[derive(Clone)]
struct Abstr {
    parameter: Local,
    body: AnyTermRef,
}

#[derive(Clone)]
struct Appl {
    function: AnyTermRef,
    argument: AnyTermRef,
}

#[derive(Clone)]
struct Local {
    id: usize,
    name: String,
}

impl GlobalRef {
    pub fn new(decl: Decl) -> Self {
        Self {
            value: Box::new(decl),
        }
    }
}

impl Decl {
    pub fn new(name: impl Into<String>, term: impl Into<FreeTerm>) -> Self {
        Self {
            name: name.into(),
            term: term.into(),
        }
    }
}

impl Abstr {
    pub fn new(parameter: Local, body: impl Into<AnyTerm>) -> Self {
        Self {
            parameter,
            body: Box::new(body.into()),
        }
    }
}

impl Appl {
    pub fn new(function: impl Into<AnyTerm>, argument: impl Into<AnyTerm>) -> Self {
        Self {
            function: Box::new(function.into()),
            argument: Box::new(argument.into()),
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

impl From<GlobalRef> for ProductTerm {
    fn from(value: GlobalRef) -> Self {
        ProductTerm::Global(value)
    }
}

impl From<Abstr> for ProductTerm {
    fn from(value: Abstr) -> Self {
        ProductTerm::Abstraction(value)
    }
}

impl From<Appl> for FreeTerm {
    fn from(value: Appl) -> Self {
        FreeTerm::Application(value)
    }
}

impl From<Local> for AnyTerm {
    fn from(value: Local) -> Self {
        Self::Local(value)
    }
}

impl<T> From<T> for FreeTerm
where
    T: Into<ProductTerm>,
{
    fn from(value: T) -> Self {
        match value.into() {
            ProductTerm::Global(global) => Self::Global(global),
            ProductTerm::Abstraction(abstr) => Self::Abstraction(abstr),
        }
    }
}

impl<T> From<T> for AnyTerm
where
    T: Into<FreeTerm>,
{
    fn from(value: T) -> Self {
        match value.into() {
            FreeTerm::Global(global) => Self::Global(global),
            FreeTerm::Abstraction(abstr) => Self::Abstraction(abstr),
            FreeTerm::Application(appl) => Self::Application(appl),
        }
    }
}

impl ProductTerm {
    pub fn from_any(term: AnyTerm) -> Option<Self> {
        Some(match term {
            AnyTerm::Global(global) => Self::Global(global),
            AnyTerm::Abstraction(abstr) => Self::Abstraction(abstr),
            AnyTerm::Application(_) | AnyTerm::Local(_) => return None,
        })
    }

    pub fn from_free(term: FreeTerm) -> Option<Self> {
        Some(match term {
            FreeTerm::Global(global) => Self::Global(global),
            FreeTerm::Abstraction(abstr) => Self::Abstraction(abstr),
            FreeTerm::Application(_) => return None,
        })
    }
}

impl FreeTerm {
    pub fn from_any(term: AnyTerm) -> Option<Self> {
        Some(match term {
            AnyTerm::Global(global) => Self::Global(global),
            AnyTerm::Abstraction(abstr) => Self::Abstraction(abstr),
            AnyTerm::Application(appl) => Self::Application(appl),
            AnyTerm::Local(_) => return None,
        })
    }
}

trait Print {
    fn print(&self, depth: usize, prefix: &str);
}

impl Print for Decl {
    fn print(&self, _depth: usize, _prefix: &str) {
        println!("[_] {}", self.name);
        self.term.print(0, "");
    }
}

impl Print for ProductTerm {
    fn print(&self, depth: usize, prefix: &str) {
        match self {
            Self::Global(global) => global.print(depth, prefix),
            Self::Abstraction(abstr) => abstr.print(depth, prefix),
        }
    }
}

impl Print for FreeTerm {
    fn print(&self, depth: usize, prefix: &str) {
        match self {
            Self::Global(global) => global.print(depth, prefix),
            Self::Abstraction(abstr) => abstr.print(depth, prefix),
            Self::Application(appl) => appl.print(depth, prefix),
        }
    }
}

impl Print for AnyTerm {
    fn print(&self, depth: usize, prefix: &str) {
        match self {
            Self::Global(global) => global.print(depth, prefix),
            Self::Abstraction(abstr) => abstr.print(depth, prefix),
            Self::Application(appl) => appl.print(depth, prefix),
            Self::Local(local) => {
                print_label(depth, prefix, "local");
                print_local(local);
            }
        }
    }
}

impl Print for GlobalRef {
    fn print(&self, depth: usize, prefix: &str) {
        print_label(depth, prefix, "global");
        println!("[_] `{}` [...]", self.value.name);
    }
}

impl Print for Abstr {
    fn print(&self, depth: usize, prefix: &str) {
        print_label(depth, prefix, "abstraction");
        println!();
        print_label(depth + 1, "parameter", "");
        print_local(&self.parameter);
        self.body.print(depth + 1, "body");
    }
}

impl Print for Appl {
    fn print(&self, depth: usize, prefix: &str) {
        print_label(depth, prefix, "application");
        println!();
        self.function.print(depth + 1, "function");
        self.argument.print(depth + 1, "argument");
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
