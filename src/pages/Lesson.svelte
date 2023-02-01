<script lang="ts">
  import {formatForLessonView} from "~/utils/date-format";
  import type {Lesson} from "~/utils/interfaces";

  export let lesson: Lesson;
  console.log(lesson);
</script>

<h1>{formatForLessonView(lesson.date)}</h1>
<h2>{lesson.event.user}</h2>
