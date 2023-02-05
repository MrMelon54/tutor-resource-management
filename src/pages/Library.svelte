<script lang="ts">
  import {listen} from "@tauri-apps/api/event";
  import {invoke} from "@tauri-apps/api/tauri";
  import {BaseDirectory, createDir} from "@tauri-apps/api/fs";
  import {libraryCategories} from "~/stores/main";

  listen("tauri://file-drop", event => {
    (event.payload as string[]).forEach(element => {
      importFileToLibrary(element);
    });
  });

  async function importFileToLibrary(name: string) {
    console.log("Importing file:", name);
    console.log(await invoke("import_file_to_library", {name}));
  }
</script>

<div class="category-cards">
  {#each $libraryCategories as category (category.id)}
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
