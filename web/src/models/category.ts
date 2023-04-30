export interface Category {
  id: number;
  slug: string;
  name: string;
  description: string;
}

export type CompactCategory = Pick<Category, "id" | "slug" | "name">;
