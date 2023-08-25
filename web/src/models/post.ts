export interface Post {
  id: number;
  slug: string;
  title: string;
  poster_id: number;
  short_description: string;
  description: string;
  tag_ids: number[];
  is_published: boolean;
  published_at: string;
}

export type CompactPost = Pick<
  Post,
  "id" | "slug" | "title" | "short_description" | "poster_id" | "tag_ids"
>;
