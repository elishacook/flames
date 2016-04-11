use std::collections::HashMap;
use std::ops::Deref;
use events::{EventType,EventHandler};

pub trait View<T>
{
  fn render (&self, T) ->  Node;
}

pub enum Node
{
  Text(String),
  Element(ElementData)
}

pub struct ElementData
{
  tags: TagList,
  children: NodeList,
  handlers: EventHandlerMap
}

type TagList = Vec<String>;
type NodeList = Vec<Node>;
type EventHandlerMap = HashMap<EventType,EventHandler>;

pub struct ElementBuilder {
  data: ElementData
}

impl ElementBuilder
{
  pub fn new (tags: &[&str]) -> Self
  {
    ElementBuilder {
      data: ElementData {
        tags: tags.iter().map(|x: &&str| x.to_string()).collect::<TagList>(),
        children: NodeList::new(),
        handlers: EventHandlerMap::new(),
      }
    }
  }
  
  pub fn tag (mut self, name: &str) -> Self
  {
    self.data.tags.push(name.to_string());
    self
  }
  
  pub fn add_child(mut self, child: Node) -> Self
  {
    self.data.children.push(child);
    self
  }
  
  pub fn node (self) -> Node
  {
    Node::Element(self.data)
  }
}

pub fn text (data: &str) -> Node
{
  Node::Text(data.to_string())
}

pub fn element (tags: Vec<&str>) -> ElementBuilder
{
  ElementBuilder::new(&tags)
}


/////////////////////////////
/////////PLAYGROUND//////////
/////////////////////////////

struct ExampleView;

impl<'a> View<&'a str> for ExampleView
{
  fn render(&self, item: &'a str) -> Node
  {
    element(vec!["list"]).add_child(text(item)).node()
  }
}

fn example ()
{
  let foo = "foo";
  let view = ExampleView;
  let node = view.render(foo);
}