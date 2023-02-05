<script lang="ts">
  import {students} from "~/stores/main";
  import {formatForLessonView} from "~/utils/date-format";
  import type {Lesson} from "~/utils/interfaces";

  export let lesson: Lesson;
  console.log(lesson);
</script>

<h1>{formatForLessonView(lesson.date)}</h1>
<h2>{$students[lesson.event.user].name}</h2>
