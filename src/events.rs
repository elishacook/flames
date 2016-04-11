use geometry::Point;

pub type EventHandler = fn (event:Event);

#[derive(PartialEq,Eq,Hash)]
pub enum EventType
{
  KeyDown,
  KeyUp,
  Focus,
  Blur,
  PointerEnter,
  PointerExit,
  PointerDown,
  PointerUp,
  PointerMove,
}

pub enum Event
{
  KeyDown(Key),
  KeyUp(Key),
  Focus,
  Blur,
  PointerEnter(Point),
  PointerExit(Point),
  PointerDown(Point),
  PointerUp(Point),
  PointerMove(Point),
}

pub struct Key
{
  pub code: u32,
  pub letter: Option<char>
}
