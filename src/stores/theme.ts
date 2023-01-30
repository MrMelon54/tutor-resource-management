import { writable } from "svelte/store";

export const theme = writable<"auto" | "light" | "dark">(
  (() => {
    try {
      return localStorage.getItem("theme") as "auto" | "light" | "dark";
    } catch (_) {
      return "auto";
    }
  })()
);

theme.subscribe((value) => localStorage.setItem("theme", value));
