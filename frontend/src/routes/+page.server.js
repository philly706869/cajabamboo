import { API_URL } from "$lib/env";

export async function load({ fetch }) {
  const res = await fetch(`${API_URL}/categories`);
  const categories = await res.json();

  return {
    categories,
  };
}
