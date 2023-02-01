export interface DynamicEvent {
  id: number;
  startDate: Date;
  duration: number;
  repeatWeeks: number;
  user: number;
}

export interface CalData {
  startOfView: Date;
  cells: CalCell[];
}

export interface CalCell {
  id: number;
  date: Date;
  sameMonth: boolean;
  events: DynamicEvent[];
}

export interface Lesson {
  event: DynamicEvent;
  date: Date;
}

export interface Student {
  id: number;
  name: string;
}
