mod database;
pub use database::Database;

use arena::Idx;
use smol_str::SmolStr;

type ExprIdx = Idx<Expr>;

pub fn lower(ast: ast::Root) -> (Database, Vec<Stmt>) {
  let mut db = Database::default();
  let stmts = ast.stmts().filter_map(|stmt| db.lower_stmt(stmt)).collect();

  (db, stmts)
}

#[derive(Debug)]
pub enum Stmt {
  VariableDef { name: SmolStr, value: Expr },
  Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
  Binary {
    op: BinaryOp,
    lhs: ExprIdx,
    rhs: ExprIdx,
  },
  Literal {
    n: u64,
  },
  Unary {
    op: UnaryOp,
    expr: ExprIdx,
  },
  VariableRef {
    var: SmolStr,
  },
  Missing,
}

#[derive(Debug)]
pub enum BinaryOp {
  Add,
  Sub,
  Mul,
  Div,
}

#[derive(Debug)]
pub enum UnaryOp {
  Neg,
}
