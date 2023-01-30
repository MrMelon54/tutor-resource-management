<script lang="ts">
  const categories = [
    {key: "maths", name: "Maths"},
    {key: "english", name: "English"},
    {key: "a", name: "A"},
    {key: "b", name: "B"},
    {key: "c", name: "C"},
    {key: "d", name: "D"},
    {key: "e", name: "E"},
    {key: "f", name: "F"},
    {key: "g", name: "G"},
    {key: "h", name: "H"},
    {key: "i", name: "I"},
    {key: "j", name: "J"},
    {key: "k", name: "K"},
    {key: "l", name: "L"},
    {key: "m", name: "M"},
    {key: "n", name: "N"},
    {key: "o", name: "O"},
    {key: "p", name: "P"},
  ];
</script>

<div class="category-cards">
  {#each categories as category (category.key)}
    <div class="card">{category.name}</div>
  {/each}
</div>

<style lang="scss">
  .category-cards {
    padding: 8px;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
    gap: 8px;

    .card {
      cursor: pointer;
      box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2);
      transition: 0.3s;
      border-radius: 8px;
      padding: 2px 16px;
      box-sizing: border-box;
      background-color: var(--primary);
      color: var(--primary-text);

      &:hover {
        box-shadow: 0 8px 16px 0 rgba(0, 0, 0, 0.2);
      }
    }
  }
</style>
