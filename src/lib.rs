pub mod window;


type View = fn <T> (T state) -> ViewNode;

struct ViewNode
{
  children: Vec<ViewNode>,
  tags: Vec<String>,
  handlers: HashMap<Enum,EventHandler>,
}

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

struct Point
{
  x: f32,
  y: f32
}