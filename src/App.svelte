<script lang="ts">
  import {listen} from "@tauri-apps/api/event";
  import {openAbout} from "~/utils/about-dialog";
  import {theme} from "~/stores/theme";
  import MoonIcon from "~/icons/Moon.svelte";
  import SunIcon from "~/icons/Sun.svelte";
  import UsersIcon from "~/icons/Users.svelte";
  import CalendarIcon from "~/icons/Calendar.svelte";
  import LibraryIcon from "~/icons/Library.svelte";
  import LibraryPage from "~/pages/Library.svelte";
  import TimetablePage from "./pages/Timetable.svelte";

  listen("tauri://file-drop", event => {
    console.log(event);
  });

  function toggleTheme() {
    theme.update(value => (value == "dark" ? "auto" : "dark"));
  }

  let selectedTab = "cal";

  const tabItems = [
    {key: "cal", name: "Timetable", icon: CalendarIcon, content: TimetablePage},
    {key: "stu", name: "Students", icon: UsersIcon, content: UsersIcon},
    {key: "lib", name: "Library", icon: LibraryIcon, content: LibraryPage},
  ];
  const tabMap = {};
  tabItems.forEach(x => {
    tabMap[x.key] = x;
  });
</script>

<main class="container">
  <header>
    <h1 class="large">Tutor Resource Management</h1>
    <h1 class="small">TRM</h1>
    <span class="flex-gap" />
    <button class="about-button solid" on:click={() => openAbout()}>About</button>
    <button class="theme-button" on:click={() => toggleTheme()}>
      {#if $theme == "dark"}
        <MoonIcon />
      {:else}
        <SunIcon />
      {/if}
    </button>
  </header>
  <nav class="main-tabs">
    {#each tabItems as tab (tab.key)}
      <button class="tab {selectedTab == tab.key ? 'active' : ''}" on:click={() => (selectedTab = tab.key)}>
        <svelte:component this={tab.icon} />
      </button>
    {/each}
  </nav>
  <main class="content">
    <svelte:component this={tabMap[selectedTab].content} />
  </main>
</main>

<style lang="scss">
  main.container {
    display: grid;
    grid-template-areas: "header header" "tabs content";
    grid-template-columns: 48px auto;
    grid-template-rows: 48px auto;
    min-height: 100vh;

    > header {
      display: flex;
      flex-direction: row;
      grid-area: header;
      background-color: var(--primary);
      color: var(--primary-text);

      > h1 {
        margin: 0;
        line-height: normal;
        vertical-align: middle;
        height: auto;
        align-self: center;
        margin-left: 16px;

        &.small {
          display: none;
        }

        @media screen and (max-width: 700px) {
          &.large {
            display: none;
          }

          &.small {
            display: inherit;
          }
        }
      }

      > .about-button {
        margin: 4px;
      }

      > .theme-button {
        height: 100%;
        aspect-ratio: 1/1;
        box-sizing: border-box;
        padding: 0;
        margin: 0;
      }
    }

    > nav.main-tabs {
      grid-area: tabs;

      > .tab {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 100%;
        aspect-ratio: 1/1;
        position: relative;
        margin: 0;
        padding: 0;

        &.active::before {
          position: absolute;
          content: "";
          top: 2px;
          left: 2px;
          bottom: 2px;
          border-radius: 2px;
          border-left: 4px solid var(--primary);
        }
      }
    }

    > main.content {
      grid-area: content;
    }
  }
</style>
