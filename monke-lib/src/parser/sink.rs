use super::event::Event;
use crate::syntax::{MonkeLanguage, SyntaxKind};
use lexer::Token;
use rowan::{GreenNode, GreenNodeBuilder, Language};
use std::mem;

pub(super) struct Sink<'t, 'input> {
  builder: GreenNodeBuilder<'static>,
  tokens: &'t [Token<'input>],
  cursor: usize,
  events: Vec<Event>,
}

impl<'t, 'input> Sink<'t, 'input> {
  pub(super) fn new(tokens: &'t [Token<'input>], events: Vec<Event>) -> Self {
    Self {
      builder: GreenNodeBuilder::new(),
      tokens,
      cursor: 0,
      events,
    }
  }

  pub(super) fn finish(mut self) -> GreenNode {
    for idx in 0..self.events.len() {
      match mem::replace(&mut self.events[idx], Event::Placeholder) {
        Event::StartNode {
          kind,
          forward_parent,
        } => {
          let mut kinds = vec![kind];

          let mut idx = idx;
          let mut forward_parent = forward_parent;

          while let Some(fp) = forward_parent {
            idx += fp;

            forward_parent = if let Event::StartNode {
              kind,
              forward_parent,
            } = mem::replace(&mut self.events[idx], Event::Placeholder)
            {
              kinds.push(kind);
              forward_parent
            } else {
              unreachable!()
            };
          }

          for kind in kinds.into_iter().rev() {
            self.builder.start_node(MonkeLanguage::kind_to_raw(kind));
          }
        }
        Event::AddToken => self.token(),
        Event::FinishNode => self.builder.finish_node(),
        Event::Placeholder => {}
      }

      self.eat_trivia();
    }

    self.builder.finish()
  }

  fn token(&mut self) {
    let Token { kind, text } = self.tokens[self.cursor];

    self
      .builder
      .token(MonkeLanguage::kind_to_raw(kind.into()), text.into());

    self.cursor += 1;
  }

  fn eat_trivia(&mut self) {
    while let Some(token) = self.tokens.get(self.cursor) {
      if !SyntaxKind::from(token.kind).is_trivia() {
        break;
      }

      self.token();
    }
  }
}
