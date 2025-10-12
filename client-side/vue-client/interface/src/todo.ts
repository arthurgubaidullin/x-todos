export interface Todo {
  readonly id: string;
  readonly text: string;
  readonly version: string;
  readonly equals: (other: Todo) => boolean;
  readonly remove: () => Promise<void>;
  readonly update: (data: Readonly<{ text: string }>) => Promise<void>;
}
