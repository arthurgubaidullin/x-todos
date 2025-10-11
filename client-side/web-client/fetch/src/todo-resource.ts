type Links = Readonly<{
  self: string;
  remove: string;
  update: string;
}>;

export type TodoResource = Readonly<{
  id: string;
  text: string;
  version: string;
  links: Links;
}>;
