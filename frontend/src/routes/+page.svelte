<script>
  const { data } = $props();
  const { categories } = data;
</script>

{#each categories as category}
  <button>{category.name}</button>
{/each}
