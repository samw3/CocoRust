use std::path::PathBuf;
use std::rc::Rc;
use bit_set::BitSet;
use std::iter::FromIterator;
use crate::dfa::CharSet;

struct Sets();

impl Sets {
    pub fn elements(b: &BitSet) -> i32 {
        b.len() as i32
    }

    pub fn equals(a: &BitSet, b: &BitSet) -> bool {
        a.eq(b)
    }

    pub fn intersects(a: &BitSet, b: &BitSet) -> bool {
        BitSet::<u32>::from_iter(a.intersection(b)).len() != 0
    }

    pub fn subtract(a: &mut BitSet, b: &BitSet) {
        a.difference_with(b);
    }
}

struct CharClass {
    n: i32,
    name: String,
    set: CharSet,
}

struct Position {
    beg: i32,
    end: i32,
    col: i32,
}

impl Position {
    pub fn new(beg: i32, end: i32, col: i32) -> Self {
        Position { beg, end, col }
    }
}

enum SymInfoKind {
    Ident = 0,
    String = 1,
}

struct SymInfo {
    name: String,
    kind: SymInfoKind,
}

enum TokenKind {
    Fixed = 0,
    Class = 1,
    Lit = 2,
    ClassLit = 3,
}

struct Symbol {
    n: i32,
    typ: i32,
    name: String,
    graph: Node,
    token_kind: TokenKind,
    deletable: bool,
    first_ready: bool,
    first: BitSet,
    follow: BitSet,
    nts: BitSet,
    line: i32,
    attr_pos: Position,
    sem_pos: Position,
    ret_type: String,
    ret_var: String,
}

impl Symbol {
    pub fn new(typ: i32, name: String, line: i32) -> Self {
        Symbol {
            n: 0,
            typ,
            name,
            graph: None,
            token_kind: TokenKind::Fixed,
            deletable: false,
            first_ready: false,
            first: BitSet::new(),
            follow: BitSet::new(),
            nts: BitSet::new(),
            line,
            attr_pos: Position::new(0, 0, 0),
            sem_pos: Position::new(0, 0, 0),
            ret_type: String::new(),
            ret_var: String::new(),
        }
    }
}

struct State {}

impl State {
    pub fn new() -> Self {
        State {}
    }
}

enum NodeType {
    None = 0,
    T = 1,
    // terminal symbol
    Pr = 2,
    // pragma
    Nt = 3,
    // nonterminal symbol
    Clas = 4,
    // character class
    Chr = 5,
    // character
    Wt = 6,
    // weak terminal symbol
    Any = 7,
    //
    Eps = 8,
    // empty
    Sync = 9,
    // synchronization symbol
    Sem = 10,
    // semantic action: (. .)
    Alt = 11,
    // alternative: |
    Iter = 12,
    // iteration: { }
    Opt = 13,
    // option: [ ]
    Rslv = 14,  // resolver expr  /* ML */ /* AW 03-01-13 renamed slv --> rslv */
}

enum TransitionCode {
    Normal = 0,
    Context = 1,
}

pub struct NodeRef {
    n: Rc<NodeObject>,
}

type Node = Option<NodeRef>;

impl Clone for NodeRef {
    fn clone(&self) -> NodeRef {
        NodeRef {
            n: Rc::clone(&self.n)
        }
    }
}

struct NodeObject {
    n: i32,
    // node number
    typ: NodeType,
    // t, nt, wt, chr, clas, any, eps, sem, sync, alt, iter, opt, rslv
    next: Node,
    // to successor node
    down: Node,
    // alt: to next alternative
    sub: Node,
    // alt, iter, opt: to first node of substructure
    up: bool,
    // true: "next" leads to successor in enclosing structure
    sym: Symbol,
    // nt, t, wt: symbol represented by this node
    val: i32,
    // chr:  ordinal character value
    code: TransitionCode,
    // chr, clas: transition code
    set: BitSet,
    // any, sync: the set represented by this node
    pos: Position,
    // nt, t, wt: pos of actual attributes
    line: i32,
    // source text line number of item in this node
    state: State,
    // DFA state corresponding to this node
    ret_var: String,            // AH 20040206 - nt: name of output attribute (or null)
}

impl NodeObject {
    pub fn new(typ: NodeType, sym: Symbol, line: i32) -> Self {
        NodeObject {
            n: 0,
            typ,
            next: None,
            down: None,
            sub: None,
            up: false,
            sym,
            val: 0,
            code: TransitionCode::Normal,
            set: BitSet::new(),
            pos: Position::new(0, 0, 0),
            line,
            state: State::new(),
            ret_var: String::new(),
        }
    }
}

pub struct Graph {
    pub l: Node,
    pub r: Node,
}

impl Graph {
    pub fn new() -> Self {
        Graph { l: None, r: None }
    }

    pub fn new2(left: Node, right: Node) -> Self {
        Graph { l: left, r: right }
    }

    pub fn new1(p: Node) -> Self {
        Graph { l: p.clone(), r: p }
    }
}

pub struct Tab {
    src_name: PathBuf,
    src_dir: PathBuf,
    ns_name: Option<String>,
    frame_dir: Option<PathBuf>,
    out_dir: PathBuf,
    ddt_string: Option<String>,
}

impl Tab {
    pub fn new(
        module_name: Option<&str>,
        frame_path: Option<PathBuf>,
        ddt_string: Option<&str>,
        out_path: Option<PathBuf>,
        src_path: &PathBuf,
        src_dir: PathBuf,
    ) -> Tab {
        Tab {
            src_name: PathBuf::from(&src_path),
            src_dir: PathBuf::from(src_dir),
            ns_name: module_name.map(String::from),
            frame_dir: frame_path,
            out_dir: out_path.unwrap_or(PathBuf::from(&src_path)),
            ddt_string: ddt_string.map(String::from),
        }
    }
}
