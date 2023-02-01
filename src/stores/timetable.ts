import {writable} from "svelte/store";
import type {DynamicEvent, Student} from "~/utils/interfaces";

export const dynamicEvents = writable<DynamicEvent[]>(
  (() => {
    try {
      return console.error("Code load dynamic events"), [];
    } catch (_) {
      return [];
    }
  })(),
);

dynamicEvents.subscribe(value => console.error("Code save dynamic events:", value));

export const students = writable<Student[]>(
  (() => {
    try {
      return console.error("Code load students"), [];
    } catch (_) {
      return [];
    }
  })(),
);

students.subscribe(value => console.error("Code save students:", value));
