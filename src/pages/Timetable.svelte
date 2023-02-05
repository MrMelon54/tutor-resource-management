<script lang="ts">
  import {createEventDispatcher} from "svelte";
  import {addDate, dateTimeMerge, getDate, monthNames, timeSec, timeStr} from "~/utils/date-format";
  import type {CalCell, CalData, DynamicEvent, Student} from "~/utils/interfaces";
  import {dynamicEvents, students} from "~/stores/main";

  let startWithSunday: boolean = false;
  let onlyOneWeek: boolean = false;
  let d: Date = new Date();
  let month: number = d.getMonth();
  let year: number = d.getFullYear();
  $: cal = createCalendar(new Date(year, month, onlyOneWeek ? d.getDate() : 1), startWithSunday, onlyOneWeek, $dynamicEvents, $students);

  let dispatch = createEventDispatcher();

  function createCalendar(d: Date, startWithSunday: boolean, onlyOneWeek: boolean, dynamicEvents: DynamicEvent[], _: Student[]): CalData {
    let m = d.getMonth();
    let offsetDay = d.getDay();
    if (!startWithSunday) {
      if (offsetDay == 0) offsetDay = 6;
      else offsetDay--;
    }

    let startOfView = new Date(d.getFullYear(), d.getMonth(), d.getDate() - offsetDay);
    let cells = Array.from({length: (onlyOneWeek ? 1 : 6) * 7}, (_, id): CalCell => {
      let date = new Date(startOfView.getFullYear(), startOfView.getMonth(), startOfView.getDate() + id);
      let events = dynamicEvents
        .filter(x => {
          if (date.getDay() != x.startDate.getDay()) return false;
          return getDate(x.startDate) <= date && (x.repeatWeeks == 0 || addDate(x.startDate, 7 * x.repeatWeeks) > date);
        })
        .sort((a, b) => timeSec(a.startDate) - timeSec(b.startDate));
      return {
        id,
        date,
        sameMonth: date.getMonth() == m,
        events,
      };
    });
    return {
      startOfView,
      cells,
    };
  }
</script>

<table>
  <tr class="month-row">
    <td colspan="7">
      <h1>
        <span>{monthNames[month]}</span>
        <span style="float:right;">{year}</span>
      </h1>
    </td>
  </tr>
  <tr class="day-row">
    {#if startWithSunday}
      <th>Sun<span>day</span></th>
    {/if}
    <th>Mon<span>day</span></th>
    <th>Tue<span>sday</span></th>
    <th>Wed<span>nesday</span></th>
    <th>Thu<span>rsday</span></th>
    <th>Fri<span>day</span></th>
    <th>Sat<span>urday</span></th>
    {#if !startWithSunday}
      <th>Sun<span>day</span></th>
    {/if}
  </tr>
  {#each onlyOneWeek ? [0] : [0, 1, 2, 3, 4, 5] as row (row)}
    <tr>
      {#each [0, 1, 2, 3, 4, 5, 6] as col (col)}
        {@const cell = cal.cells[row * 7 + col]}
        <td class={cell.sameMonth ? "" : "extra-day"} data-DoM={cell.date.getDate()}>
          <div class="wrapper">
            {#each cell.events as e (e.id)}
              <button class="pill" on:click={() => dispatch("openLesson", {event: e, date: dateTimeMerge(cell.date, e.startDate)})}>
                <span>{timeStr(e.startDate)} {$students[e.user] ? $students[e.user].name : "Unknown"}</span>
              </button>
            {/each}
          </div>
        </td>
      {/each}
    </tr>
  {/each}
</table>

<button style="position:absolute;top:50px;" on:click={() => (startWithSunday = !startWithSunday)}>Toggle startWithSunday: {startWithSunday}</button>
<button style="position:absolute;top:50px;left:250px;" on:click={() => (onlyOneWeek = !onlyOneWeek)}>Toggle onlyOneWeek: {onlyOneWeek}</button>

<style lang="scss">
  table {
    table-layout: fixed;
    border-collapse: collapse;
    width: 100%;

    tr {
      + tr {
        border-top: 1px solid grey;
      }

      &.day-row {
        height: 20px;

        @media screen and (max-width: 750px) {
          th span {
            display: none;
          }
        }
      }

      &.month-row {
        height: 45px;

        h1 {
          margin: 0;
          padding: 0 12px;
          line-height: normal;
        }
      }
    }

    td + td,
    th + th {
      border-left: 1px solid grey;
    }

    td {
      position: relative;
      padding-top: 20px;
      vertical-align: top;

      div.wrapper {
        display: flex;
        flex-direction: column;
        margin: 2px;
        gap: 2px;

        button.pill {
          font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
          font-size: 14px;
          font-weight: 500;
          text-align: left;
          line-height: normal;
          text-overflow: ellipsis;
          overflow: hidden;
          white-space: nowrap;
          color: var(--primary-text);
          background-color: var(--primary);
          border-radius: 4px;
          padding: 2px 4px;
          cursor: pointer;
        }
      }

      &::before {
        content: attr(data-DoM);
        position: absolute;
        top: 0;
        left: 0;
      }

      &.extra-day {
        position: relative;

        &::after {
          content: "";
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          background-color: rgba(0, 0, 0, 0.3);
        }
      }
    }
  }
</style>
