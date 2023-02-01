import {writable} from "svelte/store";

export const theme = writable<"light" | "dark">(
  (() => {
    try {
      return localStorage.getItem("theme") as "light" | "dark";
    } catch (_) {
      return "dark";
    }
  })(),
);

theme.subscribe(value => localStorage.setItem("theme", value));
