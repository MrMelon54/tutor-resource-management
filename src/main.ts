// @ts-nocheck
import {get} from "svelte/store";
import "~/app.scss";
import App from "~/App.svelte";
import {theme} from "~/stores/theme";

const app = new App({
  target: document.getElementById("app"),
});

export default app;

updateTheme(get(theme));
theme.subscribe(value => updateTheme(value));

function updateTheme(value: "light" | "dark") {
  if (value == "light") {
    document.body.classList.remove("dark-mode");
  } else {
    document.body.classList.add("dark-mode");
  }
}
