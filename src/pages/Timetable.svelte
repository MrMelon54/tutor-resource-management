<script lang="ts">
  let startWithSunday: boolean = false;
  let d: Date = new Date();
  let month: number = d.getMonth();
  let year: number = d.getFullYear();
  $: cal = createCalendar(new Date(year, month, 1), startWithSunday);

  const monthNames = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

  interface CalData {
    startOfView: Date;
    cells: CalCell[];
  }

  interface CalCell {
    id: number;
    date: Date;
    sameMonth: boolean;
    events: Event[];
  }

  function createCalendar(d: Date, startWithSunday: boolean): CalData {
    let m = d.getMonth();
    let offsetDay = d.getDay();
    if (!startWithSunday) {
      if (offsetDay == 0) offsetDay = 6;
      else offsetDay--;
    }

    let startOfView = new Date(d.getFullYear(), d.getMonth(), d.getDate() - offsetDay);
    let cells = Array.from({length: 6 * 7}, (_, id): CalCell => {
      let nd = new Date(startOfView.getFullYear(), startOfView.getMonth(), startOfView.getDate() + id);
      return {
        id,
        date: nd,
        sameMonth: nd.getMonth() == m,
        events: [],
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
  {#each [0, 1, 2, 3, 4, 5] as row (row)}
    <tr>
      {#each [0, 1, 2, 3, 4, 5, 6] as col (col)}
        {@const cell = cal.cells[row * 7 + col]}
        <td class={cell.sameMonth ? "" : "extra-day"} data-DoM={cell.date.getDate()}> Some day data</td>
      {/each}
    </tr>
  {/each}
</table>

<button style="position:absolute;top:50px;" on:click={() => (startWithSunday = !startWithSunday)}>Toggle startWithSunday: {startWithSunday}</button>

<style lang="scss">
  table {
    table-layout: fixed;
    border-collapse: collapse;
    width: 100%;
    height: 100%;

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
