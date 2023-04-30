export interface Tag {
  id: number;
  slug: string;
  name: string;
}

export type CompactTag = Pick<Tag, "id" | "slug" | "name">;
