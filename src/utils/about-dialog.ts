import { message } from "@tauri-apps/api/dialog";
import { appDetailStore } from "~/stores/app";
import { get } from "svelte/store";

export function openAbout() {
  let app = get(appDetailStore);
  message(
    `
Version: ${app.version}
Tauri: ${app.tauriVersion}
ID: ${app.appId}
Copyright: ${app.copyright}

`,
    { title: app.name, type: "info" }
  );
}
