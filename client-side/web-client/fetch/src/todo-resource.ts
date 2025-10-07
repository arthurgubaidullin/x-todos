type Links = Readonly<{
  self: string;
  remove: string;
}>;

export type TodoResource = Readonly<{
  id: string;
  text: string;
  links: Links;
}>;
