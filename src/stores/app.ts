import {getName, getVersion, getTauriVersion} from "@tauri-apps/api/app";
import {appConfigDir, appDataDir} from "@tauri-apps/api/path";
import {writable} from "svelte/store";

const copyright: string = "© 2023 MrMelon54";
const appId: string = "com.mrmelon54.tutor-resource-management";

export interface AppDetail {
  name: string;
  version: string;
  tauriVersion: string;
  copyright: string;
  appId: string;
  configDir: string;
  dataDir: string;
}

export const appDetailStore = writable<AppDetail | null>(
  (() => {
    Promise.all([getName(), getVersion(), getTauriVersion(), appConfigDir(), appDataDir()]).then(
      ([name, version, tauriVersion, configDir, dataDir]) => {
        appDetailStore.set({name, version, tauriVersion, copyright, appId, configDir, dataDir});
      },
    );
    return null;
  })(),
);
