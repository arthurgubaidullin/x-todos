export interface Todo {
  readonly id: string;
  readonly text: string;
  readonly remove: () => Promise<void>;
}
