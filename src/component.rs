use std::cmp::Eq;
use events::{EventType, EventHandler};

struct Component <T: Controller, M: Eq>
{
  controller: T<M>,
  view: View<T>,
}

trait Controller <T: Eq>
{
  fn new (model: T) -> Self;
  fn set_state (model: T);
  fn get_state () -> T;
}

type View<T: Controller> = fn <T> (controller: T) -> ViewNode;

struct ViewNode
{
  children: Vec<ViewNode>,
  type: ViewNodeType
}

enum ViewNodeType
{
  Text(String),
  Element(ViewElementData),
  Component(Component,Option<ViewNode>)
}

struct ViewElementData
{
  tags: Vec<String>,
  handlers: HashMap<EventType, EventHandler>,
}

pub fn component<T, M> (view: View<T>, model: M) where T: Controller <M> -> ViewNode;
{
  ViewNode {
    children: Vec::new(),
    type: ViewNodeType::Component(
      Component {
        controller: T::new(model),
        view: view,
      },
      Option::None
    )
  }
}