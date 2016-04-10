use geometry::Point;

type EventHandler = fn (event:Event);

enum EventType
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

enum Event
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

struct Key
{
  code: u32,
  letter: Some(char)
}
