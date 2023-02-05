<script lang="ts">
  import {dynamicEvents, students} from "~/stores/main";
  import {formatForLessonView} from "~/utils/date-format";
</script>

<h1>Students</h1>
{#each $students as student (student.id)}
  <h2>{student.id} - {student.name}</h2>
  {@const events = $dynamicEvents.filter(x => x.user == student.id)}
  {#each events as event (event.id)}
    {#if event.repeatWeeks == 0}
      <div>{formatForLessonView(event.startDate)} forever for {event.duration / 3600} hours</div>
    {:else if event.repeatWeeks == 1}
      <div>{formatForLessonView(event.startDate)} as a one off for {event.duration / 3600} hours</div>
    {:else}
      <div>{formatForLessonView(event.startDate)} for {event.repeatWeeks} weeks for {event.duration / 3600} hours</div>
    {/if}
  {/each}
{/each}
