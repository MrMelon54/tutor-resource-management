import {writeTextFile, renameFile, BaseDirectory, readTextFile, createDir} from "@tauri-apps/api/fs";
import {writable, type Writable} from "svelte/store";
import type {DynamicEvent, Student} from "~/utils/interfaces";
import {dateTimeReviver} from "~/utils/date-time-reviver";

function createAsyncWritable<T>(value: T, call: Promise<T>): Writable<T> {
  value.__NO_VALUE__ = true;
  let w = writable<T>(value);
  call.then(v => w.set(v));
  return w;
}

function createConfigWritable<T>(value: T, name: string): Writable<T> {
  let w = createAsyncWritable(
    value,
    (async () => {
      let f = await readTextFile(name, {dir: BaseDirectory.AppConfig});
      return JSON.parse(f, dateTimeReviver) as T;
    })(),
  );
  w.subscribe(value => {
    if (value.__NO_VALUE__) return;
    (async () => {
      await createDir("", {
        dir: BaseDirectory.AppConfig,
        recursive: true,
      });
      try {
        await renameFile(name, name + "-old", {dir: BaseDirectory.AppConfig});
      } catch (_) {}
      await writeTextFile(name, JSON.stringify(value), {dir: BaseDirectory.AppConfig});
    })();
  });
  return w;
}

export const dynamicEvents = createConfigWritable<DynamicEvent[]>([], "dynamic-events.json");

export const students = createConfigWritable<Student[]>([], "students.json");

window.aaaa = [dynamicEvents, students];
